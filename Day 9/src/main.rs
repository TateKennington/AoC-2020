use std::collections::HashSet;
use std::io::Read;

fn main() {
    let mut stdin = std::io::stdin();
    let mut input = String::default();
    let mut part1 = 0;

    stdin.read_to_string(&mut input).unwrap();
    let nums: Vec<i64> = input
        .lines()
        .map(|line| line.trim().parse().unwrap())
        .collect();
    for i in 25..nums.len() {
        let mut sums = HashSet::new();
        let mut found = false;
        let curr = nums[i];
        for j in i - 25..i {
            if sums.contains(&nums[j]) {
                found = true;
                break;
            }
            sums.insert(curr - nums[j]);
        }
        if !found {
            part1 = curr;
            break;
        }
    }
    println!("Part 1: {}", part1);

    let mut start = 0;
    let mut end = 0;
    let mut curr_sum = nums[0];
    while curr_sum != part1 {
        if curr_sum > part1 {
            curr_sum -= nums[start];
            start += 1;
        } else {
            end += 1;
            curr_sum += nums[end];
        }
    }
    println!(
        "Part 2: {}",
        nums[start..=end].iter().max().unwrap() + nums[start..=end].iter().min().unwrap()
    );
}
