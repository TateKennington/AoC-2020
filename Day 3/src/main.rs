fn process(grid: &Vec<Vec<char>>, step: (usize, usize)) -> i32 {
    let mut ans = 0;
    let mut pos = (0, 0);
    while pos.1 < grid.len() {
        if grid[pos.1][pos.0] == '#' {
            ans += 1;
        }
        pos.0 = (pos.0 + step.0) % grid[0].len();
        pos.1 += step.1;
    }
    ans
}

fn main() {
    let stdin = std::io::stdin();
    let mut input = String::default();
    let mut grid: Vec<Vec<char>> = Vec::default();

    let mut bytes = stdin.read_line(&mut input).unwrap();
    while bytes > 0 {
        grid.push(input.chars().collect());

        input = String::default();
        bytes = stdin.read_line(&mut input).unwrap();
    }
    println!("Part 1: {}", process(&grid, (3, 1)));
    println!(
        "Part 2: {}",
        process(&grid, (1, 1))
            * process(&grid, (3, 1))
            * process(&grid, (5, 1))
            * process(&grid, (7, 1))
            * process(&grid, (1, 2))
    )
}
