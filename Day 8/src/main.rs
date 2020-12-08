use regex::Regex;
use std::collections::HashMap;
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
    let mut loops = HashMap::new();
    let mut acc = 0;
    let mut curr: i32 = 0;
    let mut critical_loop = HashSet::new();
    loop {
        if critical_loop.contains(&curr) {
            println!("{}", acc);
            break;
        }
        critical_loop.insert(curr);
        loops.insert(curr, true);
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
        let mut curr = i as i32;
        if loops.contains_key(&curr) {
            continue;
        }
        let mut seen = HashSet::new();
        loop {
            if curr == instructions.len() as i32 {
                seen.iter().for_each(|pc| {
                    loops.insert(*pc, false);
                });
                break;
            }
            if seen.contains(&curr) {
                seen.iter().for_each(|pc| {
                    loops.insert(*pc, true);
                });
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
    }
    let mut mutation = 0;
    for pc in critical_loop.iter() {
        if let Instructions::Nop(arg) = instructions[*pc as usize] {
            if !loops[&(*pc + arg)] {
                mutation = *pc;
                break;
            }
        }
        if let Instructions::Jmp(_) = instructions[*pc as usize] {
            if !loops[&(*pc + 1)] {
                mutation = *pc;
                break;
            }
        }
    }

    let mut acc = 0;
    let mut curr: i32 = 0;
    loop {
        if curr == instructions.len() as i32 {
            println!("{}", acc);
            break;
        }
        if curr == mutation {
            match instructions[curr as usize] {
                Instructions::Nop(arg) => curr += arg,
                Instructions::Jmp(_) => curr += 1,
                Instructions::Acc(arg) => {
                    acc += arg;
                    curr += 1;
                }
            }
        } else {
            match instructions[curr as usize] {
                Instructions::Nop(_) => curr += 1,
                Instructions::Jmp(arg) => curr += arg,
                Instructions::Acc(arg) => {
                    acc += arg;
                    curr += 1;
                }
            }
        }
    }
}
