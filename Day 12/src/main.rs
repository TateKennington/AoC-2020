use std::io::Read;

fn main() {
    let mut stdin = std::io::stdin();
    let mut input = String::default();

    let regex = regex::Regex::new(r"(?P<move>(N|S|E|W|L|R|F))(?P<amount>\d+)").unwrap();
    stdin.read_to_string(&mut input).unwrap();
    let moves: Vec<_> = input
        .lines()
        .map(|line| {
            let captures = regex.captures(line.trim()).unwrap();
            return (
                String::from(&captures["move"]),
                captures["amount"].parse::<i32>().unwrap(),
            );
        })
        .collect();
    let mut x = 0;
    let mut y = 0;
    let mut facing: i32 = 0;
    let directions = [(1, 0), (0, -1), (-1, 0), (0, 1)];
    for curr in &moves {
        match &curr.0[..] {
            "N" => y += curr.1,
            "S" => y -= curr.1,
            "W" => x -= curr.1,
            "E" => x += curr.1,
            "F" => {
                x += directions[facing as usize].0 * curr.1;
                y += directions[facing as usize].1 * curr.1;
            }
            "R" => facing = (facing + (curr.1 / 90)) % 4,
            "L" => {
                facing = facing - (curr.1 / 90) % 4;
                if facing < 0 {
                    facing += 4;
                }
            }
            _ => {}
        }
    }
    println!("Part 1: {}", x.abs() + y.abs());
    let mut x = 0;
    let mut y = 0;
    let mut waypoint = (10, 1);
    for curr in &moves {
        match &curr.0[..] {
            "N" => waypoint.1 += curr.1,
            "S" => waypoint.1 -= curr.1,
            "W" => waypoint.0 -= curr.1,
            "E" => waypoint.0 += curr.1,
            "F" => {
                x += waypoint.0 * curr.1;
                y += waypoint.1 * curr.1;
            }
            "R" => {
                for _i in 0..curr.1 / 90 {
                    waypoint = (waypoint.1, -waypoint.0)
                }
            }
            "L" => {
                for _i in 0..curr.1 / 90 {
                    waypoint = (-waypoint.1, waypoint.0)
                }
            }
            _ => {}
        }
    }
    println!("Part 2: {}", x.abs() + y.abs());
}
