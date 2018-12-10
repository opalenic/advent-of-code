use std::env;

use std::fmt;
use std::fmt::Display;

#[derive(Debug, Clone, Copy)]
struct Marble {
    prev_idx: usize,
    next_idx: usize,
    val: usize,
}

#[derive(Debug)]
struct Marbles {
    all_marbles: Vec<Marble>,
    curr_marble_idx: usize,
}

impl Marbles {
    fn new() -> Marbles {
        let all_marbles = vec![Marble {
            prev_idx: 0,
            next_idx: 0,
            val: 0,
        }];

        Marbles {
            all_marbles,
            curr_marble_idx: 0,
        }
    }

    fn shift_by(&mut self, offset: isize) {
        if offset >= 0 {
            for _ in 0..offset {
                self.curr_marble_idx = self.all_marbles[self.curr_marble_idx].next_idx;
            }
        } else {
            for _ in 0..(offset.abs()) {
                self.curr_marble_idx = self.all_marbles[self.curr_marble_idx].prev_idx;
            }
        }
    }

    fn remove_current(&mut self) -> usize {
        let curr_marble = self.all_marbles[self.curr_marble_idx];

        let next = curr_marble.next_idx;
        let prev = curr_marble.prev_idx;

        self.all_marbles[next].prev_idx = prev;
        self.all_marbles[prev].next_idx = next;

        self.curr_marble_idx = curr_marble.next_idx;

        curr_marble.val
    }

    fn insert_at_curr_idx(&mut self, marble_val: usize) {
        let curr_marble = self.all_marbles[self.curr_marble_idx];

        self.all_marbles.push(Marble {
            prev_idx: self.curr_marble_idx,
            next_idx: curr_marble.next_idx,
            val: marble_val,
        });

        self.all_marbles[self.curr_marble_idx].next_idx = self.all_marbles.len() - 1;
        self.all_marbles[curr_marble.next_idx].prev_idx = self.all_marbles.len() - 1;

        if self.all_marbles[curr_marble.prev_idx].prev_idx == self.curr_marble_idx {
            self.all_marbles[curr_marble.prev_idx].prev_idx = self.all_marbles.len() - 1;
        }

        self.curr_marble_idx = self.all_marbles.len() - 1;
    }
}

impl Display for Marbles {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        let mut curr_idx = 0;

        loop {
            if curr_idx == self.curr_marble_idx {
                write!(formatter, "({}) ", self.all_marbles[curr_idx].val)?;
            } else {
                write!(formatter, "{} ", self.all_marbles[curr_idx].val)?;
            }

            curr_idx = self.all_marbles[curr_idx].next_idx;

            if curr_idx == 0 {
                break;
            }
        }

        writeln!(formatter)
    }
}

fn play_game(num_marbles: usize, num_players: usize) -> usize {
    let mut marbles = Marbles::new();
    let mut scores = vec![0; num_players];
    let mut curr_player = 0;

    for marble in 1..num_marbles {
        if marble % 23 == 0 {
            marbles.shift_by(-7);

            scores[curr_player] += marbles.remove_current();
            scores[curr_player] += marble;
        } else {
            marbles.shift_by(1);
            marbles.insert_at_curr_idx(marble);
        }

        curr_player = (curr_player + 1) % scores.len();
    }

    scores.into_iter().max().unwrap()
}

fn main() {
    let mut args = env::args();
    args.next();

    let num_players = args
        .next()
        .expect("missing argument")
        .parse()
        .expect("parse error");
    let max_marble = args
        .next()
        .expect("missing argument")
        .parse()
        .expect("parse error");

    println!(
        "When played with {} players, and {} marbles, the high score is {}.",
        num_players,
        max_marble,
        play_game(max_marble, num_players)
    );

    println!(
        "When played with {} players, and {} marbles, the high score is {}.",
        num_players,
        max_marble * 100,
        play_game(max_marble * 100, num_players)
    );
}

#[cfg(test)]
mod tests {
    use super::play_game;

    #[test]
    fn high_score_test() {
        assert_eq!(32, play_game(25, 9));
        assert_eq!(8317, play_game(1618, 10));
        assert_eq!(146373, play_game(7999, 13));
        // assert_eq!(2764, play_game(1104, 17));
        assert_eq!(2720, play_game(1104, 17));
        assert_eq!(54718, play_game(6111, 21));
        assert_eq!(37305, play_game(5807, 30));
    }
}
