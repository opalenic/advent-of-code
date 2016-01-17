extern crate regex;
extern crate permutohedron;

use regex::Regex;

use std::collections::HashMap;

use permutohedron::Heap;

use std::io;
use std::io::prelude::*;


struct Seating {
    people: Vec<String>,
    relations: HashMap<String, HashMap<String, i32>>
}

impl Seating {
    fn from_str(input: &str) -> Seating {
        let re = Regex::new("^(?P<person>[:alpha:]+) would (?P<action>lose|gain) (?P<amount>[:digit:]+) happiness units by sitting next to (?P<target>[:alpha:]+).$").unwrap();

        let mut p: Vec<String> = Vec::new();

        let mut rel: HashMap<String, HashMap<String, i32>> = HashMap::new();

        for line in input.lines() {

            let caps = re.captures(&line).unwrap();
            let person = caps.name("person").unwrap().to_string();

            let mut amount = caps.name("amount").unwrap().parse::<i32>().unwrap();
            if caps.name("action").unwrap() == "lose" {
                amount = -amount;
            }

            let target = caps.name("target").unwrap().to_string();

            if !p.contains(&person) {
                p.push(person.clone());
            }

            let person_rel = rel.entry(person).or_insert(HashMap::new());

            person_rel.insert(target, amount);

        }

        Seating { people: p, relations: rel }
    }

    fn best_seating(&mut self) -> (Vec<String>, i32) {

        let perm = Heap::new(&mut self.people);

        let mut seating = Vec::new();
        let mut max_happiness = std::i32::MIN;


        for p in perm {

            let mut happiness = 0;

            for pair in p.windows(2) {
                let person = &pair[0];
                let neighbor = &pair[1];

                happiness += self.relations[person][neighbor];
                happiness += self.relations[neighbor][person];
            }

            let first_person = p.first().unwrap();
            let last_person = p.last().unwrap();

            happiness += self.relations[first_person][last_person];
            happiness += self.relations[last_person][first_person];


            if max_happiness < happiness {
                seating = p.clone();
                max_happiness = happiness;
            }
        }

        (seating, max_happiness)
    }

    fn add_self(&mut self) {
        let mut my_relations = HashMap::new();

        for person in &self.people {
            my_relations.insert(person.clone(), 0);
            self.relations.get_mut(person).unwrap().insert("Me".to_string(), 0);
        }

        self.relations.insert("Me".to_string(), my_relations);
        self.people.push("Me".to_string());
    }
}

fn main() {

    let mut input_str = String::new();

    let _ = io::stdin().read_to_string(&mut input_str);

    let mut seating = Seating::from_str(&input_str);

    let (arrangement, happiness) = seating.best_seating();

    println!("The best seating arrangement is: {:?}", arrangement);
    println!("Total happiness achieved: {}", happiness);

    seating.add_self();
    let (arrangement_w_me, happiness_w_me) = seating.best_seating();

    println!("---------------------------------------------------");
    println!("Added myself.");
    println!("The best seating arrangement is: {:?}", arrangement_w_me);
    println!("Total happiness achieved: {}", happiness_w_me);
}


#[cfg(test)]
mod tests {
    use super::Seating;
    use std::collections::HashMap;

    const TEST_INPUT: &'static str =
                "Alice would gain 54 happiness units by sitting next to Bob.\n\
                 Alice would lose 79 happiness units by sitting next to Carol.\n\
                 Alice would lose 2 happiness units by sitting next to David.\n\
                 Bob would gain 83 happiness units by sitting next to Alice.\n\
                 Bob would lose 7 happiness units by sitting next to Carol.\n\
                 Bob would lose 63 happiness units by sitting next to David.\n\
                 Carol would lose 62 happiness units by sitting next to Alice.\n\
                 Carol would gain 60 happiness units by sitting next to Bob.\n\
                 Carol would gain 55 happiness units by sitting next to David.\n\
                 David would gain 46 happiness units by sitting next to Alice.\n\
                 David would lose 7 happiness units by sitting next to Bob.\n\
                 David would gain 41 happiness units by sitting next to Carol.";


    #[test]
    fn create_test() {
        let seating = Seating::from_str(TEST_INPUT);

        assert_eq!(seating.people, vec!["Alice".to_string(),
                                        "Bob".to_string(),
                                        "Carol".to_string(),
                                        "David".to_string()]);

        let mut relations = HashMap::new();
        {
            relations.insert("Alice".to_string(), HashMap::new());
            let mut alice_rel = relations.get_mut("Alice").unwrap();
            alice_rel.insert("Bob".to_string(), 54);
            alice_rel.insert("Carol".to_string(), -79);
            alice_rel.insert("David".to_string(), -2);
        }

        {
            relations.insert("Bob".to_string(), HashMap::new());
            let mut bob_rel = relations.get_mut("Bob").unwrap();
            bob_rel.insert("Alice".to_string(), 83);
            bob_rel.insert("Carol".to_string(), -7);
            bob_rel.insert("David".to_string(), -63);
        }

        {
            relations.insert("Carol".to_string(), HashMap::new());
            let mut carol_rel = relations.get_mut("Carol").unwrap();
            carol_rel.insert("Alice".to_string(), -62);
            carol_rel.insert("Bob".to_string(), 60);
            carol_rel.insert("David".to_string(), 55);
        }

        {
            relations.insert("David".to_string(), HashMap::new());
            let mut david_rel = relations.get_mut("David").unwrap();
            david_rel.insert("Alice".to_string(), 46);
            david_rel.insert("Bob".to_string(), -7);
            david_rel.insert("Carol".to_string(), 41);
        }

        assert_eq!(seating.relations, relations);
    }

    #[test]
    fn most_happiness_test() {
        let mut seating = Seating::from_str(TEST_INPUT);

        let (seating, happiness) = seating.best_seating();

        assert_eq!(330, happiness);

        let seating_var1 = vec!["David".to_string(), "Alice".to_string(), "Bob".to_string(), "Carol".to_string()];
        let seating_var2 = vec!["Carol".to_string(), "David".to_string(), "Alice".to_string(), "Bob".to_string()];
        let seating_var3 = vec!["Bob".to_string(), "Carol".to_string(), "David".to_string(), "Alice".to_string()];
        let seating_var4 = vec!["Alice".to_string(), "Bob".to_string(), "Carol".to_string(), "David".to_string()];

        assert!(seating == seating_var1 || seating == seating_var2 || seating == seating_var3 || seating == seating_var4);
    }
}