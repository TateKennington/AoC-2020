use std::io::Read;

fn can_see(pos: (i32, i32), dir: (i32, i32), grid: &Vec<Vec<char>>) -> i32 {
    let mut pos = (pos.0 + dir.0, pos.1 + dir.1);
    while pos.0 >= 0
        && (pos.0 as usize) < grid.len()
        && pos.1 >= 0
        && (pos.1 as usize) < grid[pos.0 as usize].len()
    {
        if grid[pos.0 as usize][pos.1 as usize] == 'L' {
            return 0;
        }
        if grid[pos.0 as usize][pos.1 as usize] == '#' {
            return 1;
        }
        pos = (pos.0 + dir.0, pos.1 + dir.1);
    }
    return 0;
}

fn is_adj(pos: (i32, i32), dir: (i32, i32), grid: &Vec<Vec<char>>) -> i32 {
    let pos = (pos.0 + dir.0, pos.1 + dir.1);
    if pos.0 >= 0
        && (pos.0 as usize) < grid.len()
        && pos.1 >= 0
        && (pos.1 as usize) < grid[pos.0 as usize].len()
    {
        if grid[pos.0 as usize][pos.1 as usize] == 'L' {
            return 0;
        }
        if grid[pos.0 as usize][pos.1 as usize] == '#' {
            return 1;
        }
    }
    return 0;
}

fn main() {
    let mut stdin = std::io::stdin();
    let mut input = String::default();

    stdin.read_to_string(&mut input).unwrap();
    let mut grid = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let mut changed = true;
    while changed {
        changed = false;
        let mut next = Vec::new();
        for i in 0..grid.len() {
            next.push(Vec::new());
            for j in 0..grid[i].len() {
                if grid[i][j] == '.' {
                    next[i].push(grid[i][j]);
                    continue;
                }
                let pos = (i as i32, j as i32);
                let adj = is_adj(pos, (1, 0), &grid)
                    + is_adj(pos, (-1, 0), &grid)
                    + is_adj(pos, (0, 1), &grid)
                    + is_adj(pos, (0, -1), &grid)
                    + is_adj(pos, (1, 1), &grid)
                    + is_adj(pos, (1, -1), &grid)
                    + is_adj(pos, (-1, 1), &grid)
                    + is_adj(pos, (-1, -1), &grid);

                if grid[i][j] == 'L' && adj == 0 {
                    next[i].push('#');
                    changed = true;
                } else if grid[i][j] == '#' && adj >= 4 {
                    next[i].push('L');
                    changed = true;
                } else {
                    next[i].push(grid[i][j]);
                }
            }
        }
        grid = next;
    }
    println!(
        "Part 1: {}",
        grid.iter()
            .map(|row| row.iter().filter(|c| **c == '#').count())
            .sum::<usize>()
    );
    let mut grid = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let mut changed = true;
    while changed {
        changed = false;
        let mut next = Vec::new();
        for i in 0..grid.len() {
            next.push(Vec::new());
            for j in 0..grid[i].len() {
                if grid[i][j] != 'L' && grid[i][j] != '#' {
                    next[i].push(grid[i][j]);
                    continue;
                }
                let pos = (i as i32, j as i32);
                let adj = can_see(pos, (1, 0), &grid)
                    + can_see(pos, (-1, 0), &grid)
                    + can_see(pos, (0, 1), &grid)
                    + can_see(pos, (0, -1), &grid)
                    + can_see(pos, (1, 1), &grid)
                    + can_see(pos, (1, -1), &grid)
                    + can_see(pos, (-1, 1), &grid)
                    + can_see(pos, (-1, -1), &grid);
                if grid[i][j] == 'L' && adj == 0 {
                    next[i].push('#');
                    changed = true;
                } else if grid[i][j] == '#' && adj >= 5 {
                    next[i].push('L');
                    changed = true;
                } else {
                    next[i].push(grid[i][j]);
                }
            }
        }
        grid = next;
    }
    println!(
        "Part 2: {}",
        grid.iter()
            .map(|row| row.iter().filter(|c| **c == '#').count())
            .sum::<usize>()
    );
}
