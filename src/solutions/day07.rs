use std::collections::{BTreeMap, BTreeSet, HashSet};

use aoc_2018_rust::{output, read_lines, OutputStatus, Runner};

const INPUT: &str = "input/day07.txt";

#[derive(Default)]
pub struct Day07 {
    graph: BTreeMap<char, Node>,
    possible: BTreeSet<char>,
}

impl Day07 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Day07 {
    fn name(&self) -> (usize, usize) {
        (2018, 7)
    }

    fn parse(&mut self) {
        let lines = read_lines(INPUT);

        for l in lines {
            let mut words = l.split_whitespace();
            let node = words.nth(1).unwrap().chars().next().unwrap();
            let prereq = words.nth(7).unwrap().chars().next().unwrap();
            self.possible.insert(node);
            self.possible.insert(prereq);
            let e = self.graph.entry(node).or_default();
            e.prereq.insert(prereq);

            let e = self.graph.entry(prereq).or_default();
            e.count += 1;
        }
    }

    fn part1(&mut self) -> Vec<String> {
        let mut completed = String::new();
        for node in self.graph.values() {
            for ch in node.prereq.iter() {
                self.possible.remove(ch);
            }
        }

        while let Some(item) = self.possible.pop_first() {
            completed.push(item);
            let prereq = self.graph.get(&item).unwrap().clone();
            for ch in prereq.prereq.iter() {
                let e = self.graph.get_mut(ch).unwrap();
                e.count -= 1;
                if e.count == 0 {
                    self.possible.insert(*ch);
                }
            }
        }

        output(completed)
    }

    fn part2(&mut self) -> Vec<String> {
        output(OutputStatus::Unsolved)
    }
}

// ---------------------------------------------------

#[derive(Debug, Default, Clone)]
struct Node {
    prereq: HashSet<char>,
    count: usize,
}

// --------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_works() {
        let mut day = Day07::new();
        day.parse();
        let _ = day.part1();
    }

    #[test]
    fn part2_works() {
        let mut day = Day07::new();
        day.parse();
        let _ = day.part2();
    }
}
