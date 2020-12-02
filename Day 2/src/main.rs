fn main() {
    let mut stdin = std::io::stdin();
    let mut input = String::default();
    let mut ans = 0;
    let mut ans2 = 0;

    let re =
        regex::Regex::new(r"(?P<lower>\d+)-(?P<upper>\d+) (?P<character>.): (?P<password>\w*)")
            .unwrap();
    let mut bytes = stdin.read_line(&mut input).unwrap();
    while (bytes > 0) {
        let captures = re.captures(&input).unwrap();
        let lower: usize = captures["lower"].parse().unwrap();
        let higher: usize = captures["upper"].parse().unwrap();
        let character: &str = &captures["character"];
        let password = &captures["password"];

        let count = password.matches(character).count();
        if count >= lower && count <= higher {
            ans += 1;
        }

        let valid = password
            .match_indices(character)
            .fold(false, |acc, (index, _)| {
                if index == lower - 1 || index == higher - 1 {
                    !acc
                } else {
                    acc
                }
            });
        if valid {
            ans2 += 1;
        }
        input = String::default();
        bytes = stdin.read_line(&mut input).unwrap();
    }
    println!("Part 1: {}", ans);
    println!("Part 2: {}", ans2);
}
