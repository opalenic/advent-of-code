
#![feature(plugin)]

#![plugin(regex_macros)]
extern crate regex;

use std::io;
use std::io::prelude::*;


#[derive(Eq, PartialEq, Debug, Clone, Copy)]
enum Spell {
    MagicMissile,
    Drain,
    Shield,
    Poison,
    Recharge,
}

const MAGIC_MISSILE_COST: u32 = 53;
const MAGIC_MISSILE_DAMAGE: u32 = 4;

const DRAIN_COST: u32 = 73;
const DRAIN_DAMAGE: u32 = 2;

const SHIELD_COST: u32 = 113;
const SHIELD_ARMOR: u32 = 7;
const SHIELD_DURATION: u8 = 6;

const POISON_COST: u32 = 173;
const POISON_DAMAGE: u32 = 3;
const POISON_DURATION: u8 = 6;

const RECHARGE_COST: u32 = 229;
const RECHARGE_MANA_AMOUNT: u32 = 101;
const RECHARGE_DURATION: u8 = 5;

trait Affectable {
    fn apply_effects(&mut self) -> ActionResult;
}

trait Attackable {
    fn attack(&mut self, damage: u32) -> ActionResult;
}

#[derive(Clone, Debug)]
struct Player {
    hp: u32,
    armor: u32,
    mana: u32,
    mana_spent: u32,
    recharge: Option<u8>,
    shield: Option<u8>,
}

impl Player {
    fn new(hp: u32, mana: u32) -> Player {
        Player {
            hp: hp,
            armor: 0,
            mana: mana,
            mana_spent: 0,
            recharge: None,
            shield: None,
        }
    }

    fn cast(&mut self, spell: Spell, boss: &mut Boss) -> ActionResult {
        if (spell == Spell::Recharge && self.recharge.is_some()) ||
           (spell == Spell::Shield && self.shield.is_some()) ||
           (spell == Spell::Poison && boss.poison.is_some()) {

            panic!("Spell already in effect: {:#?}", spell);
        }

        // println!("Casting: {:?}", spell);

        let mut action_res = ActionResult::BothAlive;

        match spell {
            Spell::MagicMissile => {
                self.mana -= MAGIC_MISSILE_COST;
                self.mana_spent += MAGIC_MISSILE_COST;

                action_res = boss.attack(MAGIC_MISSILE_DAMAGE);
            }
            Spell::Drain => {
                self.mana -= DRAIN_COST;
                self.mana_spent += DRAIN_COST;

                action_res = boss.attack(DRAIN_DAMAGE);
                self.hp += DRAIN_DAMAGE;
            }
            Spell::Shield => {
                self.mana -= SHIELD_COST;
                self.mana_spent += SHIELD_COST;

                self.armor += SHIELD_ARMOR;
                self.shield = Some(SHIELD_DURATION);
            }
            Spell::Poison => {
                self.mana -= POISON_COST;
                self.mana_spent += POISON_COST;

                boss.poison = Some(POISON_DURATION);
            }
            Spell::Recharge => {
                self.mana -= RECHARGE_COST;
                self.mana_spent += RECHARGE_COST;

                self.recharge = Some(RECHARGE_DURATION);
            }
        }

        action_res
    }

    fn castable_spells(&self, boss: &Boss) -> Vec<Spell> {
        let mut spells = Vec::new();

        if self.mana >= MAGIC_MISSILE_COST {
            spells.push(Spell::MagicMissile);
        }

        if self.mana >= DRAIN_COST {
            spells.push(Spell::Drain);
        }

        if self.mana >= SHIELD_COST && self.shield.is_none() {
            spells.push(Spell::Shield);
        }

        if self.mana >= POISON_COST && boss.poison.is_none() {
            spells.push(Spell::Poison);
        }

        if self.mana >= RECHARGE_COST && self.recharge.is_none() {
            spells.push(Spell::Recharge);
        }

        spells
    }
}

impl Affectable for Player {
    fn apply_effects(&mut self) -> ActionResult {
        // println!("Applying effects to player.");

        if self.shield.is_some() {
            // println!("Shield in effect.");
            let dur = self.shield.unwrap() - 1;

            if dur == 0 {
                // println!("Shield expired.");
                self.shield = None;
                self.armor -= SHIELD_ARMOR;
            } else {
                // println!("Remaining shield duration: {}", dur);
                self.shield = Some(dur);
            }
        }

        if self.recharge.is_some() {
            // println!("Recharge in effect.");
            let dur = self.recharge.unwrap() - 1;

            self.mana += RECHARGE_MANA_AMOUNT;

            if dur == 0 {
                // println!("Recharge expired.");
                self.recharge = None;
            } else {
                // println!("Remaining recharge duration: {}", dur);
                self.recharge = Some(dur);
            }
        }

        ActionResult::BothAlive
    }
}

impl Attackable for Player {
    fn attack(&mut self, damage: u32) -> ActionResult {

        let damage_done = if let Some(v) = damage.checked_sub(self.armor) {
            v
        } else {
            1
        };

        self.hp = if let Some(v) = self.hp.checked_sub(damage_done) {
            v
        } else {
            0
        };

        if self.hp == 0 {
            ActionResult::PlayerDead
        } else {
            ActionResult::BothAlive
        }
    }
}


#[derive(Clone, Debug)]
struct Boss {
    hp: u32,
    damage: u32,
    poison: Option<u8>,
}

impl Boss {
    fn new(hp: u32, damage: u32) -> Boss {
        Boss {
            hp: hp,
            damage: damage,
            poison: None,
        }
    }
}

impl Affectable for Boss {
    fn apply_effects(&mut self) -> ActionResult {

        if self.poison.is_some() {
            let dur = self.poison.unwrap() - 1;

            self.attack(POISON_DAMAGE);

            if dur == 0 {
                self.poison = None;
            } else {
                self.poison = Some(dur);
            }
        }

        if self.hp == 0 {
            ActionResult::BossDead
        } else {
            ActionResult::BothAlive
        }
    }
}

impl Attackable for Boss {
    fn attack(&mut self, damage: u32) -> ActionResult {
        self.hp = if let Some(v) = self.hp.checked_sub(damage) {
            v
        } else {
            0
        };

        if self.hp == 0 {
            ActionResult::BossDead
        } else {
            ActionResult::BothAlive
        }
    }
}


#[derive(Clone, Debug)]
struct State {
    player: Player,
    boss: Boss,
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
enum ActionResult {
    PlayerDead,
    BossDead,
    BothAlive,
}

enum TurnResult {
    BossKilled(u32),
    BossAlive(Vec<State>),
}

impl State {
    fn apply_effects(&mut self) -> ActionResult {
        self.player.apply_effects();

        self.boss.apply_effects()
    }

    fn player_action(&self) -> Vec<(State, ActionResult)> {

        // choose spell
        let spells = self.player.castable_spells(&self.boss);
        // use spell
        spells.into_iter()
              .map(|spell| {
                  let mut new_state = self.clone();

                  let result = new_state.player
                                        .cast(spell, &mut new_state.boss);

                  (new_state, result)
              })
              .collect()
    }

    fn boss_action(&mut self) -> ActionResult {
        self.player.attack(self.boss.damage)
    }

    fn execute_turn(mut self) -> TurnResult {

        // Apply effects before player's turn.
        match self.apply_effects() {
            ActionResult::BossDead => {
                println!("Boss dead by effect: {:?}", self);
                return TurnResult::BossKilled(self.player.mana_spent);
            }
            ActionResult::PlayerDead => {
                panic!("There should be no way for the player to be killed by spell effects.");
            }
            ActionResult::BothAlive => {}
        }

        // Let player cast all available spells.
        let new_states = self.player_action();

        // The boss might have been killed by one of the spells.
        // Find the one with the least mana used.
        let mut min_mana = None;
        for &(ref state, action_res) in &new_states {
            if action_res == ActionResult::BossDead {
                min_mana = if let Some(mana) = min_mana {
                    if mana > state.player.mana_spent {
                        Some(state.player.mana_spent)
                    } else {
                        Some(mana)
                    }
                } else {
                    Some(state.player.mana_spent)
                }
            }
        }

        if let Some(mana) = min_mana {
            println!("Boss killed by spell: {:?}", new_states);
            return TurnResult::BossKilled(mana);
        }


        // Execute boss' turn for each of the possible spells cast.
        let mut next_turn_states = Vec::with_capacity(new_states.len());

        for (mut state, _) in new_states.into_iter() {

            match state.apply_effects() {
                ActionResult::BossDead => {
                    println!("Boss killed by effect: {:?}", state);
                    return TurnResult::BossKilled(state.player.mana_spent);
                }
                ActionResult::PlayerDead => {
                    panic!("There should be no way for the player to be killed by spell effects.");
                }
                ActionResult::BothAlive => {}
            }

            if state.boss_action() != ActionResult::PlayerDead {
                next_turn_states.push(state);
            }
        }

        TurnResult::BossAlive(next_turn_states)
    }
}

fn find_lowest_mana_use(player: &Player, boss: &Boss) -> u32 {
    let mut states = vec![State {
                              player: player.clone(),
                              boss: boss.clone(),
                          }];


    loop {
        let state = states.remove(0);

        match state.execute_turn() {
            TurnResult::BossAlive(new_states) => {
                states.extend(new_states);
            }
            TurnResult::BossKilled(mana) => {
                return mana;
            }
        }


    }
}

fn find_lowest_mana_use_hard(player: &Player, boss: &Boss) -> u32 {
    let mut states = vec![State {
                              player: player.clone(),
                              boss: boss.clone(),
                          }];


    loop {
        let mut state = states.remove(0);

        state.player.attack(1);
        if state.player.hp == 0 {
            continue;
        }

        match state.execute_turn() {
            TurnResult::BossAlive(new_states) => {
                states.extend(new_states);
            }
            TurnResult::BossKilled(mana) => {
                return mana;
            }
        }


    }
}


fn main() {

    let player = Player::new(50, 500);


    let stdin = io::stdin();

    let mut hp_line = String::new();
    let mut damage_line = String::new();

    stdin.read_line(&mut hp_line).unwrap();
    stdin.read_line(&mut damage_line).unwrap();

    let hp_regex = regex!(r"^Hit Points: (?P<hp>[:digit:]+)$");
    let damage_regex = regex!(r"^Damage: (?P<dmg>[:digit:]+)$");


    let boss_hp = hp_regex.captures(hp_line.trim()).unwrap()
                          .name("hp").unwrap()
                          .parse::<u32>().unwrap();

    let boss_damage = damage_regex.captures(damage_line.trim()).unwrap()
                                  .name("dmg").unwrap()
                                  .parse::<u32>().unwrap();


    let boss = Boss::new(boss_hp, boss_damage);

    println!("Fight #1");
    println!("Mana used: {}", find_lowest_mana_use(&player, &boss));

    println!("------------------------------------");
    println!("Fight #2");
    println!("Mana used: {}", find_lowest_mana_use_hard(&player, &boss));

}

#[cfg(test)]
mod tests {

    use super::Player;
    use super::Boss;
    use super::State;
    use super::ActionResult;
    use super::Spell;

    fn check_hp(state: &State, player_hp: u32, boss_hp: u32) {
        assert_eq!(state.player.hp, player_hp);
        assert_eq!(state.boss.hp, boss_hp);
    }

    #[test]
    fn battle_mechanics_test() {

        let p = Player::new(10, 250);

        let b = Boss::new(13, 8);

        let mut state = State {
            player: p,
            boss: b,
        };

        assert_eq!(state.apply_effects(),
                   ActionResult::BothAlive);
        check_hp(&state, 10, 13);

        assert_eq!(state.player.cast(Spell::Poison, &mut state.boss),
                   ActionResult::BothAlive);
        check_hp(&state, 10, 13);


        assert_eq!(state.apply_effects(),
                   ActionResult::BothAlive);
        check_hp(&state, 10, 10);

        assert_eq!(state.boss_action(),
                   ActionResult::BothAlive);
        check_hp(&state, 2, 10);


        assert_eq!(state.apply_effects(),
                   ActionResult::BothAlive);
        check_hp(&state, 2, 7);

        assert_eq!(state.player.cast(Spell::MagicMissile, &mut state.boss),
                   ActionResult::BothAlive);
        check_hp(&state, 2, 3);


        assert_eq!(state.apply_effects(),
                   ActionResult::BossDead);
        check_hp(&state, 2, 0);

        assert_eq!(state.boss.poison, Some(3));
    }

    #[test]
    fn battle_mechanics_2_test() {

        let p = Player::new(10, 250);

        let b = Boss::new(14, 8);

        let mut state = State {
            player: p,
            boss: b,
        };

        assert_eq!(state.apply_effects(),
                   ActionResult::BothAlive);
        check_hp(&state, 10, 14);

        assert_eq!(state.player.cast(Spell::Recharge, &mut state.boss),
                   ActionResult::BothAlive);
        check_hp(&state, 10, 14);


        assert_eq!(state.apply_effects(),
                   ActionResult::BothAlive);
        check_hp(&state, 10, 14);

        assert_eq!(state.boss_action(),
                   ActionResult::BothAlive);
        check_hp(&state, 2, 14);



        assert_eq!(state.apply_effects(),
                   ActionResult::BothAlive);
        check_hp(&state, 2, 14);

        assert_eq!(state.player.cast(Spell::Shield, &mut state.boss),
                   ActionResult::BothAlive);
        check_hp(&state, 2, 14);


        assert_eq!(state.apply_effects(),
                   ActionResult::BothAlive);
        check_hp(&state, 2, 14);

        assert_eq!(state.boss_action(),
                   ActionResult::BothAlive);
        check_hp(&state, 1, 14);



        assert_eq!(state.apply_effects(),
                   ActionResult::BothAlive);
        check_hp(&state, 1, 14);

        assert_eq!(state.player.cast(Spell::Drain, &mut state.boss),
                   ActionResult::BothAlive);
        check_hp(&state, 3, 12);


        assert_eq!(state.apply_effects(),
                   ActionResult::BothAlive);
        check_hp(&state, 3, 12);

        assert_eq!(state.boss_action(),
                   ActionResult::BothAlive);
        check_hp(&state, 2, 12);



        assert_eq!(state.apply_effects(),
                   ActionResult::BothAlive);
        check_hp(&state, 2, 12);

        assert_eq!(state.player.cast(Spell::Poison, &mut state.boss),
                   ActionResult::BothAlive);
        check_hp(&state, 2, 12);


        assert_eq!(state.apply_effects(),
                   ActionResult::BothAlive);
        check_hp(&state, 2, 9);

        assert_eq!(state.boss_action(),
                   ActionResult::BothAlive);
        check_hp(&state, 1, 9);



        assert_eq!(state.apply_effects(),
                   ActionResult::BothAlive);
        check_hp(&state, 1, 6);

        assert_eq!(state.player.cast(Spell::MagicMissile, &mut state.boss),
                   ActionResult::BothAlive);
        check_hp(&state, 1, 2);


        assert_eq!(state.apply_effects(),
                   ActionResult::BossDead);
        check_hp(&state, 1, 0);

    }
}
