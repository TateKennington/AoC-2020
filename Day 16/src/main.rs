use std::collections::HashMap;
use std::io::Read;

fn process(curr: usize, potentials: &Vec<Vec<bool>>, seen: &mut HashMap<usize, usize>) -> bool {
    if curr >= potentials.len() {
        return true;
    }
    for i in 0..potentials[curr].len() {
        if potentials[curr][i] && !seen.contains_key(&i) {
            seen.insert(i, curr);
            if process(curr + 1, potentials, seen) {
                return true;
            }
            seen.remove(&i);
        }
    }
    return false;
}

fn main() {
    let mut stdin = std::io::stdin();
    let mut input = String::default();

    stdin.read_to_string(&mut input).unwrap();
    let mut groups = input.split("\r\n\r\n");
    let mut max = 0;
    let regex = regex::Regex::new(
        r"(?P<field>.*): (?P<lower1>\d+)-(?P<upper1>\d+) or (?P<lower2>\d+)-(?P<upper2>\d+)",
    )
    .unwrap();
    let ranges: Vec<_> = groups
        .next()
        .unwrap()
        .lines()
        .map(|line| {
            let captures = regex.captures(line).unwrap();
            let res = (
                captures["lower1"].parse::<i32>().unwrap(),
                captures["upper1"].parse::<i32>().unwrap(),
                captures["lower2"].parse::<i32>().unwrap(),
                captures["upper2"].parse::<i32>().unwrap(),
                String::from(&captures["field"]),
            );
            max = max.max(res.0);
            max = max.max(res.1);
            max = max.max(res.2);
            max = max.max(res.3);
            res
        })
        .collect();
    let mut valid: Vec<bool> = Vec::with_capacity((max + 1) as usize);
    for _i in 0..=max {
        valid.push(false);
    }
    ranges.iter().for_each(|range| {
        for i in range.0..=range.1 {
            valid[i as usize] = true;
        }
        for i in range.2..=range.3 {
            valid[i as usize] = true;
        }
    });
    let curr_ticket: Vec<i32> = groups
        .next()
        .unwrap()
        .lines()
        .skip(1)
        .next()
        .unwrap()
        .trim()
        .split(",")
        .map(|field| field.parse().unwrap())
        .collect();
    let nearby: Vec<Vec<i32>> = groups
        .next()
        .unwrap()
        .lines()
        .skip(1)
        .map(|line| {
            line.trim()
                .split(',')
                .map(|num| num.parse().unwrap())
                .collect()
        })
        .collect();
    let mut ans = 0;
    let mut valids = Vec::with_capacity(nearby.len());
    for ticket in nearby {
        let mut is_valid = true;
        for field in &ticket {
            if *field > max || !valid[*field as usize] {
                ans += field;
                is_valid = false;
            }
        }
        if is_valid {
            valids.push(ticket);
        }
    }
    let mut potentials = Vec::with_capacity(ranges.len());
    for i in 0..ranges.len() {
        potentials.push(Vec::with_capacity(ranges.len()));
        for _j in 0..ranges.len() {
            potentials[i].push(true);
        }
    }
    for ticket in valids {
        for (i, field) in ticket.iter().enumerate() {
            ranges
                .iter()
                .map(|range| {
                    *field >= range.0 && *field <= range.1 || *field >= range.2 && *field <= range.3
                })
                .enumerate()
                .for_each(|(j, val)| {
                    potentials[i][j] = potentials[i][j] && val;
                });
        }
    }
    println!("{}", ans);
    let mut res = HashMap::new();
    process(0, &potentials, &mut res);
    let mut ans: u64 = 1;
    res.iter().for_each(|(field, index)| {
        if ranges[*field].4.contains("departure") {
            ans *= curr_ticket[*index] as u64;
        }
    });
    println!("{}", ans);
}
