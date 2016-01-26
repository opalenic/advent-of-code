
#![feature(iter_arith)]

extern crate regex;

use std::str::FromStr;

use regex::Regex;
use std::error::Error;

use std::io;
use std::io::prelude::*;

#[derive(Debug, PartialEq, Eq)]
struct Ingredient {
    name: String,
    capacity: i32,
    durability: i32,
    flavor: i32,
    texture: i32,
    calories: u32,
}

#[derive(Debug)]
struct ParseIngredientErr;

impl<T: Error> From<T> for ParseIngredientErr {
    fn from(_t: T) -> ParseIngredientErr {
        ParseIngredientErr
    }
}

impl FromStr for Ingredient {
    type Err = ParseIngredientErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = try!(Regex::new("^(?P<name>[:alpha:]+): \
                                  capacity (?P<cap_neg>-)?(?P<cap>[:digit:]+), \
                                  durability (?P<dur_neg>-)?(?P<dur>[:digit:]+), \
                                  flavor (?P<flav_neg>-)?(?P<flav>[:digit:]+), \
                                  texture (?P<tex_neg>-)?(?P<tex>[:digit:]+), \
                                  calories (?P<cal>[:digit:]+)$"));

        let caps = try!(re.captures(s).ok_or(ParseIngredientErr));

        let name = try!(caps.name("name").ok_or(ParseIngredientErr));

        let cap_str = try!(caps.name("cap").ok_or(ParseIngredientErr));
        let mut cap = try!(cap_str.parse::<i32>());
        if caps.name("cap_neg").is_some() {
            cap = -cap;
        }

        let dur_str = try!(caps.name("dur").ok_or(ParseIngredientErr));
        let mut dur = try!(dur_str.parse::<i32>());
        if caps.name("dur_neg").is_some() {
            dur = -dur;
        }

        let flav_str = try!(caps.name("flav").ok_or(ParseIngredientErr));
        let mut flav = try!(flav_str.parse::<i32>());
        if caps.name("flav_neg").is_some() {
            flav = -flav;
        }

        let tex_str = try!(caps.name("tex").ok_or(ParseIngredientErr));
        let mut tex = try!(tex_str.parse::<i32>());
        if caps.name("tex_neg").is_some() {
            tex = -tex;
        }

        let cal_str = try!(caps.name("cal").ok_or(ParseIngredientErr));
        let cal = try!(cal_str.parse::<u32>());

        Ok(Ingredient {
            name: name.to_string(),
            capacity: cap,
            durability: dur,
            flavor: flav,
            texture: tex,
            calories: cal,
        })
    }
}

#[derive(Debug)]
struct Combination<'a> {
    ingredients: &'a Vec<Ingredient>,
    combination: Vec<u32>,
    ingredient_sum: u32,
    calorie_tgt: u32,
}

impl<'a> Combination<'a> {
    fn new(ingredients: &Vec<Ingredient>, ingredient_sum: u32, calorie_tgt: u32) -> Combination {
        let len = ingredients.len();

        let mut comb: Vec<u32> = Vec::with_capacity(len);
        for _ in 0..len {
            comb.push(0);
        }

        Combination {
            ingredients: ingredients,
            combination: comb,
            ingredient_sum: ingredient_sum,
            calorie_tgt: calorie_tgt,
        }
    }

    fn get_goodness(&self) -> u32 {
        let mut capacity: i32 = 0;
        let mut durability: i32 = 0;
        let mut flavor: i32 = 0;
        let mut texture: i32 = 0;

        for ingr_idx in 0..self.combination.len() {
            capacity += self.combination[ingr_idx] as i32 * self.ingredients[ingr_idx].capacity;
            durability += self.combination[ingr_idx] as i32 * self.ingredients[ingr_idx].durability;
            flavor += self.combination[ingr_idx] as i32 * self.ingredients[ingr_idx].flavor;
            texture += self.combination[ingr_idx] as i32 * self.ingredients[ingr_idx].texture;
        }

        if capacity < 0 {
            capacity = 0;
        }

        if durability < 0 {
            durability = 0;
        }

        if flavor < 0 {
            flavor = 0;
        }

        if texture < 0 {
            texture = 0;
        }

        capacity as u32 * durability as u32 * flavor as u32 * texture as u32
    }

    fn get_calories(&self) -> u32 {
        let mut calories = 0;
        for ingr_idx in 0..self.combination.len() {
            calories += self.combination[ingr_idx] * self.ingredients[ingr_idx].calories;
        }

        calories
    }

    fn increment_comb(comb: &mut Vec<u32>, base: u32) -> bool {
        *(comb.last_mut().unwrap()) += 1;

        let mut carry: u32 = 0;

        for pos in (0..comb.len()).rev() {
            if carry > 0 {
                comb[pos] += 1;
                carry = 0;
            }

            if comb[pos] > base {
                carry = comb[pos] / base;
                comb[pos] = 0;
            }
        }

        carry == 0
    }

    fn get_next_combination(&mut self) -> bool {

        while Self::increment_comb(&mut self.combination, self.ingredient_sum) {
            if self.combination.iter().sum::<u32>() == self.ingredient_sum {
                return true;
            }
        }

        false
    }

    fn get_next_combination_cals(&mut self) -> bool {

        while self.get_next_combination() {
            if self.get_calories() == self.calorie_tgt {
                return true;
            }
        }

        false
    }

    fn clone_current_combination(&self) -> Vec<u32> {
        self.combination.clone()
    }
}

fn find_max_goodness(comb: &mut Combination) -> (u32, Vec<u32>) {

    let mut max_comb = comb.clone_current_combination();
    let mut max = comb.get_goodness();

    while comb.get_next_combination() {
        let goodness = comb.get_goodness();

        if goodness > max {
            max = goodness;
            max_comb = comb.clone_current_combination();
        }
    }

    (max, max_comb)
}

fn find_max_goodness_cals(comb: &mut Combination) -> (u32, Vec<u32>) {

    let mut max_comb = comb.clone_current_combination();
    let mut max = comb.get_goodness();

    while comb.get_next_combination_cals() {
        let goodness = comb.get_goodness();

        if goodness > max {
            max = goodness;
            max_comb = comb.clone_current_combination();
        }
    }

    (max, max_comb)
}


fn main() {
    let stdin = io::stdin();

    let ingredients: Vec<Ingredient> = stdin.lock()
                                            .lines()
                                            .map(|line| line.unwrap().parse().unwrap())
                                            .collect();

    let mut c = Combination::new(&ingredients, 100, 500);

    let (max, max_comb) = find_max_goodness(&mut c);

    println!("-----------------------------------------------");
    println!("Without calorie constraint.");
    println!("Maximum goodness: {}", max);

    for i in 0..ingredients.len() {
        println!("{} - {}", max_comb[i], ingredients[i].name);
    }


    let (max_cals, max_comb_cals) = find_max_goodness_cals(&mut c);

    println!("-----------------------------------------------");
    println!("With calorie constraint.");
    println!("Maximum goodness: {}", max_cals);

    for i in 0..ingredients.len() {
        println!("{} - {}", max_comb_cals[i], ingredients[i].name);
    }

}


#[cfg(test)]
mod tests {
    use super::Ingredient;
    use super::Combination;
    use super::find_max_goodness;
    use super::find_max_goodness_cals;

    const TEST_INPUT: &'static str =
            "Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8\n\
             Cinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3";

    const BAD_TEST_INPUT: &'static str = "Butterscotch: capacity -1";

    #[test]
    fn parser_test() {
        assert!(BAD_TEST_INPUT.parse::<Ingredient>().is_err());


        let mut input = TEST_INPUT.lines();

        let but_parsed = input.next().unwrap().parse::<Ingredient>().unwrap();
        let but_expected = Ingredient {
            name: "Butterscotch".to_string(),
            capacity: -1,
            durability: -2,
            flavor: 6,
            texture: 3,
            calories: 8,
        };

        assert_eq!(but_expected, but_parsed);

        let cin_parsed = input.next().unwrap().parse::<Ingredient>().unwrap();
        let cin_expected = Ingredient {
            name: "Cinnamon".to_string(),
            capacity: 2,
            durability: 3,
            flavor: -2,
            texture: -1,
            calories: 3,
        };

        assert_eq!(cin_expected, cin_parsed);
    }

    #[test]
    fn max_goodness_test() {
        let ingredients: Vec<Ingredient> =
                TEST_INPUT.lines()
                          .map(|line| line.parse().unwrap())
                          .collect();

        let mut combination = Combination::new(&ingredients, 100, 500);

        let (max, max_comb) = find_max_goodness(&mut combination);

        assert_eq!(62842880, max);
        assert_eq!(vec![44, 56], max_comb);
    }

    #[test]
    fn cal_constraint_test() {
        let ingredients: Vec<Ingredient> =
                TEST_INPUT.lines()
                          .map(|line| line.parse().unwrap())
                          .collect();

        let mut combination = Combination::new(&ingredients, 100, 500);

        let (max, max_comb) = find_max_goodness_cals(&mut combination);

        assert_eq!(57600000, max);
        assert_eq!(vec![40, 60], max_comb);
    }
}
