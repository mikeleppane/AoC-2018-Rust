use std::collections::HashSet;

use aoc_2018_rust::{output, read_lines, Runner};

const INPUT: &str = "input/day01.txt";

pub struct Day01 {
    nums: Vec<i32>,
}

impl Day01 {
    pub fn new() -> Self {
        Self { nums: Vec::new() }
    }
}

impl Runner for Day01 {
    fn name(&self) -> (usize, usize) {
        (2018, 1)
    }

    fn parse(&mut self) {
        self.nums = read_lines(INPUT)
            .into_iter()
            .map(|s| s.parse::<i32>().unwrap())
            .collect::<Vec<_>>();
    }

    fn part1(&mut self) -> Vec<String> {
        output(self.nums.iter().sum::<i32>())
    }

    fn part2(&mut self) -> Vec<String> {
        let mut seen = HashSet::new();
        let mut sum = 0;
        for num in self.nums.iter().cycle() {
            if !seen.insert(sum) {
                break;
            }
            sum += num;
        }
        output(sum)
    }
}

// ---------------------------------------------------

// --------------------------------------
#[cfg(test)]
mod tests {
    use aoc_2018_rust::Runner;

    use crate::solutions::day01::Day01;

    #[test]
    fn part1_works() {
        let mut day01 = Day01::new();
        day01.parse();
        let output = day01.part1();
        assert_eq!(output[0], "587")
    }

    #[test]
    fn part2_works() {
        let mut day01 = Day01::new();
        day01.parse();
        let output = day01.part2();
        assert_eq!(output[0], "83130")
    }
}
