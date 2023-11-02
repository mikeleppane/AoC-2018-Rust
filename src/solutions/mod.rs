use crate::Selector;
use aoc_2018_rust::{run_solution, Runner};
use day01::Day01;

mod day01;

pub fn run(which: Selector) {
    let mut day01 = Day01::new();

    let mut days: Vec<&mut dyn Runner> = vec![&mut day01];

    match which {
        Selector::Last => {
            let last = days.len() - 1;
            if let Some(d) = days.get_mut(last) {
                run_solution(*d);
            }
        }
        Selector::All => {
            for d in days {
                run_solution(d);
            }
        }
        Selector::One(num) => {
            if let Some(d) = days.get_mut(num - 1) {
                run_solution(*d);
            }
        }
    }
}
