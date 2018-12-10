use std::collections::HashMap;

use std::str::FromStr;

use std::ops::{Deref, DerefMut};

use std::io;
use std::io::prelude::*;

use regex::Regex;

use std::num::ParseIntError;

use lazy_static::lazy_static;

#[derive(Debug)]
struct AocError(String);

#[derive(Debug, PartialEq, Eq)]
struct Schedule(HashMap<usize, HashMap<u8, usize>>);

lazy_static! {
    static ref EVENTS_RE: Regex = Regex::new(r"^(?P<datetime>\[[0-9]{4}-[0-9]{2}-[0-9]{2} (?P<hour>[0-9]{2}):(?P<minute>[0-9]{2})\]) (?P<action>Guard #(?P<guard_id>[0-9]+) begins shift|falls asleep|wakes up)$").unwrap();
}

impl FromStr for Schedule {
    type Err = AocError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        #[derive(Debug)]
        enum Event<'a> {
            GuardStarts(&'a str, usize),
            FallsAsleep(&'a str, u8),
            WakesUp(&'a str, u8),
        }

        let mut event_strings = input.lines().collect::<Vec<&str>>();

        // Sort by date
        event_strings.sort();

        // Create a list of Events out of the input
        let mut events = Vec::new();
        for event_str in &event_strings {
            let caps = EVENTS_RE
                .captures(event_str)
                .ok_or_else(|| AocError(format!("Invalid input: {:?}", event_str)))?;

            let datetime_str = caps
                .name("datetime")
                .ok_or_else(|| {
                    AocError(format!("Missing date and time in input: {:?}", event_str))
                })?
                .as_str();

            let hour: u8 = caps
                .name("hour")
                .ok_or_else(|| AocError(format!("Missing hour in input: {:?}", event_str)))?
                .as_str()
                .parse()
                .map_err(|e: ParseIntError| AocError(e.to_string()))?;

            let minute = caps
                .name("minute")
                .ok_or_else(|| AocError(format!("Missing minute in input: {:?}", event_str)))?
                .as_str()
                .parse()
                .map_err(|e: ParseIntError| AocError(e.to_string()))?;

            let event = match caps
                .name("action")
                .ok_or_else(|| {
                    AocError(format!("Missing action string in input: {:?}", event_str))
                })?
                .as_str()
            {
                "falls asleep" => {
                    if hour == 0 {
                        Event::FallsAsleep(datetime_str, minute)
                    } else {
                        return Err(AocError(format!("The fall asleep event should only happen during the midnight hour: {:?}", event_str)));
                    }
                }
                "wakes up" => {
                    if hour == 0 {
                        Event::WakesUp(datetime_str, minute)
                    } else {
                        return Err(AocError(format!("The fall asleep event should only happen during the midnight hour: {:?}", event_str)));
                    }
                }
                _ => {
                    let guard_id = caps
                        .name("guard_id")
                        .ok_or_else(|| {
                            AocError(format!("Missing guard ID in input: {:?}", event_str))
                        })?
                        .as_str()
                        .parse()
                        .map_err(|e: ParseIntError| AocError(e.to_string()))?;

                    Event::GuardStarts(datetime_str, guard_id)
                }
            };

            events.push(event);
        }

        let mut schedule = Schedule(HashMap::new());

        let mut curr_guard = None;
        let mut prev_event = None;
        let mut fell_asleep_at = None;

        for event in events {
            if let Some(curr) = curr_guard {
                match event {
                    Event::GuardStarts(_, guard_id) => {
                        curr_guard = Some(guard_id);
                        fell_asleep_at = None;
                    }
                    Event::FallsAsleep(_, minute) => {
                        if let Some(ev @ Event::FallsAsleep(_, _)) = prev_event {
                            return Err(AocError(format!(
                                "Cannot have event '{:?}' after event '{:?}.",
                                event, ev
                            )));
                        }

                        fell_asleep_at = Some(minute);
                    }
                    Event::WakesUp(_, minute) => {
                        match prev_event {
                            Some(Event::GuardStarts(_, _)) | Some(Event::WakesUp(_, _)) | None => {
                                return Err(AocError(format!(
                                    "Cannot have event '{:?}' after event '{:?}.",
                                    event, prev_event
                                )));
                            }
                            _ => {}
                        }

                        if let Some(start_time) = fell_asleep_at {
                            if start_time >= minute {
                                return Err(AocError(format!(
                                    "Wakeup time before fall asleep time: '{:?}' -> '{:?}'",
                                    prev_event, event
                                )));
                            }

                            for min in start_time..minute {
                                *schedule
                                    .entry(curr)
                                    .or_insert_with(HashMap::new)
                                    .entry(min)
                                    .or_insert(0) += 1
                            }
                        } else {
                            return Err(AocError(format!(
                                "Wakeup without falling asleep at '{:?}'.",
                                event
                            )));
                        }
                    }
                }
            } else {
                match event {
                    Event::GuardStarts(_, guard_id) => {
                        curr_guard = Some(guard_id);
                        fell_asleep_at = None;
                    }
                    ev => {
                        return Err(AocError(format!("Invalid event at this time: {:?}", ev)));
                    }
                }
            }

            prev_event = Some(event);
        }

        Ok(schedule)
    }
}

impl Deref for Schedule {
    type Target = HashMap<usize, HashMap<u8, usize>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Schedule {
    fn deref_mut(&mut self) -> &mut HashMap<usize, HashMap<u8, usize>> {
        &mut self.0
    }
}

impl Schedule {
    fn find_sleepiest_guard_and_minute(&self) -> Result<(usize, u8), AocError> {
        let max_guard: (&usize, &HashMap<u8, usize>) = self
            .iter()
            .max_by(|(_, guard_a_sched), (_, guard_b_sched)| {
                guard_a_sched
                    .values()
                    .sum::<usize>()
                    .cmp(&guard_b_sched.values().sum::<usize>())
            })
            .ok_or_else(|| AocError("No schedule stored!".into()))?;

        let max_minute = max_guard
            .1
            .iter()
            .max_by(|(_, minute_a_cnt), (_, minute_b_cnt)| minute_a_cnt.cmp(minute_b_cnt))
            .ok_or_else(|| AocError(format!("Guard #{} never fell asleep!", max_guard.0)))?
            .0;

        Ok((*max_guard.0, *max_minute))
    }

    fn find_most_often_asleep_guard_and_minute(&self) -> Result<(usize, u8), AocError> {
        let max_guard: (&usize, &HashMap<u8, usize>) = self
            .iter()
            .max_by(|(_, guard_a_sched), (_, guard_b_sched)| {
                guard_a_sched
                    .values()
                    .max()
                    .cmp(&guard_b_sched.values().max())
            })
            .ok_or_else(|| AocError("No schedule stored!".into()))?;

        let max_minute = max_guard
            .1
            .iter()
            .max_by(|(_, minute_a_cnt), (_, minute_b_cnt)| minute_a_cnt.cmp(minute_b_cnt))
            .ok_or_else(|| AocError(format!("Guard #{} never fell asleep!", max_guard.0)))?
            .0;

        Ok((*max_guard.0, *max_minute))
    }
}

fn main() {
    let mut input_str = String::new();
    io::stdin().read_to_string(&mut input_str).unwrap();

    let schedule: Schedule = input_str.parse().expect("parsing error");

    let (max_guard, max_minute) = schedule
        .find_sleepiest_guard_and_minute()
        .expect("schedule search error");

    println!(
        "Guard #{} is the sleepiest, and has most often slept on minute {}. Multiplied {}.",
        max_guard,
        max_minute,
        max_guard * max_minute as usize
    );


    let (max_guard_2, max_minute_2) = schedule
        .find_most_often_asleep_guard_and_minute()
        .expect("schedule search error");

    println!(
        "Guard #{} is most frequently asleep, on minute {}. Multiplied {}.",
        max_guard_2,
        max_minute_2,
        max_guard_2 * max_minute_2 as usize
    );
}

#[cfg(test)]
mod tests {
    use super::Schedule;
    use lazy_static::lazy_static;
    use std::collections::HashMap;

    const INPUT_STR: &str = "[1518-11-01 00:00] Guard #10 begins shift\n\
                             [1518-11-01 00:05] falls asleep\n\
                             [1518-11-01 00:25] wakes up\n\
                             [1518-11-01 00:30] falls asleep\n\
                             [1518-11-01 00:55] wakes up\n\
                             [1518-11-01 23:58] Guard #99 begins shift\n\
                             [1518-11-02 00:40] falls asleep\n\
                             [1518-11-02 00:50] wakes up\n\
                             [1518-11-03 00:05] Guard #10 begins shift\n\
                             [1518-11-03 00:24] falls asleep\n\
                             [1518-11-03 00:29] wakes up\n\
                             [1518-11-04 00:02] Guard #99 begins shift\n\
                             [1518-11-04 00:36] falls asleep\n\
                             [1518-11-04 00:46] wakes up\n\
                             [1518-11-05 00:03] Guard #99 begins shift\n\
                             [1518-11-05 00:45] falls asleep\n\
                             [1518-11-05 00:55] wakes up";

    lazy_static! {
        static ref EXPECTED_SCHED: Schedule = {
            let mut sched = Schedule(HashMap::new());

            let mut guard_10 = HashMap::new();
            let mut guard_99 = HashMap::new();
            (5..25u8).for_each(|min| *guard_10.entry(min).or_insert(0) += 1);
            (30..55u8).for_each(|min| *guard_10.entry(min).or_insert(0) += 1);

            (40..50u8).for_each(|min| *guard_99.entry(min).or_insert(0) += 1);

            (24..29u8).for_each(|min| *guard_10.entry(min).or_insert(0) += 1);

            (36..46u8).for_each(|min| *guard_99.entry(min).or_insert(0) += 1);
            (45..55u8).for_each(|min| *guard_99.entry(min).or_insert(0) += 1);

            sched.insert(10, guard_10);
            sched.insert(99, guard_99);
            sched
        };
    }

    #[test]
    fn parse_test() {
        let schedule: Schedule = INPUT_STR.parse().unwrap();

        assert_eq!(*EXPECTED_SCHED, schedule);
    }

    #[test]
    fn max_guard_test() {
        let (guard_id, max_minute) = EXPECTED_SCHED.find_sleepiest_guard_and_minute().unwrap();

        assert_eq!(10, guard_id);
        assert_eq!(24, max_minute);

        let (guard_id, max_minute) = EXPECTED_SCHED
            .find_most_often_asleep_guard_and_minute()
            .unwrap();

        assert_eq!(99, guard_id);
        assert_eq!(45, max_minute);
    }
}
