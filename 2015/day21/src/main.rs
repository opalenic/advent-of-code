
#[macro_use]
extern crate itertools;

use std::u32;

#[derive(Debug, Clone)]
struct Character {
    hp: u32,
    damage: u32,
    armor: u32,
}

impl Character {
    fn new(inv: &Inventory) -> Character {
        Character {
            hp: 100,
            damage: inv.weapon.damage +
                    inv.armor.damage +
                    inv.left_ring.damage +
                    inv.right_ring.damage,
            armor: inv.weapon.armor +
                   inv.armor.armor +
                   inv.left_ring.armor +
                   inv.right_ring.armor,
        }
    }

    fn attack(&self, enemy: &mut Character) -> bool {
        let damage = if self.damage > enemy.armor {
            self.damage - enemy.armor
        } else {
            1
        };

        if damage >= enemy.hp {
            enemy.hp = 0;
            true
        } else {
            enemy.hp -= damage;
            false
        }
    }

    fn fight_it_out(&mut self, enemy: &mut Character) -> (bool, u32) {
        let mut round = 1;

        loop {
            if self.attack(enemy) {
                return (true, round);
            }

            if enemy.attack(self) {
                return (false, round);
            }

            round += 1;
        }
    }
}


#[derive(Debug)]
struct Item {
    name: String,
    cost: u32,
    damage: u32,
    armor: u32,
}

#[derive(Debug)]
struct Inventory<'a> {
    weapon: &'a Item,
    armor: &'a Item,
    left_ring: &'a Item,
    right_ring: &'a Item,
}

impl<'a> Inventory<'a> {
    fn get_cost(&self) -> u32 {
        self.weapon.cost + self.armor.cost + self.left_ring.cost + self.right_ring.cost
    }
}


fn main() {
    let boss_template = Character {
        hp: 100,
        damage: 8,
        armor: 2,
    };

    let weapons = vec![
        Item { name: "Dagger".to_string(), cost: 8, damage: 4, armor: 0 },
        Item { name: "Shortsword".to_string(), cost: 10, damage: 5, armor: 0 },
        Item { name: "Warhammer".to_string(), cost: 25, damage: 6, armor: 0 },
        Item { name: "Longsword".to_string(), cost: 40, damage: 7, armor: 0 },
        Item { name: "Greataxe".to_string(), cost: 74, damage: 8, armor: 0 },
    ];

    let armors = vec![
        Item { name: "Unequipped".to_string(), cost: 0, damage: 0, armor: 0 },
        Item { name: "Leather".to_string(), cost: 13, damage: 0, armor: 1 },
        Item { name: "Chainmail".to_string(), cost: 31, damage: 0, armor: 2 },
        Item { name: "Splintmail".to_string(), cost: 53, damage: 0, armor: 3 },
        Item { name: "Bandedmail".to_string(), cost: 75, damage: 0, armor: 4 },
        Item { name: "Platemail".to_string(), cost: 102, damage: 0, armor: 5 },
    ];

    let rings = vec![
        Item { name: "Unequipped".to_string(), cost: 0, damage: 0, armor: 0 },
        Item { name: "Damage+1".to_string(), cost: 25, damage: 1, armor: 0 },
        Item { name: "Damage+2".to_string(), cost: 50, damage: 2, armor: 0 },
        Item { name: "Damage+3".to_string(), cost: 100, damage: 3, armor: 0 },
        Item { name: "Defense+1".to_string(), cost: 20, damage: 0, armor: 1 },
        Item { name: "Defense+2".to_string(), cost: 40, damage: 0, armor: 2 },
        Item { name: "Defense+3".to_string(), cost: 80, damage: 0, armor: 3 },
    ];


    let equipment_combinations = iproduct!(weapons.iter(),
                                               armors.iter(),
                                               rings.iter(),
                                               rings.iter())
                                         .filter(|&(_, _, lr, rr)| {
                                             lr.name != rr.name ||
                                             (lr.name == "Unequiped" && rr.name == "Unequiped")
                                         });

    let mut min_cost = u32::MAX;
    let mut max_cost = u32::MIN;

    for combination in equipment_combinations {

        let i = Inventory {
            weapon: combination.0,
            armor: combination.1,
            left_ring: combination.2,
            right_ring: combination.3,
        };

        let mut player = Character::new(&i);
        let mut boss = boss_template.clone();

        let (won, _) = player.fight_it_out(&mut boss);
        if won {
            if min_cost > i.get_cost() {
                min_cost = i.get_cost();

                println!("--- Found a new possible minimum configuration.");
                println!("--- Cost: {}", min_cost);
                println!("--- {:?}", i);
            }
        }

        if !won {
            if max_cost < i.get_cost() {
                max_cost = i.get_cost();

                println!("### Found a new possible maximum configuration.");
                println!("### Cost: {}", max_cost);
                println!("### {:?}", i);
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::Character;

    #[test]
    fn fight_test() {
        let mut player = Character {
            hp: 8,
            damage: 5,
            armor: 5,
        };

        let mut boss = Character {
            hp: 12,
            damage: 7,
            armor: 2,
        };

        assert!(!player.attack(&mut boss));
        assert_eq!(boss.hp, 9);
        assert!(!boss.attack(&mut player));
        assert_eq!(player.hp, 6);

        assert!(!player.attack(&mut boss));
        assert_eq!(boss.hp, 6);
        assert!(!boss.attack(&mut player));
        assert_eq!(player.hp, 4);

        assert!(!player.attack(&mut boss));
        assert_eq!(boss.hp, 3);
        assert!(!boss.attack(&mut player));
        assert_eq!(player.hp, 2);

        assert!(player.attack(&mut boss));
        assert_eq!(boss.hp, 0);


        player.hp = 8;
        boss.hp = 12;

        assert_eq!(player.fight_it_out(&mut boss), (true, 4));
    }
}
