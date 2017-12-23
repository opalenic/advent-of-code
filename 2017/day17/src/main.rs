
use std::env;

struct Spinlock {
    buffer: Vec<usize>,
}

impl Spinlock {
    fn new(step_size: usize, num_insertions: usize) -> Spinlock {
        let mut buffer = Vec::with_capacity(num_insertions + 1);
        buffer.push(0);

        let mut curr_pos = 0;

        for i in 1..(num_insertions + 1) {
            curr_pos = (curr_pos + step_size) % buffer.len() + 1;

            buffer.insert(curr_pos, i);

            if i % 10_000 == 0 {
                println!("*** {}", i);
            }
        }

        Spinlock { buffer }
    }

    fn get_value_after(&self, value: usize) -> Option<usize> {
        if let Some((idx, _)) = self.buffer.iter().enumerate().find(
            |&(_idx, val)| *val == value,
        )
        {
            let pos = (idx + 1) % self.buffer.len();

            self.buffer.iter().nth(pos).map(|val| *val)
        } else {
            None
        }
    }
}

fn get_spinlock_value_after_zero(step_size: usize, num_insertions: usize) -> Option<usize> {
    let mut val = None;
    let mut curr_pos = 0;
    let mut lock_len = 1;

    for i in 1..(num_insertions + 1) {
        curr_pos = (curr_pos + step_size) % lock_len + 1;

        if curr_pos == 1 {
            val = Some(i);
        }

        lock_len += 1;
    }

    val
}



fn main() {
    let mut args = env::args();
    args.next();

    let step_size = args.next().expect("missing argument").parse().expect(
        "parse error",
    );

    let lock = Spinlock::new(step_size, 2017);

    println!(
        "Short lock: The value right after 2017 is {}.",
        lock.get_value_after(2017).expect(
            "2017 missing in spinlock",
        )
    );

    println!(
        "Long lock: The value right after 0 is {}.",
        get_spinlock_value_after_zero(step_size, 50_000_000).unwrap()
    );
}


#[cfg(test)]
mod tests {
    use super::Spinlock;
    use super::get_spinlock_value_after_zero;

    #[test]
    fn spinlock_test() {
        let lock = Spinlock::new(3, 3);

        assert_eq!(vec![0, 2, 3, 1], lock.buffer);

        let lock = Spinlock::new(3, 2017);

        println!("{:?}", lock.buffer);

        assert_eq!(Some(638), lock.get_value_after(2017));
    }

    #[test]
    fn spinlock_special_case_test() {
        assert_eq!(Some(1226), get_spinlock_value_after_zero(3, 2017));
    }
}
