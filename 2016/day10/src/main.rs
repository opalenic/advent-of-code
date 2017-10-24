
extern crate regex;

use regex::Regex;

use std::collections::{HashMap, HashSet};

use std::io;
use std::io::prelude::*;

use std::cell::RefCell;

#[derive(Debug, PartialEq, Eq)]
struct Output {
    contained_chips: HashSet<usize>,
}

impl Output {
    fn put_chip(&mut self, chip_id: usize) {
        self.contained_chips.insert(chip_id);
    }
}

impl Default for Output {
    fn default() -> Output {
        Output { contained_chips: HashSet::new() }
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Action {
    GiveToBot(usize),
    PutIntoOutput(usize),
}

#[derive(Debug, PartialEq, Eq)]
struct Bot {
    chips: (Option<usize>, Option<usize>),
    action_lower: Option<Action>,
    action_higher: Option<Action>,
}

impl Default for Bot {
    fn default() -> Bot {
        Bot {
            chips: (None, None),
            action_lower: None,
            action_higher: None,
        }
    }
}

impl Bot {
    fn give_chip(&mut self, new_chip: usize) {
        if self.chips.0.is_none() {
            self.chips.0 = Some(new_chip);
        } else if self.chips.1.is_none() {
            self.chips.1 = Some(new_chip);
        } else {
            panic!("bot has full hands");
        }
    }

    fn can_accept_chip(&self) -> bool {
        self.chips.0.is_none() || self.chips.1.is_none()
    }

    fn get_actions(&self) -> Option<((Action, usize), (Action, usize))> {
        if self.chips.0.is_some() && self.chips.1.is_some() {
            let left = self.chips.0.unwrap();
            let right = self.chips.1.unwrap();

            let (low, high) = if left <= right {
                (left, right)
            } else {
                (right, left)
            };

            Some(((self.action_lower.unwrap(), low), (
                self.action_higher.unwrap(),
                high,
            )))
        } else {
            None
        }
    }

    fn take_chips(&mut self) {
        self.chips = (None, None);
    }
}


fn parse_input(input: &str) -> (HashMap<usize, Output>, HashMap<usize, RefCell<Bot>>) {
    let mut outputs = HashMap::new();
    let mut bots = HashMap::new();

    let value_re = Regex::new(r"value (?P<chip_id>[0-9]+) goes to bot (?P<bot_id>[0-9]+)").unwrap();
    let bot_instr_re = Regex::new(r"bot (?P<bot_id>[0-9]+) gives low to (?P<dest_low>output|bot) (?P<dest_low_id>[0-9]+) and high to (?P<dest_high>output|bot) (?P<dest_high_id>[0-9]+)")
        .unwrap();

    for line in input.lines() {
        if let Some(value_caps) = value_re.captures(line) {
            let chip_id = value_caps.name("chip_id").unwrap().parse().unwrap();
            let bot_id = value_caps.name("bot_id").unwrap().parse().unwrap();

            let bot = bots.entry(bot_id).or_insert(RefCell::new(Bot::default()));
            bot.get_mut().give_chip(chip_id);

        } else if let Some(bot_instr_caps) = bot_instr_re.captures(line) {
            let bot_id = bot_instr_caps.name("bot_id").unwrap().parse().unwrap();
            let dest_low = bot_instr_caps.name("dest_low").unwrap();
            let dest_low_id = bot_instr_caps.name("dest_low_id").unwrap().parse().unwrap();
            let dest_high = bot_instr_caps.name("dest_high").unwrap();
            let dest_high_id = bot_instr_caps
                .name("dest_high_id")
                .unwrap()
                .parse()
                .unwrap();

            if dest_low == "bot" {
                let bot = bots.entry(bot_id).or_insert(RefCell::new(Bot::default()));
                bot.get_mut().action_lower = Some(Action::GiveToBot(dest_low_id));

            } else if dest_low == "output" {
                let bot = bots.entry(bot_id).or_insert(RefCell::new(Bot::default()));
                bot.get_mut().action_lower = Some(Action::PutIntoOutput(dest_low_id));

                outputs.entry(dest_low_id).or_insert(Output::default());
            }

            if dest_high == "bot" {
                let bot = bots.entry(bot_id).or_insert(RefCell::new(Bot::default()));
                bot.get_mut().action_higher = Some(Action::GiveToBot(dest_high_id));

            } else if dest_high == "output" {
                let bot = bots.entry(bot_id).or_insert(RefCell::new(Bot::default()));
                bot.get_mut().action_higher = Some(Action::PutIntoOutput(dest_high_id));

                outputs.entry(dest_high_id).or_insert(Output::default());
            }

        } else {
            panic!();
        }
    }


    (outputs, bots)
}

fn find_ready_bot<'a, 'b>(
    bots: &'a HashMap<usize, RefCell<Bot>>,
    unusable_bot_ids: &'b HashSet<usize>,
) -> Option<(&'a usize, &'a RefCell<Bot>)> {
    bots.iter()
        .filter(|&(id, bot)| {
            !unusable_bot_ids.contains(id) && bot.borrow().get_actions().is_some()
        })
        .next()
}


fn process_chips(outputs: &mut HashMap<usize, Output>, bots: &mut HashMap<usize, RefCell<Bot>>) {

    let mut unusable_bot_ids = HashSet::new();

    while let Some((id, bot)) = find_ready_bot(bots, &unusable_bot_ids) {

        let ((low_action, low_chip_id), (high_action, high_chip_id)) =
            bot.borrow().get_actions().unwrap();

        if low_chip_id == 17 && high_chip_id == 61 {
            println!("The bot you're looking for is #{}", id);
        }

        if let Action::GiveToBot(low_bot_id) = low_action {
            let low_bot = bots.get(&low_bot_id).unwrap().borrow_mut();
            if !low_bot.can_accept_chip() {
                unusable_bot_ids.insert(*id);
                continue;
            }
        }

        if let Action::GiveToBot(high_bot_id) = high_action {
            let high_bot = bots.get(&high_bot_id).unwrap().borrow_mut();
            if !high_bot.can_accept_chip() {
                unusable_bot_ids.insert(*id);
                continue;
            }
        }

        match low_action {
            Action::GiveToBot(tgt_bot_id) => {
                bots.get(&tgt_bot_id).unwrap().borrow_mut().give_chip(
                    low_chip_id,
                );
            }
            Action::PutIntoOutput(output_id) => {
                outputs.get_mut(&output_id).unwrap().put_chip(low_chip_id);
            }
        }

        match high_action {
            Action::GiveToBot(tgt_bot_id) => {
                bots.get(&tgt_bot_id).unwrap().borrow_mut().give_chip(
                    high_chip_id,
                );
            }
            Action::PutIntoOutput(output_id) => {
                outputs.get_mut(&output_id).unwrap().put_chip(high_chip_id);
            }
        }

        bot.borrow_mut().take_chips();
        unusable_bot_ids.clear();
    }

}

fn get_product_of_outputs(outputs: &HashMap<usize, Output>) -> usize {
    outputs
        .iter()
        .filter_map(|(id, out)| -> Option<usize> {
            if *id == 0 || *id == 1 || *id == 2 {
                Some(out.contained_chips.iter().product())
            } else {
                None
            }
        })
        .product()
}

fn main() {
    let mut instruction_str = String::new();
    io::stdin().read_to_string(&mut instruction_str).expect(
        "Invalid input string!",
    );

    let (mut outputs, mut bots) = parse_input(&instruction_str);

    process_chips(&mut outputs, &mut bots);

    println!(
        "Product of outputs 1, 2, 3 = {}",
        get_product_of_outputs(&outputs)
    );
}


#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::cell::RefCell;

    use super::parse_input;
    use super::process_chips;
    use super::get_product_of_outputs;
    use super::Bot;
    use super::Output;
    use super::Action;

    const TEST_INPUT: &'static str = "value 5 goes to bot 2\n\
                                      bot 2 gives low to bot 1 and high to bot 0\n\
                                      value 3 goes to bot 1\n\
                                      bot 1 gives low to output 1 and high to bot 0\n\
                                      bot 0 gives low to output 2 and high to output 0\n\
                                      value 2 goes to bot 2";

    #[test]
    fn parse_test() {
        let mut expected_outputs = HashMap::new();
        expected_outputs.insert(0, Output::default());
        expected_outputs.insert(1, Output::default());
        expected_outputs.insert(2, Output::default());

        let mut expected_bots = HashMap::new();
        expected_bots.insert(
            0,
            RefCell::new(Bot {
                chips: (None, None),
                action_lower: Some(Action::PutIntoOutput(2)),
                action_higher: Some(Action::PutIntoOutput(0)),
            }),
        );

        expected_bots.insert(
            1,
            RefCell::new(Bot {
                chips: (Some(3), None),
                action_lower: Some(Action::PutIntoOutput(1)),
                action_higher: Some(Action::GiveToBot(0)),
            }),
        );

        expected_bots.insert(
            2,
            RefCell::new(Bot {
                chips: (Some(5), Some(2)),
                action_lower: Some(Action::GiveToBot(1)),
                action_higher: Some(Action::GiveToBot(0)),
            }),
        );

        let (outputs, bots) = parse_input(TEST_INPUT);

        assert_eq!(expected_outputs, outputs);

        assert_eq!(expected_bots, bots);
    }

    #[test]
    fn run_test() {
        let (mut outputs, mut bots) = parse_input(&TEST_INPUT);

        process_chips(&mut outputs, &mut bots);


        let mut out_0 = Output::default();
        out_0.put_chip(5);
        let mut out_1 = Output::default();
        out_1.put_chip(2);
        let mut out_2 = Output::default();
        out_2.put_chip(3);

        assert_eq!(out_0, outputs[&0]);
        assert_eq!(out_1, outputs[&1]);
        assert_eq!(out_2, outputs[&2]);
    }

    #[test]
    fn product_test() {
        let (mut outputs, mut bots) = parse_input(&TEST_INPUT);

        process_chips(&mut outputs, &mut bots);

        assert_eq!(30, get_product_of_outputs(&outputs));
    }
}
