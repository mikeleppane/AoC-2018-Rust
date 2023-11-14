use aoc_2018_rust::{output, Runner};
use std::collections::VecDeque;
use std::fs::read_to_string;
const INPUT: &str = "input/day09.txt";

#[derive(Default)]
pub struct Day09 {
    players: usize,
    marbles: usize,
}

impl Day09 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Day09 {
    fn name(&self) -> (usize, usize) {
        (2018, 9)
    }

    fn parse(&mut self) {
        let nums: Vec<usize> = read_to_string(INPUT)
            .unwrap()
            .split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();
        self.players = nums[0] as usize;
        self.marbles = nums[1] as usize;
    }

    fn part1(&mut self) -> Vec<String> {
        let mut score = vec![0; self.players];
        let mut circle = VecDeque::new();
        circle.push_back(0usize);

        for m in 1..self.marbles {
            if m % 23 == 0 && m > 0 {
                for _ in 0..7 {
                    let tmp = circle.pop_back().unwrap();
                    circle.push_front(tmp);
                }
                score[m % self.players] += m + circle.pop_front().unwrap();
            } else {
                for _ in 0..2 {
                    let tmp = circle.pop_front().unwrap();
                    circle.push_back(tmp);
                }
                circle.push_front(m);
            }
        }

        output(score.iter().max().unwrap())
    }

    fn part2(&mut self) -> Vec<String> {
        self.marbles *= 100;
        self.part1()
    }
}

// ---------------------------------------------------

// --------------------------------------
#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn part1_works() {
        let mut day = Day09::new();
        day.parse();
        let output = day.part1();
        assert_eq!(output[0], "439089")
    }

    #[test]
    fn part2_works() {
        let mut day = Day09::new();
        day.parse();
        let output = day.part2();
        assert_eq!(output[0], "3668541094")
    }
}
