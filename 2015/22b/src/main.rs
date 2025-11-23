use adventutil::pullparser::{ParseError, PullParser, Token};
use adventutil::{Input, dijkstra_length};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct GameState(Battle);

impl GameState {
    fn new(boss: Boss) -> GameState {
        GameState(Battle {
            player_hp: 50,
            mana: 500,
            armor: 0,
            effects: [0, 0, 0],
            boss,
        })
    }

    /// Cast a spell, let the boss take eir turn, and return the new game state
    /// and the cost of the spell cast.  Returns `None` if the spell couldn't
    /// be cast, if the player is defeated, or if the player has already won.
    fn cast(&self, spell: Spell) -> Option<(GameState, u32)> {
        let mut battle = self.0;
        let cost = battle.cast(spell)?;
        Some((GameState(battle), cost))
    }

    fn victory(&self) -> bool {
        self.0.won()
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Battle {
    player_hp: i32,
    mana: u32,
    armor: i32,
    effects: [usize; 3],
    boss: Boss,
}

impl Battle {
    /// Cast a spell, let the boss take eir turn, and return the cost of the
    /// spell cast.  Returns `None` if the spell couldn't be cast, if the
    /// player is defeated, or if the player has already won.
    fn cast(&mut self, spell: Spell) -> Option<u32> {
        self.player_hp -= 1;
        if self.player_hp <= 0 {
            return None;
        }
        self.tick();
        if self.won() {
            return Some(0);
        }
        let cost = spell.cost();
        self.mana = self.mana.checked_sub(cost)?;
        spell.apply(self)?;
        self.tick();
        if self.won() {
            return Some(cost);
        }
        self.player_hp -= (self.boss.damage - self.armor).max(1);
        (self.player_hp > 0).then_some(cost)
    }

    fn tick(&mut self) {
        for effect in Effect::iter() {
            let e = effect as usize;
            if self.effects[e] > 0 {
                effect.tick(self);
                self.effects[e] -= 1;
                if self.effects[e] == 0 {
                    effect.end(self);
                }
            }
        }
    }

    // Returns `None` if the effect could not be started due to another
    // instance of the same effect already being in ... effect
    fn start_effect(&mut self, effect: Effect, duration: usize) -> Option<()> {
        let e = effect as usize;
        if self.effects[e] > 0 {
            return None;
        }
        self.effects[e] = duration;
        effect.start(self);
        Some(())
    }

    fn won(&self) -> bool {
        self.boss.hp <= 0
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum Spell {
    MagicMissile,
    Drain,
    Shield,
    Poison,
    Recharge,
}

impl Spell {
    fn iter() -> impl Iterator<Item = Spell> {
        use Spell::*;
        [MagicMissile, Drain, Shield, Poison, Recharge].into_iter()
    }

    fn cost(&self) -> u32 {
        use Spell::*;
        match self {
            MagicMissile => 53,
            Drain => 73,
            Shield => 113,
            Poison => 173,
            Recharge => 229,
        }
    }

    // Returns `None` if the spell could not be cast due to it creating an
    // effect already in ... effect
    fn apply(&self, battle: &mut Battle) -> Option<()> {
        use Spell::*;
        match self {
            MagicMissile => battle.boss.hp -= 4,
            Drain => {
                battle.boss.hp -= 2;
                battle.player_hp += 2;
            }
            Shield => battle.start_effect(Effect::Shield, 6)?,
            Poison => battle.start_effect(Effect::Poison, 6)?,
            Recharge => battle.start_effect(Effect::Recharge, 5)?,
        }
        Some(())
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum Effect {
    Shield,
    Poison,
    Recharge,
}

impl Effect {
    fn iter() -> impl Iterator<Item = Effect> {
        use Effect::*;
        [Shield, Poison, Recharge].into_iter()
    }

    fn start(&self, battle: &mut Battle) {
        if self == &Effect::Shield {
            battle.armor += 7;
        }
    }

    fn tick(&self, battle: &mut Battle) {
        match self {
            Effect::Poison => battle.boss.hp -= 3,
            Effect::Recharge => battle.mana += 101,
            _ => (),
        }
    }

    fn end(&self, battle: &mut Battle) {
        if self == &Effect::Shield {
            battle.armor -= 7;
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Boss {
    hp: i32,
    damage: i32,
}

impl std::str::FromStr for Boss {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Boss, ParseError> {
        let mut parser = PullParser::new(s);
        parser.skip("Hit Points: ")?;
        let hp = parser.parse_to::<i32, _>(Token::Whitespace)?;
        parser.skip("Damage: ")?;
        let damage = parser.parse_to::<i32, _>(Token::Eof)?;
        Ok(Boss { hp, damage })
    }
}

fn solve(input: Input) -> u32 {
    let start = GameState::new(input.parse::<Boss>());
    dijkstra_length(start, GameState::victory, |state| {
        Spell::iter()
            .filter_map(|sp| state.cast(sp))
            .collect::<Vec<_>>()
    })
    .unwrap()
}

fn main() {
    println!("{}", solve(Input::from_env()));
}
