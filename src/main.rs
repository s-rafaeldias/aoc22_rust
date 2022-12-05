mod day_01;
mod day_02;
use std::fs;

fn main() {
    let input = fs::read_to_string("data/day_one.txt").unwrap();

    let r1 = day_01::part_one(&input, 1);
    println!("part one: {:?}", r1)
}
