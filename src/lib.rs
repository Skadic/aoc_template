pub const INPUT: &str = include_str!("../input.txt");

pub mod part1;
pub mod part2;

#[tracing::instrument(name = "parse", skip(input))]
pub fn parse_input(input: &str) {
    let _ = input;
    todo!("parse input")
}

