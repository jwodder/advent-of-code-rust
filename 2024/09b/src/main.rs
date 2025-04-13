use adventutil::Input;
use std::collections::VecDeque;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct File {
    position: u64,
    id: u64,
    size: u64,
}

impl File {
    fn with_position(mut self, pos: u64) -> File {
        self.position = pos;
        self
    }

    fn checksum(&self) -> u64 {
        (self.position..)
            .map(|p| p * self.id)
            .take(usize::try_from(self.size).unwrap())
            .sum()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Free {
    position: u64,
    size: u64,
}

impl Free {
    fn reduce(&mut self, size: u64) {
        self.position += size;
        self.size -= size;
    }
}

fn solve(input: Input) -> u64 {
    let mut files = VecDeque::new();
    let mut free_space = Vec::new();
    let s = input.read();
    let mut is_file = true;
    let mut file_id = 0u64;
    let mut position = 0u64;
    for d in s.trim().chars() {
        let size = u64::from(d.to_digit(10).unwrap());
        if is_file {
            files.push_back(File {
                position,
                id: file_id,
                size,
            });
        } else {
            free_space.push(Free { position, size });
            file_id += 1;
        }
        is_file = !is_file;
        position += size;
    }
    let mut checksum = 0;
    if let Some(f) = files.pop_front() {
        checksum += f.checksum();
    }
    while let Some(f) = files.pop_back() {
        let mut moved = false;
        let mut del_index = None;
        for (j, free) in free_space.iter_mut().enumerate() {
            if free.position >= f.position {
                break;
            }
            if free.size >= f.size {
                checksum += f.with_position(free.position).checksum();
                free.reduce(f.size);
                if free.size == 0 {
                    del_index = Some(j);
                }
                moved = true;
                break;
            }
        }
        if !moved {
            checksum += f.checksum();
        }
        if let Some(j) = del_index {
            free_space.remove(j);
        }
    }
    checksum
}

fn main() {
    println!("{}", solve(Input::from_env()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = Input::from("2333133121414131402");
        assert_eq!(solve(input), 2858);
    }
}
