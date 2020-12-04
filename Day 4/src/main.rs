use regex::Regex;
use std::collections::HashMap;

fn main() {
    let stdin = std::io::stdin();
    let mut input = String::default();
    let mut ans = 0;
    let mut ans2 = 0;

    let mut bytes = stdin.read_line(&mut input).unwrap();
    while bytes > 0 {
        let mut passport = HashMap::new();
        while input.trim().len() > 0 {
            input.trim().split(" ").for_each(|s| {
                passport.insert(
                    String::from(s.split(":").nth(0).unwrap()),
                    String::from(s.split(":").nth(1).unwrap()),
                );
            });
            input = String::default();
            stdin.read_line(&mut input).unwrap();
        }
        if passport.contains_key("byr")
            && passport.contains_key("iyr")
            && passport.contains_key("eyr")
            && passport.contains_key("hgt")
            && passport.contains_key("hcl")
            && passport.contains_key("ecl")
            && passport.contains_key("pid")
        {
            ans += 1;
            if valid_passport(passport) {
                ans2 += 1;
            }
        }

        input = String::default();
        bytes = stdin.read_line(&mut input).unwrap();
    }
    println!("Part 1: {}", ans);
    println!("Part 2: {}", ans2);
}

fn valid_passport(passport: HashMap<String, String>) -> bool {
    let byr: i32 = passport["byr"].parse().unwrap_or(-1);
    let iyr: i32 = passport["iyr"].parse().unwrap_or(-1);
    let eyr: i32 = passport["eyr"].parse().unwrap_or(-1);
    let height_regex = Regex::new(r"(?P<amount>\d+)(?P<unit>cm|in)").unwrap();
    let hcl_regex = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
    let ecl_regex = Regex::new(r"^amb|blu|brn|gry|grn|hzl|oth$").unwrap();
    let pid_regex = Regex::new(r"^\d{9}$").unwrap();
    if byr < 1920 || byr > 2002 {
        return false;
    }
    if iyr < 2010 || iyr > 2020 {
        return false;
    }
    if eyr < 2020 || eyr > 2030 {
        return false;
    }
    if let Some(captures) = height_regex.captures(&passport["hgt"]) {
        let hgt = captures["amount"].parse().unwrap_or(-1);
        if captures["unit"] == String::from("cm") && (hgt < 150 || hgt > 193) {
            return false;
        }
        if captures["unit"] == String::from("in") && (hgt < 59 || hgt > 76) {
            return false;
        }
    } else {
        return false;
    }
    if !hcl_regex.is_match(&passport["hcl"]) {
        return false;
    }
    if !ecl_regex.is_match(&passport["ecl"]) {
        return false;
    }
    if !pid_regex.is_match(&passport["pid"]) {
        return false;
    }
    return true;
}
