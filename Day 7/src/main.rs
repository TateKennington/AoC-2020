use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;

fn bag_count(color: &str, contains: &HashMap<String, Vec<(String, i32)>>) -> i32 {
    let mut res = 0;
    for (dest, count) in &contains[color] {
        res += count + count * bag_count(&dest, &contains);
    }
    return res;
}

fn main() {
    let stdin = std::io::stdin();
    let mut input = String::default();
    let mut contained_by = HashMap::new();
    let mut contains = HashMap::new();
    let mut ans = 0;

    let mut bytes = stdin.read_line(&mut input).unwrap();
    while bytes > 0 {
        let regex = Regex::new(r"(^(.*) bags contain)|(\d) ([^.,]*) bags?(,|.)").unwrap();
        let mut captures = regex.captures_iter(&input);
        let color = String::from(captures.next().unwrap().get(2).unwrap().as_str());
        if !contained_by.contains_key(&color) {
            contained_by.insert(color.clone(), Vec::default());
        }
        if !contains.contains_key(&color) {
            contains.insert(color.clone(), Vec::default());
        }
        for cap in captures {
            let count: i32 = cap.get(3).unwrap().as_str().parse().unwrap();
            let dest_color = String::from(cap.get(4).unwrap().as_str());
            if !contained_by.contains_key(&dest_color) {
                contained_by.insert(dest_color.clone(), Vec::default());
            }
            if !contains.contains_key(&dest_color) {
                contains.insert(dest_color.clone(), Vec::default());
            }
            contained_by
                .get_mut(&dest_color)
                .unwrap()
                .push(color.clone());
            contains
                .get_mut(&color)
                .unwrap()
                .push((dest_color.clone(), count));
        }
        input = String::default();
        bytes = stdin.read_line(&mut input).unwrap();
    }
    let mut seen = HashSet::new();
    let mut queue = vec!["shiny gold"];
    while !queue.is_empty() {
        let curr = queue.pop().unwrap();
        if seen.contains(&curr) {
            continue;
        }
        seen.insert(curr);
        ans += 1;
        for adj in &contained_by[curr] {
            if seen.contains(&adj[..]) {
                continue;
            }
            queue.push(adj);
        }
    }
    println!("{}", ans - 1);
    println!("{}", bag_count("shiny gold", &contains));
}
