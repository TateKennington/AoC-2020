use std::collections::HashSet;
use std::io::Read;

fn main() {
    let mut stdin = std::io::stdin();
    let mut input = String::default();
    let mut part1 = 0;

    stdin.read_to_string(&mut input).unwrap();
    let lines: Vec<i64> = input
        .lines()
        .map(|line| line.trim().parse().unwrap())
        .collect();
    for i in 25..lines.len() {
        let mut set = HashSet::new();
        let mut found = false;
        let curr = lines[i];
        for j in i - 25..i {
            if set.contains(&lines[j]) {
                found = true;
                break;
            }
            set.insert(curr - lines[j]);
        }
        if !found {
            part1 = curr;
            break;
        }
    }
    println!("Part 1: {}", part1);

    let mut start = 0;
    let mut end = 0;
    let mut curr_sum = lines[0];
    while curr_sum != part1 {
        if curr_sum > part1 {
            curr_sum -= lines[start];
            start += 1;
        } else {
            end += 1;
            curr_sum += lines[end];
        }
    }
    println!(
        "Part 2: {}",
        lines[start..=end].iter().max().unwrap() + lines[start..=end].iter().min().unwrap()
    );
}
