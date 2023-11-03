use std::collections::HashMap;

use aoc_2018_rust::{output, read_lines, OutputStatus, Runner};

const INPUT: &str = "input/day02.txt";

#[derive(Default)]
pub struct Day02 {
    boxid: Vec<String>,
}

impl Day02 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Day02 {
    fn name(&self) -> (usize, usize) {
        (2018, 1)
    }

    fn parse(&mut self) {
        self.boxid = read_lines(INPUT)
    }

    fn part1(&mut self) -> Vec<String> {
        let mut count_of_twos = 0;
        let mut count_of_threes = 0;
        for id in &self.boxid {
            let mut seen = HashMap::new();
            for c in id.chars() {
                let count = seen.entry(c).or_insert(0);
                *count += 1;
            }
            if seen.iter().any(|x| *x.1 == 2) {
                count_of_twos += 1;
            }
            if seen.iter().any(|x| *x.1 == 3) {
                count_of_threes += 1;
            }
        }
        output(count_of_twos * count_of_threes)
    }

    fn part2(&mut self) -> Vec<String> {
        for i in 0..self.boxid.len() {
            for j in i + 1..self.boxid.len() {
                let (id1, id2) = (&self.boxid[i], &self.boxid[j]);
                let mut diff = 0;
                let mut common = String::new();
                for (c1, c2) in id1.chars().zip(id2.chars()) {
                    if c1 != c2 {
                        diff += 1;
                        if diff > 1 {
                            break;
                        }
                    } else {
                        common.push(c1);
                    }
                }
                if diff == 1 {
                    return output(common);
                }
            }
        }
        output(OutputStatus::Failed)
    }
}

// ---------------------------------------------------

// --------------------------------------
#[cfg(test)]
mod tests {
    use aoc_2018_rust::Runner;
    use pretty_assertions::assert_eq;

    use crate::solutions::day02::Day02;

    #[test]
    fn part1_works() {
        let mut day02 = Day02::new();
        day02.parse();
        let output = day02.part1();
        assert_eq!(output[0], "6370")
    }

    #[test]
    fn part2_works() {
        let mut day02 = Day02::new();
        day02.parse();
        let output = day02.part2();
        assert_eq!(output[0], "rmyxgdlihczskunpfijqcebtv")
    }
}
