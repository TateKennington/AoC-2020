use std::collections::HashMap;
use std::io::Read;

fn main() {
    let mut stdin = std::io::stdin();
    let mut input = String::default();

    let mut said = HashMap::new();
    let mut turn = 1;
    let mut curr = 0;
    stdin.read_to_string(&mut input).unwrap();
    input
        .trim()
        .split(",")
        .map(|s| s.parse::<i32>().unwrap())
        .for_each(|num| {
            said.insert(num, turn);
            turn += 1;
        });
    while turn < 30000000 {
        if turn == 2020 {
            println!("Part 1: {}", curr);
        }
        let next = if !said.contains_key(&curr) {
            0
        } else {
            turn - said[&curr]
        };
        said.insert(curr, turn);
        curr = next;
        turn += 1;
    }
    println!("Part 2: {}", curr);
}
