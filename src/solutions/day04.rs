use std::collections::HashMap;

use aoc_2018_rust::{output, read_lines, Runner};

const INPUT: &str = "input/day04.txt";

#[derive(Default)]
pub struct Day04 {
    guards: HashMap<usize, Guard>,
}

impl Day04 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Day04 {
    fn name(&self) -> (usize, usize) {
        (2018, 4)
    }

    fn parse(&mut self) {
        let mut log = read_lines(INPUT);
        log.sort();

        let mut guard_id = 0;
        let mut sleep_time = 0;

        for line in &log {
            match Log::parse(line) {
                Log::GuardID(id) => {
                    guard_id = id;
                }
                Log::Sleep(minute) => {
                    sleep_time = minute;
                }
                Log::Wake(minute) => {
                    let guard = self.guards.entry(guard_id).or_insert(Guard::new());
                    for t in sleep_time..minute {
                        guard.minutes[t] += 1;
                    }
                    guard.total += minute - sleep_time;
                }
            }
        }
    }

    fn part1(&mut self) -> Vec<String> {
        let max_guard = self.guards.iter().max_by_key(|g| g.1.total).unwrap();
        output(max_guard.0 * max_guard.1.max_minute())
    }

    fn part2(&mut self) -> Vec<String> {
        let g = self
            .guards
            .iter()
            .max_by_key(|g| g.1.minutes.iter().max())
            .unwrap();
        output(g.0 * g.1.max_minute())
    }
}

// ---------------------------------------------------

struct Guard {
    minutes: [usize; 60],
    total: usize,
}

impl Guard {
    fn new() -> Self {
        Self {
            minutes: [0; 60],
            total: 0,
        }
    }

    fn max_minute(&self) -> usize {
        self.minutes
            .iter()
            .enumerate()
            .max_by_key(|m| m.1)
            .unwrap()
            .0
    }
}

enum Log {
    GuardID(usize),
    Sleep(usize),
    Wake(usize),
}

impl Log {
    fn parse(s: &str) -> Self {
        let mut iter = s.split_whitespace();
        let _ = iter.next().unwrap();
        let time = iter.next().unwrap();
        let minute = time[3..5].parse::<usize>().unwrap();
        let action = iter.next().unwrap();

        match action {
            "Guard" => {
                let id = iter.next().unwrap()[1..].parse::<usize>().unwrap();
                Log::GuardID(id)
            }
            "falls" => Log::Sleep(minute),
            "wakes" => Log::Wake(minute),
            _ => panic!("Unknown action"),
        }
    }
}

// --------------------------------------
#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn part1_works() {
        let mut day = Day04::new();
        day.parse();
        let output = day.part1();
        assert_eq!(output[0], "77941")
    }

    #[test]
    fn part2_works() {
        let mut day = Day04::new();
        day.parse();
        let output = day.part2();
        assert_eq!(output[0], "35289")
    }
}
