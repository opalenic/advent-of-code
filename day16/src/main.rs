
extern crate regex;

use regex::Regex;

use std::io;
use std::io::prelude::*;

use std::collections::BTreeMap;

#[derive(Debug)]
struct Sue {
    children: Option<u32>,
    cats: Option<u32>,
    samoyeds: Option<u32>,
    pomeranians: Option<u32>,
    akitas: Option<u32>,
    vizslas: Option<u32>,
    goldfish: Option<u32>,
    trees: Option<u32>,
    cars: Option<u32>,
    perfumes: Option<u32>,
}

impl Sue {
    fn from_str(s: &str) -> (u32, Sue) {
        let mut children = None;
        let mut cats = None;
        let mut samoyeds = None;
        let mut pomeranians = None;
        let mut akitas = None;
        let mut vizslas = None;
        let mut goldfish = None;
        let mut trees = None;
        let mut cars = None;
        let mut perfumes = None;

        let re1 = Regex::new("^Sue (?P<id>[:digit:]+): (?P<rest>.*)$").unwrap();

        let cap1 = re1.captures(s).unwrap();
        let id = cap1.name("id").unwrap().parse::<u32>().unwrap();

        let animal_list = cap1.name("rest").unwrap().split(", ");
        let re2 = Regex::new("^(?P<name>[:alpha:]+): (?P<val>[:digit:]+)$").unwrap();

        for animal in animal_list {
            let cap2 = re2.captures(animal).unwrap();

            let val = cap2.name("val").unwrap().parse::<u32>().unwrap();
            let name = cap2.name("name").unwrap();

            match name {
                "children" => children = Some(val),
                "cats" => cats = Some(val),
                "samoyeds" => samoyeds = Some(val),
                "pomeranians" => pomeranians = Some(val),
                "akitas" => akitas = Some(val),
                "vizslas" => vizslas = Some(val),
                "goldfish" => goldfish = Some(val),
                "trees" => trees = Some(val),
                "cars" => cars = Some(val),
                "perfumes" => perfumes = Some(val),
                _ => {
                    panic!("Unknown animal.");
                }
            }
        }

        (id,
         Sue {
            children: children,
            cats: cats,
            samoyeds: samoyeds,
            pomeranians: pomeranians,
            akitas: akitas,
            vizslas: vizslas,
            goldfish: goldfish,
            trees: trees,
            cars: cars,
            perfumes: perfumes,
        })
    }

    fn matches_reference(&self, ref_sue: &Sue) -> bool {
        if let Some(children) = self.children {
            if let Some(ref_children) = ref_sue.children {
                if children != ref_children {
                    return false;
                }
            }
        }

        if let Some(cats) = self.cats {
            if let Some(ref_cats) = ref_sue.cats {
                if cats != ref_cats {
                    return false;
                }
            }
        }

        if let Some(samoyeds) = self.samoyeds {
            if let Some(ref_samoyeds) = ref_sue.samoyeds {
                if samoyeds != ref_samoyeds {
                    return false;
                }
            }
        }

        if let Some(pomeranians) = self.pomeranians {
            if let Some(ref_pomeranians) = ref_sue.pomeranians {
                if pomeranians != ref_pomeranians {
                    return false;
                }
            }
        }

        if let Some(akitas) = self.akitas {
            if let Some(ref_akitas) = ref_sue.akitas {
                if akitas != ref_akitas {
                    return false;
                }
            }
        }

        if let Some(vizslas) = self.vizslas {
            if let Some(ref_vizslas) = ref_sue.vizslas {
                if vizslas != ref_vizslas {
                    return false;
                }
            }
        }

        if let Some(goldfish) = self.goldfish {
            if let Some(ref_goldfish) = ref_sue.goldfish {
                if goldfish != ref_goldfish {
                    return false;
                }
            }
        }

        if let Some(trees) = self.trees {
            if let Some(ref_trees) = ref_sue.trees {
                if trees != ref_trees {
                    return false;
                }
            }
        }

        if let Some(cars) = self.cars {
            if let Some(ref_cars) = ref_sue.cars {
                if cars != ref_cars {
                    return false;
                }
            }
        }

        if let Some(perfumes) = self.perfumes {
            if let Some(ref_perfumes) = ref_sue.perfumes {
                if perfumes != ref_perfumes {
                    return false;
                }
            }
        }

        true
    }

    fn matches_reference_recalibrated(&self, ref_sue: &Sue) -> bool {
        if let Some(children) = self.children {
            if let Some(ref_children) = ref_sue.children {
                if children != ref_children {
                    return false;
                }
            }
        }

        if let Some(cats) = self.cats {
            if let Some(ref_cats) = ref_sue.cats {
                if cats <= ref_cats {
                    return false;
                }
            }
        }

        if let Some(samoyeds) = self.samoyeds {
            if let Some(ref_samoyeds) = ref_sue.samoyeds {
                if samoyeds != ref_samoyeds {
                    return false;
                }
            }
        }

        if let Some(pomeranians) = self.pomeranians {
            if let Some(ref_pomeranians) = ref_sue.pomeranians {
                if pomeranians >= ref_pomeranians {
                    return false;
                }
            }
        }

        if let Some(akitas) = self.akitas {
            if let Some(ref_akitas) = ref_sue.akitas {
                if akitas != ref_akitas {
                    return false;
                }
            }
        }

        if let Some(vizslas) = self.vizslas {
            if let Some(ref_vizslas) = ref_sue.vizslas {
                if vizslas != ref_vizslas {
                    return false;
                }
            }
        }

        if let Some(goldfish) = self.goldfish {
            if let Some(ref_goldfish) = ref_sue.goldfish {
                if goldfish >= ref_goldfish {
                    return false;
                }
            }
        }

        if let Some(trees) = self.trees {
            if let Some(ref_trees) = ref_sue.trees {
                if trees <= ref_trees {
                    return false;
                }
            }
        }

        if let Some(cars) = self.cars {
            if let Some(ref_cars) = ref_sue.cars {
                if cars != ref_cars {
                    return false;
                }
            }
        }

        if let Some(perfumes) = self.perfumes {
            if let Some(ref_perfumes) = ref_sue.perfumes {
                if perfumes != ref_perfumes {
                    return false;
                }
            }
        }

        true
    }
}


fn main() {
    let mut sues = BTreeMap::new();

    let stdin = io::stdin();

    for wline in stdin.lock().lines() {
        let line = wline.unwrap();

        let (id, sue) = Sue::from_str(&line);

        sues.insert(id, sue);
    }

    let ref_sue = Sue {
        children: Some(3),
        cats: Some(7),
        samoyeds: Some(2),
        pomeranians: Some(3),
        akitas: Some(0),
        vizslas: Some(0),
        goldfish: Some(5),
        trees: Some(3),
        cars: Some(2),
        perfumes: Some(1),
    };

    println!("Looking for the following Sue: {:?}", ref_sue);

    for (id, sue) in &sues {
        if sue.matches_reference(&ref_sue) {
            println!("Possible match with Sue #{}: {:?}", id, sue);
        }
    }

    println!("-------------------------------------------------");
    println!("Recalibrating.");

    println!("Looking for the following Sue: {:?}", ref_sue);

    for (id, sue) in &sues {
        if sue.matches_reference_recalibrated(&ref_sue) {
            println!("Possible match with Sue #{}: {:?}", id, sue);
        }
    }
}
