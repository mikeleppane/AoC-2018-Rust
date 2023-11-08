use aoc_2018_rust::{output, Runner};
use std::fs::read_to_string;

const INPUT: &str = "input/day05.txt";

#[derive(Default)]
pub struct Day05 {
    reactant: String,
}

impl Day05 {
    pub fn new() -> Self {
        Self::default()
    }

    fn react(&self, input: &str) -> String {
        let mut output = String::new();
        let mut chars = input.chars();
        let mut last = chars.next().unwrap();
        output.push(last);
        for c in chars {
            if c != last && c.eq_ignore_ascii_case(&last) {
                output.pop();
                last = output.chars().last().unwrap_or(' ');
            } else {
                output.push(c);
                last = c;
            }
        }
        output
    }

    fn shortest_polymer(&self, input: &str) -> usize {
        let mut shortest = input.len();
        for c in b'a'..=b'z' {
            let c = c as char;
            let polymer = input.replace([c, c.to_ascii_uppercase()], "");
            let reacted = self.react(&polymer);
            if reacted.len() < shortest {
                shortest = reacted.len();
            }
        }
        shortest
    }
}

impl Runner for Day05 {
    fn name(&self) -> (usize, usize) {
        (2018, 5)
    }

    fn parse(&mut self) {
        self.reactant = read_to_string(INPUT).unwrap();
    }

    fn part1(&mut self) -> Vec<String> {
        let res = self.react(&self.reactant);
        output(res.len())
    }

    fn part2(&mut self) -> Vec<String> {
        let res = self.shortest_polymer(&self.reactant);
        output(res)
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
        let mut day = Day05::new();
        day.parse();
        let output = day.part1();
        assert_eq!(output[0], "11108")
    }

    #[test]
    fn part2_works() {
        let mut day = Day05::new();
        day.parse();
        let output = day.part2();
        assert_eq!(output[0], "5094")
    }
}
