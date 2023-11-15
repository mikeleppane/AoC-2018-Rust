use std::collections::HashSet;

use aoc_2018_rust::{output, read_lines, Point, Runner};
const INPUT: &str = "input/day10.txt";

#[derive(Default)]
pub struct Day10 {
    lights: Vec<Light>,
    count: usize,
}

impl Day10 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Day10 {
    fn name(&self) -> (usize, usize) {
        (2018, 10)
    }

    fn parse(&mut self) {
        let lines = read_lines(INPUT);
        for line in lines {
            let words = line.split('<').collect::<Vec<_>>();
            let (pos, _) = words[1].split_once('>').unwrap();
            let (vel, _) = words[2].split_once('>').unwrap();
            self.lights.push(Light {
                pos: pos.parse().unwrap(),
                vel: vel.parse().unwrap(),
            })
        }
    }

    fn part1(&mut self) -> Vec<String> {
        let mut min_y;
        let mut max_y;
        let mut min_x;
        let mut max_x;
        loop {
            min_y = self.lights[0].pos.y;
            max_y = self.lights[0].pos.y;
            min_x = self.lights[0].pos.x;
            max_x = self.lights[0].pos.x;
            for light in &mut self.lights {
                light.pos += light.vel;
                min_y = min_y.min(light.pos.y);
                max_y = max_y.max(light.pos.y);
                min_x = min_x.min(light.pos.x);
                max_x = max_x.max(light.pos.x);
            }
            self.count += 1;
            if max_y - min_y == 9 {
                break;
            }
        }
        let mut display = HashSet::new();
        for light in &self.lights {
            display.insert((light.pos.x, light.pos.y));
        }
        let mut output = Vec::new();
        for y in min_y..=max_y {
            let mut line = String::new();
            for x in min_x..=max_x {
                if display.contains(&(x, y)) {
                    line.push('#');
                } else {
                    line.push('.');
                }
            }
            output.push(line);
        }
        output
    }

    fn part2(&mut self) -> Vec<String> {
        self.part1();
        output(self.count)
    }
}

// ---------------------------------------------------

struct Light {
    pos: Point<i64>,
    vel: Point<i64>,
}

// --------------------------------------
#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn part1_works() {
        let mut day = Day10::new();
        day.parse();
        let output = day.part1();
        println!("{:#?}", output);
    }

    #[test]
    fn part2_works() {
        let mut day = Day10::new();
        day.parse();
        let output = day.part2();
        assert_eq!(output[0], "10333")
    }
}
