
use std::env;

struct Generator {
    initial_val: usize,
    curr_val: usize,
    factor: usize,
    divisible_by: usize,
}

enum GeneratorType {
    A,
    B,
}

impl Generator {
    fn new(initial_val: usize, gen_type: GeneratorType) -> Generator {
        let factor = match gen_type {
            GeneratorType::A => 16807,
            GeneratorType::B => 48271,
        };

        let divisible_by = match gen_type {
            GeneratorType::A => 4,
            GeneratorType::B => 8,
        };

        Generator {
            initial_val: initial_val,
            curr_val: initial_val,
            factor: factor,
            divisible_by: divisible_by,
        }
    }

    fn next_val(&mut self) -> usize {
        self.curr_val = (self.curr_val * self.factor) % 2147483647;

        self.curr_val
    }

    fn next_val_picky(&mut self) -> usize {
        loop {
            let val = self.next_val();

            if val % self.divisible_by == 0 {
                return val;
            }
        }
    }

    fn reset(&mut self) {
        self.curr_val = self.initial_val;
    }
}

struct Judge {
    gen_a: Generator,
    gen_b: Generator,
}

impl Judge {
    fn new(gen_a: Generator, gen_b: Generator) -> Judge {
        Judge { gen_a, gen_b }
    }

    fn count_matches(&mut self, num_tests: usize) -> usize {
        let mut match_count = 0;

        for _ in 0..num_tests {
            let a = self.gen_a.next_val();
            let b = self.gen_b.next_val();

            if (a as u16) == (b as u16) {
                match_count += 1;
            }
        }

        match_count
    }

    fn count_matches_picky(&mut self, num_tests: usize) -> usize {
        let mut match_count = 0;

        for _ in 0..num_tests {
            let a = self.gen_a.next_val_picky();
            let b = self.gen_b.next_val_picky();

            if (a as u16) == (b as u16) {
                match_count += 1;
            }
        }

        match_count
    }

    fn reset(&mut self) {
        self.gen_a.reset();
        self.gen_b.reset();
    }
}

fn main() {
    let mut args = env::args();
    args.next();

    let factor_a = args.next().expect("missing param").parse().expect(
        "parse error",
    );
    let factor_b = args.next().expect("missing param").parse().expect(
        "parse error",
    );


    let gen_a = Generator::new(factor_a, GeneratorType::A);
    let gen_b = Generator::new(factor_b, GeneratorType::B);

    let mut judge = Judge::new(gen_a, gen_b);
    println!(
        "The number of matches after 40 000 000 iterations is {}.",
        judge.count_matches(40_000_000)
    );

    judge.reset();
    println!(
        "The number of matches after 5 000 000 iterations while using the picky algorithm is {}.",
        judge.count_matches_picky(5_000_000)
    );
}


#[cfg(test)]
mod tests {
    use super::Generator;
    use super::GeneratorType;
    use super::Judge;

    #[test]
    fn generator_test() {
        let expected_vals_a = [1092455, 1181022009, 245556042, 1744312007, 1352636452];
        let mut gen_a = Generator::new(65, GeneratorType::A);

        for expected in &expected_vals_a {
            assert_eq!(*expected, gen_a.next_val());
        }


        let expected_vals_b = [430625591, 1233683848, 1431495498, 137874439, 285222916];
        let mut gen_b = Generator::new(8921, GeneratorType::B);

        for expected in &expected_vals_b {
            assert_eq!(*expected, gen_b.next_val());
        }
    }

    #[test]
    fn picky_generator_test() {
        let expected_vals_a = [1352636452, 1992081072, 530830436, 1980017072, 740335192];
        let mut gen_a = Generator::new(65, GeneratorType::A);

        for expected in &expected_vals_a {
            assert_eq!(*expected, gen_a.next_val_picky());
        }


        let expected_vals_b = [1233683848, 862516352, 1159784568, 1616057672, 412269392];
        let mut gen_b = Generator::new(8921, GeneratorType::B);

        for expected in &expected_vals_b {
            assert_eq!(*expected, gen_b.next_val_picky());
        }

    }

    #[test]
    fn judge_test() {
        let gen_a = Generator::new(65, GeneratorType::A);
        let gen_b = Generator::new(8921, GeneratorType::B);

        let mut judge = Judge::new(gen_a, gen_b);

        assert_eq!(588, judge.count_matches(40_000_000));
    }

    #[test]
    fn picky_judge_test() {
        let gen_a = Generator::new(65, GeneratorType::A);
        let gen_b = Generator::new(8921, GeneratorType::B);

        let mut judge = Judge::new(gen_a, gen_b);

        assert_eq!(309, judge.count_matches_picky(5_000_000));
    }

}
