use regex::Regex;
use std::collections::HashSet;
use std::io::Read;

#[derive(Debug)]
pub enum Instructions {
    Nop(i32),
    Jmp(i32),
    Acc(i32),
}

fn main() {
    let mut stdin = std::io::stdin();
    let mut input = String::default();

    stdin.read_to_string(&mut input).unwrap();
    let regex = Regex::new(r"(?P<type>jmp|nop|acc) (?P<sign>\+|-)(?P<amount>\d*)").unwrap();
    let instructions: Vec<_> = input
        .lines()
        .map(|line| {
            let captures = regex.captures(line).unwrap();
            let sign = match &captures["sign"] {
                "+" => 1,
                "-" => -1,
                _ => 0,
            };
            let arg: i32 = captures["amount"].parse().unwrap();
            match &captures["type"] {
                "jmp" => Instructions::Jmp(sign * arg),
                "nop" => Instructions::Nop(sign * arg),
                "acc" => Instructions::Acc(sign * arg),
                _ => Instructions::Nop(0),
            }
        })
        .collect();
    let mut acc = 0;
    let mut curr: i32 = 0;
    let mut seen = HashSet::new();
    loop {
        if seen.contains(&curr) {
            println!("{}", acc);
            break;
        }
        seen.insert(curr);
        match instructions[curr as usize] {
            Instructions::Nop(_) => curr += 1,
            Instructions::Jmp(arg) => curr += arg,
            Instructions::Acc(arg) => {
                acc += arg;
                curr += 1;
            }
        }
    }

    for i in 0..instructions.len() {
        if let Instructions::Acc(_) = instructions[i] {
            continue;
        }
        let mut acc = 0;
        let mut curr: i32 = 0;
        let mut seen = HashSet::new();
        loop {
            if curr == instructions.len() as i32 {
                println!("{}", acc);
                return;
            }
            if seen.contains(&curr) {
                break;
            }
            seen.insert(curr);
            match instructions[curr as usize] {
                Instructions::Nop(arg) => {
                    if curr == i as i32 {
                        curr += arg;
                    } else {
                        curr += 1;
                    }
                }
                Instructions::Jmp(arg) => {
                    if curr == i as i32 {
                        curr += 1;
                    } else {
                        curr += arg;
                    }
                }
                Instructions::Acc(arg) => {
                    acc += arg;
                    curr += 1;
                }
            }
        }
    }
}
