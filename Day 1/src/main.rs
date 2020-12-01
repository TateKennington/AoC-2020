use std::collections::HashSet;
use std::io::stdin;

fn main() {
    //part_1();
    part_2();
}

fn part_1() {
    let mut stdin = stdin();
    let mut input = String::default();
    let mut nums = HashSet::new();

    let mut bytes = stdin.read_line(&mut input).unwrap();
    while (bytes > 0) {
        let n: i32 = input.trim().parse().unwrap();
        if nums.contains(&n) {
            println!("{}", n * (2020 - n));
            return;
        }
        nums.insert(2020 - n);
        input = String::default();
        bytes = stdin.read_line(&mut input).unwrap();
    }
}

fn part_2() {
    let mut stdin = stdin();
    let mut input = String::default();
    let mut nums = HashSet::new();

    let mut bytes = stdin.read_line(&mut input).unwrap();
    while (bytes > 0) {
        let n: i32 = input.trim().parse().unwrap();
        nums.insert(n);
        input = String::default();
        bytes = stdin.read_line(&mut input).unwrap();
    }

    for a in &nums {
        for b in &nums {
            for c in &nums {
                if *a + *b + *c == 2020 {
                    println!("{}", *a * *b * *c);
                    return;
                }
            }
        }
    }
}
