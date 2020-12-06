use std::collections::HashMap;

fn main() {
    let stdin = std::io::stdin();
    let mut input = String::default();
    let mut ans = 0;
    let mut ans2 = 0;

    let mut bytes = stdin.read_line(&mut input).unwrap();
    while bytes > 0 {
        let mut answers = HashMap::new();
        let mut count = 0;
        while input.trim().len() > 0 {
            input.trim().chars().for_each(|c| {
                answers.insert(c, answers.get(&c).unwrap_or(&0) + 1);
                ()
            });
            count += 1;
            input = String::default();
            stdin.read_line(&mut input).unwrap();
        }
        ans += answers.len();
        ans2 += answers.iter().filter(|(_k, v)| **v == count).count();
        bytes = stdin.read_line(&mut input).unwrap();
    }
    println!("{}", ans);
    println!("{}", ans2);
}
