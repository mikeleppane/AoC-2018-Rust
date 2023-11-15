use aoc_2018_rust::{output, read_lines, Runner};
const INPUT: &str = "input/day11.txt";

#[derive(Default)]
pub struct Day11 {
    serial_number: i32,
}

impl Day11 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Day11 {
    fn name(&self) -> (usize, usize) {
        (2018, 11)
    }

    fn parse(&mut self) {
        self.serial_number = read_lines(INPUT)[0].parse().unwrap();
    }

    fn part1(&mut self) -> Vec<String> {
        let mut max_power = 0;
        let mut top_left_x = 1;
        let mut top_left_y = 1;
        for x in 1..=297 {
            for y in 1..=297 {
                let mut power = 0;
                for i in x..x + 3 {
                    for j in y..y + 3 {
                        power += calculate_fuel_power(i, j, self.serial_number)
                    }
                }
                if power > max_power {
                    max_power = power;
                    top_left_x = x;
                    top_left_y = y;
                }
            }
        }
        output(format!("{},{}", top_left_x, top_left_y))
    }

    fn part2(&mut self) -> Vec<String> {
        let mut max_power = 0;
        let mut top_left_x = 1;
        let mut top_left_y = 1;
        let mut size = 1;
        for s in 1..=300 {
            for x in 1..=(301 - s) {
                for y in 1..=(301 - s) {
                    let mut power = 0;
                    for i in x..x + s {
                        for j in y..y + s {
                            power += calculate_fuel_power(i, j, self.serial_number)
                        }
                    }
                    if power > max_power {
                        max_power = power;
                        top_left_x = x;
                        top_left_y = y;
                        size = s;
                    }
                }
            }
        }
        output(format!("{},{},{}", top_left_x, top_left_y, size))
    }
}

// ---------------------------------------------------

fn calculate_fuel_power(x: i32, y: i32, serial_number: i32) -> i32 {
    let rack_id = x + 10;
    let mut power = rack_id * y;
    power += serial_number;
    power *= rack_id;
    power = (power / 100) % 10;
    power -= 5;
    power
}

// --------------------------------------
#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn part1_works() {
        let mut day = Day11::new();
        day.parse();
        let output = day.part1();
        assert_eq!(output[0], "21,22")
    }

    #[test]
    fn part2_works() {
        let mut day = Day11::new();
        day.parse();
        let output = day.part2();
        assert_eq!(output[0], "235,288,13")
    }
}
