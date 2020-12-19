extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;
use std::io::Read;

#[derive(Parser)]
#[grammar = "small.pest"]
struct MessageParser {}

fn main() {
    let mut stdin = std::io::stdin();
    let mut input = String::default();

    stdin.read_to_string(&mut input).unwrap();
    println!(
        "{}",
        input
            .lines()
            .map(|line| match MessageParser::parse(Rule::a0, line) {
                Ok(_) => 1,
                Err(_) => 0,
            })
            .sum::<i32>()
    );
}
