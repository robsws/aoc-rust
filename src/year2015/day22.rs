use std::{cmp::max, fmt::Display};

use regex::Regex;

use crate::input_file::read_lines;

pub fn part1(input_file_path: &str) {
    let lines = read_lines(input_file_path);
    let boss = parse_lines(lines);
    let player = Wizard { health: 50, mana: 500, armour: 0 };
    let game = Game::new(player, boss);
    let best_game = find_least_mana_win(&game).unwrap();
    println!("Mana spent: {}", best_game.mana_spent);
    println!("Spells cast:");
    for spell in best_game.spells_cast {
        println!(" -> {}", spell);
    }
}

fn parse_lines(lines: Vec<String>) -> Boss {
    let number_regex = Regex::new(r"\d+").unwrap();
    if lines.len() != 2 {
        panic!("Wrong number of lines in input file.");
    }
    Boss {
        health: number_regex.find(&lines[0])
            .unwrap().as_str().parse::<i32>().expect("Hit points not numeric"),
        damage: number_regex.find(&lines[1])
            .unwrap().as_str().parse::<i32>().expect("Damage not numeric"),
    }
}

fn find_least_mana_win(game: &Game) -> Option<Game> {
    // println!(
    //     "{} {} {} {} {:?}",
    //     game.spells_cast.len(),
    //     game.player.mana,
    //     game.player.health,
    //     game.boss.health,
    //     game.spells_cast.last()
    // );
    // Set up the worst possible game as the one to beat
    let mut best_game: Option<Game> = None;
    // Iterate through each available spell to cast and try casting it.
    for spell in game.possible_spells() {
        let mut game_next = game.clone();
        game_next.spells_cast.push(spell);
        let end_state = game_next.player_turn(spell);
        match end_state {
            GameEndState::Ongoing => (),
            GameEndState::BossWon => continue,
            GameEndState::WizardWon => {
                match best_game {
                    None => {
                        best_game = Some(game_next.clone());
                    },
                    Some(ref g) => {
                        if game_next.mana_spent < g.mana_spent {
                            best_game = Some(game_next.clone());
                        }
                    }
                }
                continue;
            }
        }
        let end_state = game_next.boss_turn();
        match end_state {
            GameEndState::Ongoing => (),
            GameEndState::BossWon => continue,
            GameEndState::WizardWon => {
                match best_game {
                    None => {
                        best_game = Some(game_next.clone());
                    },
                    Some(ref g) => {
                        if game_next.mana_spent < g.mana_spent {
                            best_game = Some(game_next.clone());
                        }
                    }
                }
                continue;
            }
        }
        // If the battle is still going, recurse.
        let best_game_next = find_least_mana_win(&game_next);
        best_game = match (best_game, best_game_next) {
            (None, None) => None,
            (Some(g), None) => Some(g),
            (None, Some(ng)) => Some(ng),
            (Some(g), Some(ng)) => {
                if ng.mana_spent < g.mana_spent {
                    Some(ng)
                } else {
                    Some(g)
                }
            }
        }
    }
    best_game
}

#[derive(Clone)]
struct Wizard {
    health: i32,
    mana: i32,
    armour: i32
}

#[derive(Clone)]
struct Boss {
    health: i32,
    damage: i32
}

#[derive(Clone)]
struct Game {
    player: Wizard,
    boss: Boss,
    shield_effect_remaining: i32,
    poison_effect_remaining: i32,
    recharge_effect_remaining: i32,
    mana_spent: i32,
    spells_cast: Vec<Spell>
}

impl Game {
    fn new(player: Wizard, boss: Boss) -> Game {
        Game {
            player,
            boss,
            shield_effect_remaining: 0,
            poison_effect_remaining: 0,
            recharge_effect_remaining: 0,
            mana_spent: 0,
            spells_cast: vec![]
        }
    }

    pub fn player_turn(&mut self, spell_to_cast: Spell) -> GameEndState {
        self.player.armour = 0;
        self.apply_effects();
        self.player.mana -= spell_to_cast.cost();
        self.mana_spent += spell_to_cast.cost();
        match spell_to_cast {
            Spell::MagicMissile => self.cast_magic_missile(),
            Spell::Drain        => self.cast_drain(),
            Spell::Shield       => self.cast_shield(),
            Spell::Poison       => self.cast_poison(),
            Spell::Recharge     => self.cast_recharge()
        }
        self.get_game_end_state()
    }

    pub fn boss_turn(&mut self) -> GameEndState {
        self.player.armour = 0;
        self.apply_effects();
        self.player.health -= max(self.boss.damage - self.player.armour, 1);
        self.get_game_end_state()
    }

    pub fn possible_spells(&self) -> Vec<Spell> {
        let mut spells = Vec::<Spell>::with_capacity(5);
        spells.push(Spell::MagicMissile);
        if self.player.mana > Spell::Drain.cost() {
            spells.push(Spell::Drain);
        }
        if self.player.mana > Spell::Shield.cost() && self.shield_effect_remaining == 0 {
            spells.push(Spell::Shield);
        }
        if self.player.mana > Spell::Poison.cost() && self.poison_effect_remaining == 0 {
            spells.push(Spell::Poison);
        }
        if self.player.mana > Spell::Recharge.cost() && self.recharge_effect_remaining == 0 {
            spells.push(Spell::Recharge);
        }
        spells
    }

    fn cast_magic_missile(&mut self) {
        self.boss.health -= 4;
    }

    fn cast_drain(&mut self) {
        self.player.health += 2;
        self.boss.health -= 2;
    }

    pub fn cast_shield(&mut self) {
        self.shield_effect_remaining = 6;
    }

    pub fn cast_poison(&mut self) {
        self.poison_effect_remaining = 6;
    }

    pub fn cast_recharge(&mut self) {
        self.recharge_effect_remaining = 5;
    }

    fn apply_effects(&mut self) {
        if self.shield_effect_remaining > 0 {
            self.apply_shield();
            self.shield_effect_remaining -= 1;
        }
        if self.poison_effect_remaining > 0 {
            self.apply_poison();
            self.poison_effect_remaining -= 1;
        }
        if self.recharge_effect_remaining > 0 {
            self.apply_recharge();
            self.recharge_effect_remaining -= 1;
        }
    }

    fn apply_shield(&mut self) {
        self.player.armour += 7;
    }

    fn apply_poison(&mut self) {
        self.boss.health -= 3;
    }

    fn apply_recharge(&mut self) {
        self.player.mana += 101;
    }

    fn get_game_end_state(&self) -> GameEndState {
        if self.boss.health < -100 || self.player.health < -100 {
            println!("bad");
        }
        if self.boss.health <= 0 {
            GameEndState::WizardWon
        }
        else if self.player.mana < Spell::MagicMissile.cost() || self.player.health <= 0 {
            GameEndState::BossWon
        }
        else {
            GameEndState::Ongoing
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum Spell {
    MagicMissile,
    Drain,
    Shield,
    Poison,
    Recharge,
}

impl Spell {
    pub fn cost(&self) -> i32 {
        match self {
            Spell::MagicMissile => 53,
            Spell::Drain        => 73,
            Spell::Shield       => 113,
            Spell::Poison       => 173,
            Spell::Recharge     => 229
        }
    }
}

impl Display for Spell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let spell_str = match self {
            Spell::MagicMissile => "Magic Missile",
            Spell::Drain        => "Drain",
            Spell::Shield       => "Shield",
            Spell::Poison       => "Poison",
            Spell::Recharge     => "Recharge"
        };
        write!(f, "{}", spell_str)
    }
}

enum GameEndState {
    Ongoing,
    WizardWon,
    BossWon,
}