#![allow(dead_code)]

mod solutions;
mod utils;

use solutions::*;
use utils::load_input;

fn main() {
    let input = load_input("input.txt").unwrap();
    let solution = day2::part2(&input);
    println!("Day {} - Part {} : {}", 2, 2, solution);
}