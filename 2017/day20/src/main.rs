
extern crate regex;

#[macro_use]
extern crate lazy_static;

use std::io;
use std::io::prelude::*;

use std::str::FromStr;


use regex::Regex;

lazy_static! {
    static ref PARTICLE_RE: Regex = Regex::new(r"^p=<(?P<pos_x>-?[0-9]+),(?P<pos_y>-?[0-9]+),(?P<pos_z>-?[0-9]+)>, v=<(?P<vel_x>-?[0-9]+),(?P<vel_y>-?[0-9]+),(?P<vel_z>-?[0-9]+)>, a=<(?P<acc_x>-?[0-9]+),(?P<acc_y>-?[0-9]+),(?P<acc_z>-?[0-9]+)>$").unwrap();
}

#[derive(Debug, Copy, Clone)]
struct Particle {
    pos: (f64, f64, f64),
    vel: (f64, f64, f64),
    acc: (f64, f64, f64),
}

impl FromStr for Particle {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let caps = PARTICLE_RE.captures(s).ok_or(())?;

        let pos_x = caps.name("pos_x").ok_or(())?.as_str().parse().map_err(
            |_| (),
        )?;
        let pos_y = caps.name("pos_y").ok_or(())?.as_str().parse().map_err(
            |_| (),
        )?;
        let pos_z = caps.name("pos_z").ok_or(())?.as_str().parse().map_err(
            |_| (),
        )?;

        let vel_x = caps.name("vel_x").ok_or(())?.as_str().parse().map_err(
            |_| (),
        )?;
        let vel_y = caps.name("vel_y").ok_or(())?.as_str().parse().map_err(
            |_| (),
        )?;
        let vel_z = caps.name("vel_z").ok_or(())?.as_str().parse().map_err(
            |_| (),
        )?;

        let acc_x = caps.name("acc_x").ok_or(())?.as_str().parse().map_err(
            |_| (),
        )?;
        let acc_y = caps.name("acc_y").ok_or(())?.as_str().parse().map_err(
            |_| (),
        )?;
        let acc_z = caps.name("acc_z").ok_or(())?.as_str().parse().map_err(
            |_| (),
        )?;

        Ok(Particle {
            pos: (pos_x, pos_y, pos_z),
            vel: (vel_x, vel_y, vel_z),
            acc: (acc_x, acc_y, acc_z),
        })
    }
}

#[derive(Debug)]
struct Simulation(Vec<Particle>);

impl FromStr for Simulation {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Simulation(s.lines()
            .map(|line| line.parse())
            .collect::<Result<Vec<Particle>, ()>>()?))
    }
}


impl Simulation {
    fn do_step(&mut self) {
        for particle in self.0.iter_mut() {
            particle.vel.0 += particle.acc.0;
            particle.vel.1 += particle.acc.1;
            particle.vel.2 += particle.acc.2;

            particle.pos.0 += particle.vel.0;
            particle.pos.1 += particle.vel.1;
            particle.pos.2 += particle.vel.2;
        }


        self.0 = self.0
            .iter()
            .filter(|particle_1| {

                let occurrence_count = self.0.iter().fold(
                    0,
                    |acc, particle_2| if particle_1.pos ==
                        particle_2.pos
                    {
                        acc + 1
                    } else {
                        acc
                    },
                );

                occurrence_count <= 1
            })
            .cloned()
            .collect();
    }

    fn get_longterm_closest_particle(&self) -> (usize, &Particle) {
        let (idx, part, _) = self.0
            .iter()
            .enumerate()
            .map(|(idx, part)| {
                let norm = get_norm(&part.acc);

                (idx, part, norm)
            })
            .min_by(|&(_, _, l_norm), &(_, _, r_norm)| {
                l_norm.partial_cmp(&r_norm).unwrap()
            })
            .unwrap();

        (idx, part)
    }

    fn get_num_particles(&self) -> usize {
        self.0.len()
    }
}


fn dot_product(left: &(f64, f64, f64), right: &(f64, f64, f64)) -> f64 {
    left.0 * right.0 + left.1 * right.1 + left.2 * right.2
}

fn get_norm(vec: &(f64, f64, f64)) -> f64 {
    dot_product(&vec, &vec).sqrt()
}

fn main() {
    let mut input_str = String::new();
    io::stdin().read_to_string(&mut input_str).expect(
        "input error",
    );

    let mut sim: Simulation = input_str.parse().expect("parse error");

    println!(
        "The longterm closest particle is: {:?}",
        sim.get_longterm_closest_particle()
    );


    println!("Running 100000 steps in the simulation");
    for i in 0..100_000 {
        if i % 10_000 == 0 {
            println!("*** {}", i);
        }

        sim.do_step();
    }

    println!(
        "There are {} particles left after 100000 steps.",
        sim.get_num_particles()
    );
}


#[cfg(test)]
mod tests {
    use super::Simulation;

    #[test]
    fn simulation_test() {
        let test_str = "p=<3,0,0>, v=<2,0,0>, a=<-1,0,0>\n\
                        p=<4,0,0>, v=<0,0,0>, a=<-2,0,0>";

        let sim: Simulation = test_str.parse().unwrap();

        assert_eq!(0, sim.get_longterm_closest_particle().0);
    }

    #[test]
    fn crash_test() {
        let test_str = "p=<-6,0,0>, v=<3,0,0>, a=<0,0,0>\n\
                        p=<-4,0,0>, v=<2,0,0>, a=<0,0,0>\n\
                        p=<-2,0,0>, v=<1,0,0>, a=<0,0,0>\n\
                        p=<3,0,0>, v=<-1,0,0>, a=<0,0,0>";

        let mut sim: Simulation = test_str.parse().unwrap();
        assert_eq!(4, sim.get_num_particles());

        sim.do_step();
        assert_eq!(4, sim.get_num_particles());

        sim.do_step();
        assert_eq!(1, sim.get_num_particles());

        sim.do_step();
        assert_eq!(1, sim.get_num_particles());

        sim.do_step();
        assert_eq!(1, sim.get_num_particles());
    }
}
