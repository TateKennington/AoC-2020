use std::io::Read;

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
    let mut memo: Vec<u64> = Vec::with_capacity(nums.len());
    for _i in 0..nums.len() {
        memo.push(0);
    }
    memo[nums.len() - 1] = 1;
    let mut i = (nums.len() - 2) as i32;
    while i >= 0 {
        let mut res = 0;
        for j in i + 1..nums.len() as i32 {
            if nums[j as usize] - nums[i as usize] > 3 {
                break;
            }
            res += memo[j as usize];
        }
        memo[i as usize] = res;
        i -= 1
    }
    println!("{}", memo[0]);
}
