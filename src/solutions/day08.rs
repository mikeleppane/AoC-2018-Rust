use aoc_2018_rust::{numbers, output, Runner};
use std::slice::Iter;
const INPUT: &str = "input/day08.txt";

#[derive(Default)]
pub struct Day08 {
    tree: Vec<u64>,
}

impl Day08 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Day08 {
    fn name(&self) -> (usize, usize) {
        (2018, 8)
    }

    fn parse(&mut self) {
        self.tree = numbers(INPUT, ' ').first().unwrap().clone();
    }

    fn part1(&mut self) -> Vec<String> {
        output(sum_metadata(&mut self.tree.iter()))
    }

    fn part2(&mut self) -> Vec<String> {
        output(value_of(&mut self.tree.iter()))
    }
}

// ---------------------------------------------------
fn sum_metadata(iter: &mut Iter<u64>) -> u64 {
    let num_children = *iter.next().unwrap();
    let num_metadata = *iter.next().unwrap();

    let mut sum = 0;
    for _ in 0..num_children {
        sum += sum_metadata(iter);
    }

    for _ in 0..num_metadata {
        sum += iter.next().unwrap();
    }

    sum
}

fn value_of(iter: &mut Iter<u64>) -> u64 {
    let num_children = *iter.next().unwrap();
    let num_metadata = *iter.next().unwrap();

    let mut sum = 0;
    if num_children == 0 {
        return iter.take(num_metadata as usize).sum();
    } else {
        let mut children = Vec::new();
        for _ in 0..num_children {
            children.push(value_of(iter));
        }

        for _ in 0..num_metadata {
            let idx = *iter.next().unwrap() as usize;
            if idx > 0 && idx <= num_children as usize {
                sum += children[idx - 1];
            }
        }
    }

    sum
}

// --------------------------------------
#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn part1_works() {
        let mut day = Day08::new();
        day.parse();
        let output = day.part1();
        assert_eq!(output[0], "48496")
    }

    #[test]
    fn part2_works() {
        let mut day = Day08::new();
        day.parse();
        let output = day.part2();
        assert_eq!(output[0], "32850")
    }
}
