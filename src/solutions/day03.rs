use std::collections::HashMap;

use aoc_2018_rust::{output, read_lines, OutputStatus, Runner};

const INPUT: &str = "input/day03.txt";

#[derive(Default)]
pub struct Day03 {
    claims: Vec<Claim>,
    fabric: HashMap<(usize, usize), usize>,
}

impl Day03 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Day03 {
    fn name(&self) -> (usize, usize) {
        (2018, 1)
    }

    fn parse(&mut self) {
        for claim in read_lines(INPUT).iter() {
            let (id, rest) = claim.split_once(" @ ").unwrap();
            let (xy, wh) = rest.split_once(':').unwrap();
            let (x, y) = xy.split_once(',').unwrap();
            let (w, h) = wh.split_once('x').unwrap();
            let id = id[1..].parse::<usize>().unwrap();
            let x = x.trim().parse::<usize>().unwrap();
            let y = y.trim().parse::<usize>().unwrap();
            let w = w.trim().parse::<usize>().unwrap();
            let h = h.trim().parse::<usize>().unwrap();
            self.claims.push(Claim { id, x, y, w, h });
        }
    }

    fn part1(&mut self) -> Vec<String> {
        for c in &self.claims {
            for x in 0..c.w {
                for y in 0..c.h {
                    *self.fabric.entry((c.x + x, c.y + y)).or_insert(0) += 1;
                }
            }
        }
        output(self.fabric.values().filter(|x| **x > 1).count())
    }

    fn part2(&mut self) -> Vec<String> {
        for c in &self.claims {
            for x in 0..c.w {
                for y in 0..c.h {
                    *self.fabric.entry((c.x + x, c.y + y)).or_insert(0) += 1;
                }
            }
        }
        'next_claim: for c in &self.claims {
            for x in 0..c.w {
                for y in 0..c.h {
                    let Some(val) = self.fabric.get(&(c.x + x, c.y + y)) else {
                        panic!("no value found")
                    };
                    if *val != 1 {
                        continue 'next_claim;
                    }
                }
            }
            return output(c.id);
        }
        output(OutputStatus::Failed)
    }
}

// ---------------------------------------------------

struct Claim {
    id: usize,
    x: usize,
    y: usize,
    w: usize,
    h: usize,
}

// --------------------------------------
#[cfg(test)]
mod tests {
    use aoc_2018_rust::Runner;
    use pretty_assertions::assert_eq;

    use crate::solutions::day03::Day03;

    #[test]
    fn part1_works() {
        let mut day03 = Day03::new();
        day03.parse();
        let output = day03.part1();
        assert_eq!(output[0], "97218")
    }

    #[test]
    fn part2_works() {
        let mut day03 = Day03::new();
        day03.parse();
        let output = day03.part2();
        assert_eq!(output[0], "717")
    }
}
