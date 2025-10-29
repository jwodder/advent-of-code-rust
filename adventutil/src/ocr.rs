use super::grid::Grid;
use anyhow::{Context, bail};
use serde::{Deserialize, Deserializer, de::Error};

#[derive(Debug, Deserialize, Eq, PartialEq)]
struct Font {
    height: usize,
    #[serde(rename = "glyph")]
    glyphs: Vec<Glyph>,
}

#[derive(Debug, Deserialize, Eq, PartialEq)]
struct Glyph {
    character: char,
    #[serde(deserialize_with = "deserialize_bitmap")]
    bitmap: Grid<bool>,
}

impl Font {
    fn lookup(&self, bitmap: &Grid<bool>) -> Option<char> {
        for glyph in &self.glyphs {
            if &glyph.bitmap == bitmap {
                return Some(glyph.character);
            }
        }
        None
    }
}

fn load_fonts() -> Result<Vec<Font>, anyhow::Error> {
    let normal = toml::from_str(include_str!("fonts/normal.toml"))
        .context("Failed to parse fonts/normal.toml")?;
    let large = toml::from_str(include_str!("fonts/large.toml"))
        .context("Failed to parse fonts/large.toml")?;
    Ok(vec![normal, large])
}

pub fn ocr(drawing: Grid<bool>) -> Result<String, anyhow::Error> {
    let drawing = trim_drawing(drawing)?;
    let height = drawing.height();
    let Some(font) = load_fonts()?.into_iter().find(|f| f.height == height) else {
        bail!("No font found with glyph height {height}");
    };
    let mut text = String::new();
    let mut start = Some(0);
    let width = drawing.width();
    for x in 0..=width {
        match (x == width || column_is_false(&drawing, x), start) {
            (true, Some(st)) => {
                let Some(ch) = font.lookup(&drawing.column_slice(st..x)) else {
                    bail!("Character #{} not found in font", text.len() + 1);
                };
                text.push(ch);
                start = None;
            }
            (false, None) => start = Some(x),
            _ => (),
        }
    }
    Ok(text)
}

fn column_is_false(drawing: &Grid<bool>, colno: usize) -> bool {
    (0..(drawing.height())).all(|y| !drawing[(y, colno)])
}

fn row_is_false(drawing: &Grid<bool>, rowno: usize) -> bool {
    (0..(drawing.width())).all(|x| !drawing[(rowno, x)])
}

/// Strips off leading & trailing all-`false` columns & rows.  Errors if all
/// cells are `false`.
fn trim_drawing(drawing: Grid<bool>) -> Result<Grid<bool>, anyhow::Error> {
    let Some(start_y) = (0..(drawing.height())).find(|&y| !row_is_false(&drawing, y)) else {
        bail!("Drawing is empty");
    };
    let end_y = (0..(drawing.height()))
        .rev()
        .find(|&y| !row_is_false(&drawing, y))
        .unwrap();
    let start_x = (0..(drawing.width()))
        .find(|&x| !column_is_false(&drawing, x))
        .unwrap();
    let end_x = (0..(drawing.width()))
        .rev()
        .find(|&x| !column_is_false(&drawing, x))
        .unwrap();
    Ok(drawing
        .row_slice(start_y..=end_y)
        .column_slice(start_x..=end_x))
}

fn deserialize_bitmap<'de, D>(deserializer: D) -> Result<Grid<bool>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    Grid::from_drawing(s.trim()).map_err(D::Error::custom)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn validate_fonts() {
        let fonts = load_fonts().unwrap();
        let mut heights = HashSet::new();
        for f in &fonts {
            assert!(
                heights.insert(f.height),
                "Multiple fonts with height {}",
                f.height
            );
        }
        for f in fonts {
            let mut bitmaps = HashSet::new();
            for glyph in f.glyphs {
                assert!(
                    glyph.bitmap.height() == f.height,
                    "Glyph for {:?} has height {} instead of font's {}",
                    glyph.character,
                    glyph.bitmap.height(),
                    f.height
                );
                assert!(
                    bitmaps.insert(glyph.bitmap.clone()),
                    "Multiple glyphs with bitmap:\n{}",
                    glyph.bitmap.draw()
                );
            }
        }
    }

    #[test]
    fn test_trim_drawing() {
        let drawing = Grid::from_drawing(concat!(
            ".........\n",
            ".........\n",
            "....#....\n",
            "....#....\n",
            "..#####..\n",
            "....#....\n",
            "....#....\n",
            ".........\n",
            ".........\n",
        ))
        .unwrap();
        let drawing = trim_drawing(drawing).unwrap();
        assert_eq!(
            drawing.draw().to_string(),
            "..#..\n..#..\n#####\n..#..\n..#.."
        );
    }

    #[test]
    fn test_ocr_normal_font() {
        let drawing = Grid::from_drawing(concat!(
            ".##..####.###..##..#..#.#...#\n",
            "#..#.#.....#..#..#.#..#.#...#\n",
            "#..#.###...#..#..#.#..#..#.#.\n",
            "####.#.....#..#..#.#..#...#..\n",
            "#..#.#.....#..#..#.#..#...#..\n",
            "#..#.####.###..##...##....#..\n",
        ))
        .unwrap();
        assert_eq!(ocr(drawing).unwrap(), "AEIOUY");
    }

    #[test]
    fn test_ocr_large_font_untrimmed() {
        let drawing = Grid::from_drawing(concat!(
            "...##....#####....####..\n",
            "..#..#...#....#..#....#.\n",
            ".#....#..#....#..#......\n",
            ".#....#..#....#..#......\n",
            ".#....#..#####...#......\n",
            ".######..#....#..#......\n",
            ".#....#..#....#..#......\n",
            ".#....#..#....#..#......\n",
            ".#....#..#....#..#....#.\n",
            ".#....#..#####....####..\n",
            "........................\n",
        ))
        .unwrap();
        assert_eq!(ocr(drawing).unwrap(), "ABC");
    }
}
