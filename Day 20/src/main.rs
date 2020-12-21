use std::collections::HashMap;
use std::io::Read;

fn main() {
    let sea_monster = vec![
        "                  # ",
        "#    ##    ##    ###",
        " #  #  #  #  #  #   ",
    ];
    let mut stdin = std::io::stdin();
    let mut input = String::default();
    let mut boundaries = HashMap::new();
    let mut tiles = HashMap::new();
    let mut tile_contents = HashMap::new();

    stdin.read_to_string(&mut input).unwrap();
    input.trim().split("\r\n\r\n").for_each(|block| {
        let mut lines = block.lines();
        let mut tileId = String::from(lines.next().unwrap().trim().split(" ").nth(1).unwrap());
        tileId.pop();
        let mut left = String::new();
        let mut right = String::new();
        let mut top = String::from(lines.next().unwrap().trim());
        left.push(top.chars().nth(0).unwrap());
        right.push(top.chars().nth(top.len() - 1).unwrap());
        let mut bottom = String::new();
        let mut contents = vec![top.clone().chars().collect::<Vec<_>>()];
        lines.for_each(|line| {
            contents.push(String::from(line.trim()).chars().collect());
            bottom = String::from(line.trim());
            left.push(bottom.chars().nth(0).unwrap());
            right.push(bottom.chars().nth(bottom.len() - 1).unwrap());
        });
        tile_contents.insert(tileId.clone(), contents);
        tiles.insert(
            tileId.clone(),
            vec![top.clone(), bottom.clone(), left.clone(), right.clone()],
        );
        if !boundaries.contains_key(&top) {
            boundaries.insert(top, vec![tileId.clone()]);
        } else {
            boundaries.get_mut(&top).unwrap().push(tileId.clone());
        }
        if !boundaries.contains_key(&bottom) {
            boundaries.insert(bottom, vec![tileId.clone()]);
        } else {
            boundaries.get_mut(&bottom).unwrap().push(tileId.clone());
        }
        if !boundaries.contains_key(&left) {
            boundaries.insert(left, vec![tileId.clone()]);
        } else {
            boundaries.get_mut(&left).unwrap().push(tileId.clone());
        }
        if !boundaries.contains_key(&right) {
            boundaries.insert(right, vec![tileId.clone()]);
        } else {
            boundaries.get_mut(&right).unwrap().push(tileId.clone());
        }
    });
    let mut ans = 1;
    for (id, boundary_list) in tiles.iter() {
        let adj: u64 = boundary_list
            .iter()
            .map(|boundary| {
                let reverse: String = boundary.chars().rev().collect();
                if boundaries.get(boundary).unwrap().len()
                    + boundaries.get(&reverse).unwrap_or(&Vec::new()).len()
                    > 1
                {
                    1
                } else {
                    0
                }
            })
            .sum();
        if adj == 2 {
            ans *= id.parse::<u64>().unwrap();
        }
    }
    println!("{}", ans);
}

fn match_string(a: &String, b: &str) -> bool {
    a.len() == b.len()
        && a.chars().zip(b.chars()).fold(true, |acc, (c, d)| {
            if d == '#' && c != '#' {
                return false;
            } else {
                return acc;
            }
        })
}
