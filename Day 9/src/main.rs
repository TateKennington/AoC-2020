use std::collections::HashSet;
use std::io::Read;

fn main() {
    let mut stdin = std::io::stdin();
    let mut input = String::default();

    stdin.read_to_string(&mut input).unwrap();
    let nums = input
        .lines()
        .map(|line| line.trim().parse().unwrap())
        .collect::<Vec<i64>>();
    let part1 = nums
        .windows(26)
        .find_map(|nums| {
            let mut sums = HashSet::new();
            let curr = nums[nums.len() - 1];
            for j in 0..nums.len() - 1 {
                if sums.contains(&nums[j]) {
                    return None;
                }
                sums.insert(curr - nums[j]);
            }
            Some(curr)
        })
        .unwrap();
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
