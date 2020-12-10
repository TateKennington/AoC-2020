use std::collections::HashMap;
use std::io::Read;

fn process(curr: u64, nums: &Vec<u64>, memo: &mut HashMap<u64, u64>) -> u64 {
    if curr + 1 == nums.len() as u64 {
        return 1;
    }
    if memo.contains_key(&curr) {
        return memo[&curr];
    }
    let mut res = 0;
    let mut i = curr + 1;
    while i < nums.len() as u64 {
        if nums[i as usize] - nums[curr as usize] > 3 {
            break;
        }
        res += process(i, nums, memo);
        i += 1;
    }
    memo.insert(curr, res);
    return res;
}

fn main() {
    let mut stdin = std::io::stdin();
    let mut input = String::default();

    stdin.read_to_string(&mut input).unwrap();
    let mut nums = input
        .lines()
        .map(|line| line.trim().parse().unwrap())
        .collect::<Vec<u64>>();
    nums.push(0);
    nums.sort();
    let mut one = 0;
    let mut three = 1;
    nums.windows(2).for_each(|nums| {
        if nums[1] - nums[0] == 1 {
            one += 1;
        }
        if nums[1] - nums[0] == 3 {
            three += 1;
        }
    });
    println!("{}", one * three);
    println!("{}", process(0, &nums, &mut HashMap::new()))
}
