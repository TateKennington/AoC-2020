use std::collections::HashSet;

fn find_id(pass: String) -> i32 {
    let bounds = pass.chars().fold((0, 128, 0, 8), |acc, c| match c {
        'F' => (acc.0, (acc.0 + acc.1) / 2, acc.2, acc.3),
        'B' => ((acc.0 + acc.1) / 2, acc.1, acc.2, acc.3),
        'R' => (acc.0, acc.1, (acc.2 + acc.3) / 2, acc.3),
        'L' => (acc.0, acc.1, acc.2, (acc.2 + acc.3) / 2),
        _ => (0, 0, 0, 0),
    });
    bounds.0 * 8 + bounds.2
}

fn main() {
    let stdin = std::io::stdin();
    let mut input = String::default();
    let mut max = 0;
    let mut min = 1000;
    let mut ids = HashSet::new();

    let mut bytes = stdin.read_line(&mut input).unwrap();
    while bytes > 0 {
        let id = find_id(input.trim().to_string());
        if id > max {
            max = id;
        }
        if id < min {
            min = id;
        }
        ids.insert(id);
        input = String::default();
        bytes = stdin.read_line(&mut input).unwrap();
    }
    println!("Part 1: {}", max);
    for i in min..max {
        if !ids.contains(&(i)) {
            println!("Part 2: {}", i);
            break;
        }
    }
}
