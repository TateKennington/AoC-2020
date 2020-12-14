use std::collections::HashMap;
use std::io::Read;

pub enum Op {
    Mask,
    Mem,
}

fn main() {
    let mut stdin = std::io::stdin();
    let mut input = String::default();

    stdin.read_to_string(&mut input).unwrap();
    let regex =
        regex::Regex::new(r"(?P<type>mem|mask)(\[(?P<addr>\d+)\])? = (?P<value>[0-9X]+)").unwrap();
    let ops: Vec<_> = input
        .lines()
        .map(|line| {
            let captures = regex.captures(line).unwrap();
            let op = if &captures["type"] == "mem" {
                Op::Mem
            } else {
                Op::Mask
            };
            let value = String::from(&captures["value"]);
            let addr: u64 = match captures.name("addr") {
                None => 0,
                Some(_) => captures["addr"].parse().unwrap(),
            };
            return (op, value, addr);
        })
        .collect();
    let mut mem = HashMap::new();
    let mut ones_mask: u64 = 0;
    let mut zeros_mask: u64 = 1 << 37 - 1;
    for op in &ops {
        match op.0 {
            Op::Mask => {
                op.1.chars()
                    .rev()
                    .enumerate()
                    .for_each(|(index, c)| match c {
                        '1' => {
                            ones_mask = ones_mask | 1 << index;
                            zeros_mask = zeros_mask | 1 << index;
                        }
                        '0' => {
                            ones_mask = ones_mask & !(1 << index);
                            zeros_mask = zeros_mask & !(1 << index);
                        }
                        'X' => {
                            ones_mask = ones_mask & !(1 << index);
                            zeros_mask = zeros_mask | 1 << index;
                        }
                        _ => {}
                    })
            }
            Op::Mem => {
                mem.insert(
                    op.2,
                    (op.1.parse::<u64>().unwrap() | ones_mask) & zeros_mask,
                );
            }
        }
    }
    println!("{}", mem.iter().map(|(_k, v)| v).sum::<u64>());
    let mut mem = HashMap::new();
    let mut mask: u64 = 0;
    let mut floating = Vec::new();
    for op in ops {
        match op.0 {
            Op::Mask => {
                floating = Vec::new();
                op.1.chars()
                    .rev()
                    .enumerate()
                    .for_each(|(index, c)| match c {
                        '0' => mask = mask & !(1 << index),
                        '1' => mask = mask | 1 << index,
                        'X' => floating.push(index),
                        _ => {}
                    })
            }
            Op::Mem => {
                let mut addrs = vec![op.2 | mask];
                floating.iter().for_each(|index| {
                    for i in 0..addrs.len() {
                        addrs.push(addrs[i] ^ 1 << index);
                    }
                });
                for addr in addrs {
                    mem.insert(addr, op.1.parse::<u64>().unwrap());
                }
            }
        }
    }
    println!("{}", mem.iter().map(|(_k, v)| v).sum::<u64>());
}
