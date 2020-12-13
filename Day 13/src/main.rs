use std::io::Read;

fn main() {
    let mut stdin = std::io::stdin();
    let mut input = String::default();

    stdin.read_to_string(&mut input).unwrap();
    let mut lines = input.lines();
    let time: u64 = lines.next().unwrap().trim().parse().unwrap();
    let times: Vec<u64> = lines
        .next()
        .unwrap()
        .trim()
        .split(",")
        .map(|s| if s == "x" { 0 } else { s.parse().unwrap() })
        .collect();
    let mut ans = time;
    let mut id = 0;
    for curr in &times {
        if *curr == 0 {
            continue;
        }
        if curr - (time % curr) < ans {
            ans = curr - (time % curr);
            id = *curr;
        }
    }
    println!("{}", ans * id);
    let mut curr = 1;
    let mut ans: u64 = 0;
    for i in 0..times.len() {
        let time = times[i];
        if time == 0 {
            continue;
        }
        while (ans + i as u64) % time != 0 {
            ans += curr;
        }
        curr *= time;
    }
    println!("{}", ans);
}
