
extern crate regex;

#[macro_use]
extern crate lazy_static;


use std::io;
use std::io::prelude::*;

use std::collections::HashMap;

use std::str::FromStr;

use std::fmt;

use regex::Regex;


lazy_static! {
    static ref COMPONENT_RE: Regex = Regex::new(r"^([0-9]+)/([0-9]+)$").unwrap();
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
struct Component(usize, usize);


impl FromStr for Component {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let caps = COMPONENT_RE.captures(s).ok_or(())?;

        let l_val = caps.get(1).ok_or(())?.as_str().parse().map_err(|_| ())?;
        let r_val = caps.get(2).ok_or(())?.as_str().parse().map_err(|_| ())?;

        Ok(Component(l_val, r_val))
    }
}

impl Component {
    fn is_compatible(&self, port_val: usize) -> Option<usize> {
        if self.0 == port_val {
            Some(self.1)
        } else if self.1 == port_val {
            Some(self.0)
        } else {
            None
        }
    }

    fn get_strength(&self) -> usize {
        self.0 + self.1
    }
}

impl fmt::Display for Component {
    fn fmt(&self, w: &mut fmt::Formatter) -> fmt::Result {
        write!(w, "{}/{}", self.0, self.1)
    }
}




#[derive(Debug, PartialEq, Eq, Clone)]
struct Components(HashMap<Component, usize>);

impl Components {
    fn new() -> Components {
        Components(HashMap::new())
    }

    fn insert(&mut self, component: Component) {
        *self.0.entry(component).or_insert(0) += 1;
    }

    fn remove(&mut self, component: &Component) {
        if let Some(comp_cnt) = self.0.remove(component) {
            if comp_cnt > 1 {
                self.0.insert(*component, comp_cnt - 1);
            }
        }
    }

    fn iter(&self) -> std::collections::hash_map::Keys<Component, usize> {
        self.0.keys()
    }
}

impl FromStr for Components {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut components = Components::new();

        for line in s.lines() {
            let c = line.parse()?;

            components.insert(c);
        }

        Ok(components)
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Bridge {
    components: Vec<Component>,
    last_port: usize,
}

impl Bridge {
    fn new() -> Bridge {
        Bridge {
            components: Vec::new(),
            last_port: 0,
        }
    }

    fn expand_bridge(&self, available_comps: &Components) -> Vec<Bridge> {
        let mut out = Vec::new();

        for (valid_comp, other_port) in
            available_comps.iter().filter_map(
                |comp| if let Some(other_port) =
                    comp.is_compatible(self.last_port)
                {
                    Some((comp, other_port))
                } else {
                    None
                },
            )
        {
            let mut new_comps = available_comps.clone();
            new_comps.remove(valid_comp);

            let mut new_bridge = self.clone();
            new_bridge.components.push(*valid_comp);
            new_bridge.last_port = other_port;

            out.extend(new_bridge.expand_bridge(&new_comps));
            out.push(new_bridge);
        }

        out
    }

    fn get_strength(&self) -> usize {
        self.components.iter().map(|comp| comp.get_strength()).sum()
    }

    fn len(&self) -> usize {
        self.components.len()
    }

    fn get_valid_bridges(components: &Components) -> Vec<Bridge> {
        Bridge::new().expand_bridge(components)
    }
}

impl fmt::Display for Bridge {
    fn fmt(&self, w: &mut fmt::Formatter) -> fmt::Result {
        let mut comp_iter = self.components.iter().peekable();

        while let Some(comp) = comp_iter.next() {
            write!(w, "{}", comp)?;

            if comp_iter.peek().is_some() {
                write!(w, "--")?;
            }
        }

        Ok(())
    }
}


fn get_strongest_bridge(bridges: &Vec<Bridge>) -> Option<(&Bridge, usize)> {
    bridges.iter().map(|b| (b, b.get_strength())).max_by(
        |&(_, s_1),
         &(_, s_2)| {
            s_1.cmp(&s_2)
        },
    )
}

fn get_strongest_longest_bridge(bridges: &Vec<Bridge>) -> Option<(&Bridge, usize)> {
    bridges
        .iter()
        .map(|b| (b, b.len(), b.get_strength()))
        .max_by(|&(_, l_1, s_1), &(_, l_2, s_2)| {
            l_1.cmp(&l_2).then(s_1.cmp(&s_2))
        })
        .map(|(b, _len, strength)| (b, strength))
}


fn main() {
    let mut input_str = String::new();
    io::stdin().read_to_string(&mut input_str).unwrap();

    let components = input_str.parse().expect("parse error");

    let bridges = Bridge::get_valid_bridges(&components);

    let (strongest_bridge, max_strength) = get_strongest_bridge(&bridges).expect("no bridges");

    println!("The bridge with the maximum score is:");
    println!("{}", strongest_bridge);
    println!("Bridge score: {}", max_strength);

    println!();

    let (longest_bridge, longest_str) = get_strongest_longest_bridge(&bridges).expect("no bridges");

    println!("The longest possible bridge with the maximum score is:");
    println!("{}", longest_bridge);
    println!("Bridge score: {}", longest_str);
}



#[cfg(test)]
mod tests {
    use super::Component;
    use super::Components;
    use super::Bridge;
    use super::get_strongest_bridge;
    use super::get_strongest_longest_bridge;

    use std::collections::HashSet;

    const TEST_STR: &str = "0/2\n\
                            2/2\n\
                            2/3\n\
                            3/4\n\
                            3/5\n\
                            0/1\n\
                            10/1\n\
                            9/10\n";

    #[test]
    fn parse_test() {
        let components: Components = TEST_STR.parse().expect("parse error");

        let mut expected = Components::new();
        expected.insert(Component(0, 2));
        expected.insert(Component(2, 2));
        expected.insert(Component(2, 3));
        expected.insert(Component(3, 4));
        expected.insert(Component(3, 5));
        expected.insert(Component(0, 1));
        expected.insert(Component(10, 1));
        expected.insert(Component(9, 10));

        assert_eq!(expected, components);
    }

    #[test]
    fn expand_test() {
        let components: Components = TEST_STR.parse().expect("parse error");

        let mut expected_bridges = HashSet::new();

        let mut b1 = Bridge::new();
        b1.components.push(Component(0, 1));
        b1.last_port = 1;
        expected_bridges.insert(b1);

        let mut b2 = Bridge::new();
        b2.components.push(Component(0, 1));
        b2.components.push(Component(10, 1));
        b2.last_port = 10;
        expected_bridges.insert(b2);

        let mut b3 = Bridge::new();
        b3.components.push(Component(0, 1));
        b3.components.push(Component(10, 1));
        b3.components.push(Component(9, 10));
        b3.last_port = 9;
        expected_bridges.insert(b3);

        let mut b4 = Bridge::new();
        b4.components.push(Component(0, 2));
        b4.last_port = 2;
        expected_bridges.insert(b4);

        let mut b5 = Bridge::new();
        b5.components.push(Component(0, 2));
        b5.components.push(Component(2, 3));
        b5.last_port = 3;
        expected_bridges.insert(b5);

        let mut b6 = Bridge::new();
        b6.components.push(Component(0, 2));
        b6.components.push(Component(2, 3));
        b6.components.push(Component(3, 4));
        b6.last_port = 4;
        expected_bridges.insert(b6);

        let mut b7 = Bridge::new();
        b7.components.push(Component(0, 2));
        b7.components.push(Component(2, 3));
        b7.components.push(Component(3, 5));
        b7.last_port = 5;
        expected_bridges.insert(b7);

        let mut b8 = Bridge::new();
        b8.components.push(Component(0, 2));
        b8.components.push(Component(2, 2));
        b8.last_port = 2;
        expected_bridges.insert(b8);

        let mut b9 = Bridge::new();
        b9.components.push(Component(0, 2));
        b9.components.push(Component(2, 2));
        b9.components.push(Component(2, 3));
        b9.last_port = 3;
        expected_bridges.insert(b9);

        let mut b10 = Bridge::new();
        b10.components.push(Component(0, 2));
        b10.components.push(Component(2, 2));
        b10.components.push(Component(2, 3));
        b10.components.push(Component(3, 4));
        b10.last_port = 4;
        expected_bridges.insert(b10);

        let mut b11 = Bridge::new();
        b11.components.push(Component(0, 2));
        b11.components.push(Component(2, 2));
        b11.components.push(Component(2, 3));
        b11.components.push(Component(3, 5));
        b11.last_port = 5;
        expected_bridges.insert(b11);


        let bridge_vec = Bridge::get_valid_bridges(&components);
        let bridge_hash = bridge_vec.into_iter().collect::<HashSet<Bridge>>();

        assert_eq!(expected_bridges, bridge_hash);
    }

    #[test]
    fn strongest_bridge_test() {
        let components: Components = TEST_STR.parse().expect("parse error");
        let bridges = Bridge::get_valid_bridges(&components);

        let mut expected_max_bridge = Bridge::new();
        expected_max_bridge.components.push(Component(0, 1));
        expected_max_bridge.components.push(Component(10, 1));
        expected_max_bridge.components.push(Component(9, 10));
        expected_max_bridge.last_port = 9;


        let (max_bridge, strength) = get_strongest_bridge(&bridges).expect("no bridges");

        assert_eq!(&expected_max_bridge, max_bridge);
        assert_eq!(31, strength);
    }

    #[test]
    fn longest_strongest_bridge_test() {
        let components: Components = TEST_STR.parse().expect("parse error");
        let bridges = Bridge::get_valid_bridges(&components);

        let mut expected_max_bridge = Bridge::new();
        expected_max_bridge.components.push(Component(0, 2));
        expected_max_bridge.components.push(Component(2, 2));
        expected_max_bridge.components.push(Component(2, 3));
        expected_max_bridge.components.push(Component(3, 5));
        expected_max_bridge.last_port = 5;


        let (max_bridge, strength) = get_strongest_longest_bridge(&bridges).expect("no bridges");

        assert_eq!(&expected_max_bridge, max_bridge);
        assert_eq!(19, strength);
    }
}
