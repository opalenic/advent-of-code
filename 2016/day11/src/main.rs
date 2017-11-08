extern crate itertools;

use itertools::Itertools;

use std::collections::BTreeSet;

trait IsShielded
where
    Self: Sized,
{
    fn is_shielded(&self, other_items: &BTreeSet<Self>) -> bool;
    fn is_gen(&self) -> bool;
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Copy, Clone)]
enum Equipment {
    ThuliumGen,
    ThuliumChip,
    PlutoniumGen,
    PlutoniumChip,
    StrontiumGen,
    StrontiumChip,
    PromethiumGen,
    PromethiumChip,
    RutheniumGen,
    RutheniumChip,
}

impl IsShielded for Equipment {
    fn is_shielded(&self, other_items: &BTreeSet<Equipment>) -> bool {
        match *self {
            Equipment::ThuliumChip => {
                other_items.contains(&Equipment::ThuliumGen) ||
                    !other_items.iter().any(|item| item.is_gen())
            }
            Equipment::PlutoniumChip => {
                other_items.contains(&Equipment::PlutoniumGen) ||
                    !other_items.iter().any(|item| item.is_gen())
            }
            Equipment::StrontiumChip => {
                other_items.contains(&Equipment::StrontiumGen) ||
                    !other_items.iter().any(|item| item.is_gen())
            }
            Equipment::PromethiumChip => {
                other_items.contains(&Equipment::PromethiumGen) ||
                    !other_items.iter().any(|item| item.is_gen())
            }
            Equipment::RutheniumChip => {
                other_items.contains(&Equipment::RutheniumGen) ||
                    !other_items.iter().any(|item| item.is_gen())
            }
            _ => true,
        }
    }

    fn is_gen(&self) -> bool {
        match *self {
            Equipment::ThuliumGen | Equipment::PlutoniumGen | Equipment::StrontiumGen |
            Equipment::PromethiumGen | Equipment::RutheniumGen => true,
            _ => false,
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Copy, Clone)]
enum ExtendedEquipment {
    ThuliumGen,
    ThuliumChip,
    PlutoniumGen,
    PlutoniumChip,
    StrontiumGen,
    StrontiumChip,
    PromethiumGen,
    PromethiumChip,
    RutheniumGen,
    RutheniumChip,
    EleriumGen,
    EleriumChip,
    DilithiumGen,
    DilithiumChip,
}

impl IsShielded for ExtendedEquipment {
    fn is_shielded(&self, other_items: &BTreeSet<ExtendedEquipment>) -> bool {
        match *self {
            ExtendedEquipment::ThuliumChip => {
                other_items.contains(&ExtendedEquipment::ThuliumGen) ||
                    !other_items.iter().any(|item| item.is_gen())
            }
            ExtendedEquipment::PlutoniumChip => {
                other_items.contains(&ExtendedEquipment::PlutoniumGen) ||
                    !other_items.iter().any(|item| item.is_gen())
            }
            ExtendedEquipment::StrontiumChip => {
                other_items.contains(&ExtendedEquipment::StrontiumGen) ||
                    !other_items.iter().any(|item| item.is_gen())
            }
            ExtendedEquipment::PromethiumChip => {
                other_items.contains(&ExtendedEquipment::PromethiumGen) ||
                    !other_items.iter().any(|item| item.is_gen())
            }
            ExtendedEquipment::RutheniumChip => {
                other_items.contains(&ExtendedEquipment::RutheniumGen) ||
                    !other_items.iter().any(|item| item.is_gen())
            }
            ExtendedEquipment::EleriumChip => {
                other_items.contains(&ExtendedEquipment::EleriumChip) ||
                    !other_items.iter().any(|item| item.is_gen())
            }
            ExtendedEquipment::DilithiumChip => {
                other_items.contains(&ExtendedEquipment::DilithiumChip) ||
                    !other_items.iter().any(|item| item.is_gen())
            }
            _ => true,
        }
    }

    fn is_gen(&self) -> bool {
        match *self {
            ExtendedEquipment::ThuliumGen |
            ExtendedEquipment::PlutoniumGen |
            ExtendedEquipment::StrontiumGen |
            ExtendedEquipment::PromethiumGen |
            ExtendedEquipment::RutheniumGen |
            ExtendedEquipment::EleriumGen |
            ExtendedEquipment::DilithiumGen => true,
            _ => false,
        }
    }
}


struct Picker<'set, T: 'set>(Box<Iterator<Item = (&'set T, Option<&'set T>)> + 'set>)
where
    T: Eq;

impl<'set, T> Picker<'set, T>
where
    T: Eq,
{
    fn new(set: &'set BTreeSet<T>) -> Picker<T> {
        Picker(Box::new(set.iter().map(|thing| (thing, None)).chain(
            set.iter().tuple_combinations().map(|(first, second)| {
                (first, Some(second))
            }),
        )))
    }
}

impl<'set, T> Iterator for Picker<'set, T>
where
    T: Eq,
{
    type Item = (&'set T, Option<&'set T>);

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}



#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct State<T>
where
    T: Eq + IsShielded + Clone + Copy + Ord,
{
    floors: Vec<BTreeSet<T>>,
    curr_floor: usize,
}

impl<T> State<T>
where
    T: Eq + IsShielded + Clone + Copy + Ord,
{
    fn new(num_floors: usize) -> State<T> {
        let mut floors = Vec::new();
        for _ in 0..num_floors {
            floors.push(BTreeSet::new());
        }

        State {
            floors: floors,
            curr_floor: 0,
        }
    }

    fn insert_item(&mut self, at_floor: usize, item: T) {
        if !self.floors[at_floor].insert(item) {
            panic!("Duplicate insertion of item into floor")
        }
    }

    fn remove_item(&mut self, from_floor: usize, item: &T) {
        if !self.floors[from_floor].remove(&item) {
            panic!("Item not present in floor")
        }
    }

    fn is_valid(&self) -> bool {
        self.floors.iter().all(|floor| floor.iter().all(|item| item.is_shielded(floor)))
    }

    fn is_final_solution(&self) -> bool {
        for floor_id in 0..(self.floors.len() - 1) {
            if self.floors[floor_id].len() != 0 {
                return false;
            }
        }

        if self.floors[self.floors.len() - 1].len() != 0 {
            return true;
        } else {
            panic!("All floors are empty");
        }
    }

    fn expand_state(&self, states_seen: &mut BTreeSet<State<T>>) -> Vec<State<T>> {

        let mut new_states = Vec::new();

        // Can move up
        if self.curr_floor < self.floors.len() - 1 {
            for choice in Picker::new(&self.floors[self.curr_floor]) {
                let mut new_state = self.clone();

                let upper_floor = self.curr_floor + 1;

                new_state.remove_item(self.curr_floor, choice.0);
                new_state.insert_item(upper_floor, *choice.0);

                if let Some(item) = choice.1 {
                    new_state.remove_item(self.curr_floor, item);
                    new_state.insert_item(upper_floor, *item);
                }

                new_state.curr_floor = upper_floor;

                if new_state.is_valid() && !states_seen.contains(&new_state) {
                    states_seen.insert(new_state.clone());
                    new_states.push(new_state);
                }
            }
        }

        // Can move down
        if self.curr_floor > 0 {
            for choice in Picker::new(&self.floors[self.curr_floor]) {
                let mut new_state = self.clone();

                let lower_floor = self.curr_floor - 1;

                new_state.remove_item(self.curr_floor, choice.0);
                new_state.insert_item(lower_floor, *choice.0);

                if let Some(item) = choice.1 {
                    new_state.remove_item(self.curr_floor, item);
                    new_state.insert_item(lower_floor, *item);
                }

                new_state.curr_floor = lower_floor;

                if new_state.is_valid() && !states_seen.contains(&new_state) {
                    states_seen.insert(new_state.clone());
                    new_states.push(new_state);
                }
            }
        }

        new_states
    }
}

fn find_solution<T>(initial_state: State<T>) -> usize
where
    T: Eq + IsShielded + Clone + Copy + Ord,
{
    let mut states_seen = BTreeSet::new();

    let mut open_states = vec![initial_state];
    let mut new_states = Vec::new();
    let mut depth = 0;

    loop {
        if let Some(curr_state) = open_states.pop() {
            if curr_state.is_final_solution() {
                break;
            }

            new_states.extend(curr_state.expand_state(&mut states_seen));
        } else {
            open_states = new_states;
            new_states = Vec::new();
            depth += 1;

            if open_states.is_empty() {
                panic!("no open states");
            }
        }
    }

    depth
}


fn main() {

    let mut state = State::new(4);

    state.insert_item(0, Equipment::ThuliumGen);
    state.insert_item(0, Equipment::ThuliumChip);
    state.insert_item(0, Equipment::PlutoniumGen);
    state.insert_item(0, Equipment::StrontiumGen);

    state.insert_item(1, Equipment::PlutoniumChip);
    state.insert_item(1, Equipment::StrontiumChip);

    state.insert_item(2, Equipment::PromethiumGen);
    state.insert_item(2, Equipment::PromethiumChip);
    state.insert_item(2, Equipment::RutheniumGen);
    state.insert_item(2, Equipment::RutheniumChip);

    println!(
        "Limited equipment - Final state reached at depth: {}",
        find_solution(state)
    );

    let mut state_b = State::new(4);

    state_b.insert_item(0, ExtendedEquipment::ThuliumGen);
    state_b.insert_item(0, ExtendedEquipment::ThuliumChip);
    state_b.insert_item(0, ExtendedEquipment::PlutoniumGen);
    state_b.insert_item(0, ExtendedEquipment::StrontiumGen);
    state_b.insert_item(0, ExtendedEquipment::EleriumGen);
    state_b.insert_item(0, ExtendedEquipment::EleriumChip);
    state_b.insert_item(0, ExtendedEquipment::DilithiumGen);
    state_b.insert_item(0, ExtendedEquipment::DilithiumChip);

    state_b.insert_item(1, ExtendedEquipment::PlutoniumChip);
    state_b.insert_item(1, ExtendedEquipment::StrontiumChip);

    state_b.insert_item(2, ExtendedEquipment::PromethiumGen);
    state_b.insert_item(2, ExtendedEquipment::PromethiumChip);
    state_b.insert_item(2, ExtendedEquipment::RutheniumGen);
    state_b.insert_item(2, ExtendedEquipment::RutheniumChip);

    println!(
        "Extended equipment - Final state reached at depth: {}",
        find_solution(state_b)
    );
}


#[cfg(test)]
mod tests {
    use super::IsShielded;
    use super::State;
    use super::find_solution;

    use std::collections::BTreeSet;

    #[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
    enum TestEquipment {
        HydrogenGen,
        HydrogenChip,
        LithiumGen,
        LithiumChip,
    }

    impl IsShielded for TestEquipment {
        fn is_shielded(&self, other_items: &BTreeSet<TestEquipment>) -> bool {
            match *self {
                TestEquipment::HydrogenChip => {
                    other_items.contains(&TestEquipment::HydrogenGen) ||
                        !other_items.iter().any(|item| item.is_gen())
                }
                TestEquipment::LithiumChip => {
                    other_items.contains(&TestEquipment::LithiumGen) ||
                        !other_items.iter().any(|item| item.is_gen())
                }
                _ => true,
            }
        }

        fn is_gen(&self) -> bool {
            match *self {
                TestEquipment::HydrogenGen |
                TestEquipment::LithiumGen => true,
                _ => false,
            }
        }
    }

    #[test]
    fn find_solution_test() {
        let mut initial_state = State::new(4);

        initial_state.insert_item(0, TestEquipment::HydrogenChip);
        initial_state.insert_item(0, TestEquipment::LithiumChip);

        initial_state.insert_item(1, TestEquipment::HydrogenGen);

        initial_state.insert_item(2, TestEquipment::LithiumGen);

        assert_eq!(11, find_solution(initial_state));
    }
}
