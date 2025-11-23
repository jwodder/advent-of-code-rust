use adventutil::pullparser::{ParseError, PullParser, Token};
use adventutil::{Input, unordered_pairs};
use itertools::Itertools;
use std::iter::once;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Combatant {
    hp: i32,
    damage: i32,
    armor: i32,
}

impl std::str::FromStr for Combatant {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Combatant, ParseError> {
        let mut parser = PullParser::new(s);
        parser.skip("Hit Points: ")?;
        let hp = parser.parse_to::<i32, _>(Token::Whitespace)?;
        parser.skip("Damage: ")?;
        let damage = parser.parse_to::<i32, _>(Token::Whitespace)?;
        parser.skip("Armor: ")?;
        let armor = parser.parse_to::<i32, _>(Token::Eof)?;
        Ok(Combatant { hp, damage, armor })
    }
}

// Returns `true` iff the player wins
fn fight(mut player: Combatant, mut boss: Combatant) -> bool {
    loop {
        boss.hp -= (player.damage - boss.armor).max(1);
        if boss.hp <= 0 {
            return true;
        }
        player.hp -= (boss.damage - player.armor).max(1);
        if player.hp <= 0 {
            return false;
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Item {
    cost: u32,
    damage: i32,
    armor: i32,
}

fn build<I: IntoIterator<Item = Item>>(iter: I) -> (Combatant, u32) {
    let mut fighter = Combatant {
        hp: 100,
        damage: 0,
        armor: 0,
    };
    let mut gold_spent = 0;
    for item in iter {
        gold_spent += item.cost;
        fighter.damage += item.damage;
        fighter.armor += item.armor;
    }
    (fighter, gold_spent)
}

fn solve(input: Input) -> u32 {
    let boss = input.parse::<Combatant>();
    let weapons = [
        Item {
            cost: 8,
            damage: 4,
            armor: 0,
        }, // Dagger
        Item {
            cost: 10,
            damage: 5,
            armor: 0,
        }, // Shortsword
        Item {
            cost: 25,
            damage: 6,
            armor: 0,
        }, // Warhammer
        Item {
            cost: 40,
            damage: 7,
            armor: 0,
        }, // Longsword
        Item {
            cost: 74,
            damage: 8,
            armor: 0,
        }, // Greataxe
    ];
    let armor = [
        Item {
            cost: 0,
            damage: 0,
            armor: 0,
        }, // None
        Item {
            cost: 13,
            damage: 0,
            armor: 1,
        }, // Leather
        Item {
            cost: 31,
            damage: 0,
            armor: 2,
        }, // Chainmail
        Item {
            cost: 53,
            damage: 0,
            armor: 3,
        }, // Splintmail
        Item {
            cost: 75,
            damage: 0,
            armor: 4,
        }, // Bandedmail
        Item {
            cost: 102,
            damage: 0,
            armor: 5,
        }, // Platemail
    ];
    let rings = [
        Item {
            cost: 25,
            damage: 1,
            armor: 0,
        }, // Damage +1
        Item {
            cost: 50,
            damage: 2,
            armor: 0,
        }, // Damage +2
        Item {
            cost: 100,
            damage: 3,
            armor: 0,
        }, // Damage +3
        Item {
            cost: 20,
            damage: 0,
            armor: 1,
        }, // Defense +1
        Item {
            cost: 40,
            damage: 0,
            armor: 2,
        }, // Defense +2
        Item {
            cost: 80,
            damage: 0,
            armor: 3,
        }, // Defense +3
    ];
    let ring_options = once(Vec::new())
        .chain(rings.iter().map(|&r| vec![r]))
        .chain(unordered_pairs(&rings).map(|(&r1, &r2)| vec![r1, r2]))
        .collect::<Vec<_>>();
    weapons
        .into_iter()
        .cartesian_product(armor)
        .cartesian_product(ring_options)
        .filter_map(|((wp, arm), rs)| {
            let (player, gold) = build(once(wp).chain(once(arm)).chain(rs));
            (!fight(player, boss)).then_some(gold)
        })
        .max()
        .unwrap()
}

fn main() {
    println!("{}", solve(Input::from_env()));
}
