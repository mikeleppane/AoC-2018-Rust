use aoc_2018_rust::{output, read_lines, Runner};
use std::str::FromStr;

const INPUT: &str = "input/day06.txt";

#[derive(Default)]
pub struct Day06 {
    points: Vec<Point>,
    min: Point,
    max: Point,
}

impl Day06 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Day06 {
    fn name(&self) -> (usize, usize) {
        (2018, 6)
    }

    fn parse(&mut self) {
        let lines = read_lines(INPUT);

        self.min.x = isize::MAX;
        self.min.y = isize::MAX;

        for l in &lines {
            let p = Point::from_str(l).unwrap();
            self.min = self.min.min(p);
            self.max = self.max.max(p);
            self.points.push(p);
        }
    }

    fn part1(&mut self) -> Vec<String> {
        let mut map = vec![0usize; self.points.len()];
        for x in self.min.x..=self.max.x {
            for y in self.min.y..=self.max.y {
                let mut closest_point = self.points.len();
                let mut min_dist = self.max.x + self.max.y;
                let mut found = false;
                for i in 0..self.points.len() {
                    let d = self.points[i].distance(&Point::new(x, y));
                    if d < min_dist {
                        min_dist = d;
                        closest_point = i;
                        found = true;
                        continue;
                    }
                    if d == min_dist {
                        found = false;
                    }
                }

                if !found {
                    continue;
                }
                if x == self.min.x || x == self.max.x || y == self.min.y || y == self.max.y {
                    self.points[closest_point].infinite = true;
                    map[closest_point] = 0;
                } else if !self.points[closest_point].infinite {
                    map[closest_point] += 1;
                }
            }
        }
        output(map.iter().max().unwrap())
    }

    fn part2(&mut self) -> Vec<String> {
        let mut total = 0;
        for x in self.min.x..=self.max.x {
            for y in self.min.y..=self.max.y {
                let total_dist = self
                    .points
                    .iter()
                    .map(|p| p.distance(&Point::new(x, y)))
                    .sum::<isize>();
                total += (total_dist < 10000) as usize;
            }
        }
        output(total)
    }
}

// ---------------------------------------------------

#[derive(Debug, Default, Clone, Copy)]
struct Point {
    x: isize,
    y: isize,
    infinite: bool,
}

impl Point {
    fn new(x: isize, y: isize) -> Self {
        Self {
            x,
            y,
            infinite: false,
        }
    }

    fn distance(&self, other: &Self) -> isize {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }

    fn min(&self, other: Self) -> Self {
        Self::new(self.x.min(other.x), self.y.min(other.y))
    }

    fn max(&self, other: Self) -> Self {
        Self::new(self.x.max(other.x), self.y.max(other.y))
    }
}

impl FromStr for Point {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = s.split_once(", ").ok_or("input error")?;
        let x = x.parse().map_err(|_| "x value error")?;
        let y = y.parse().map_err(|_| "y value error")?;
        Ok(Self::new(x, y))
    }
}

// --------------------------------------
#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn part1_works() {
        let mut day = Day06::new();
        day.parse();
        let output = day.part1();
        assert_eq!(output[0], "5035")
    }

    #[test]
    fn part2_works() {
        let mut day = Day06::new();
        day.parse();
        let output = day.part2();
        assert_eq!(output[0], "35294")
    }
}
