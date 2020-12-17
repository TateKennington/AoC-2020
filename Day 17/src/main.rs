use std::io::Read;
use std::marker::Copy;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum State {
    Active,
    Inactive,
}

fn main() {
    let mut stdin = std::io::stdin();
    let mut input = String::default();

    stdin.read_to_string(&mut input).unwrap();
    let mut grid: [[[State; 20]; 20]; 20] = [[[State::Inactive; 20]; 20]; 20];
    input.lines().enumerate().for_each(|(y, line)| {
        line.trim().chars().enumerate().for_each(|(x, cell)| {
            grid[x + 6][y + 6][10] = match cell {
                '.' => State::Inactive,
                _ => State::Active,
            };
        })
    });
    for _i in 0..6 {
        let mut next_grid: [[[State; 20]; 20]; 20] = [[[State::Inactive; 20]; 20]; 20];
        for x in 0..20 {
            for y in 0..20 {
                for z in 0..20 {
                    let mut adj = 0;
                    for dx in -1..=1 as i32 {
                        for dy in -1..=1 as i32 {
                            for dz in -1..=1 as i32 {
                                if dx == 0 && dy == 0 && dz == 0 {
                                    continue;
                                }
                                if x + dx > 0
                                    && x + dx < 20
                                    && y + dy > 0
                                    && y + dy < 20
                                    && z + dz > 0
                                    && z + dz < 20
                                    && grid[(x + dx) as usize][(y + dy) as usize][(z + dz) as usize]
                                        == State::Active
                                {
                                    adj += 1
                                }
                            }
                        }
                    }
                    if grid[x as usize][y as usize][z as usize] == State::Active
                        && adj != 2
                        && adj != 3
                    {
                        next_grid[x as usize][y as usize][z as usize] = State::Inactive;
                    } else if grid[x as usize][y as usize][z as usize] == State::Inactive
                        && adj == 3
                    {
                        next_grid[x as usize][y as usize][z as usize] = State::Active;
                    } else {
                        next_grid[x as usize][y as usize][z as usize] =
                            grid[x as usize][y as usize][z as usize]
                    }
                }
            }
        }
        grid = next_grid;
    }
    let mut ans = 0;
    for x in 0..20 {
        for y in 0..20 {
            for z in 0..20 {
                if grid[x][y][z] == State::Active {
                    ans += 1;
                }
            }
        }
    }
    println!("{}", ans);

    let mut grid: [[[[State; 20]; 20]; 20]; 20] = [[[[State::Inactive; 20]; 20]; 20]; 20];
    input.lines().enumerate().for_each(|(y, line)| {
        line.trim().chars().enumerate().for_each(|(x, cell)| {
            grid[x + 6][y + 6][10][10] = match cell {
                '.' => State::Inactive,
                _ => State::Active,
            };
        })
    });
    for _i in 0..6 {
        let mut next_grid: [[[[State; 20]; 20]; 20]; 20] = [[[[State::Inactive; 20]; 20]; 20]; 20];
        for x in 0..20 {
            for y in 0..20 {
                for z in 0..20 {
                    for w in 0..20 {
                        let mut adj = 0;
                        for dx in -1..=1 as i32 {
                            for dy in -1..=1 as i32 {
                                for dz in -1..=1 as i32 {
                                    for dw in -1..=1 as i32 {
                                        if dx == 0 && dy == 0 && dz == 0 && dw == 0 {
                                            continue;
                                        }
                                        if x + dx > 0
                                            && x + dx < 20
                                            && y + dy > 0
                                            && y + dy < 20
                                            && z + dz > 0
                                            && z + dz < 20
                                            && w + dw > 0
                                            && w + dw < 20
                                            && grid[(x + dx) as usize][(y + dy) as usize]
                                                [(z + dz) as usize]
                                                [(w + dw) as usize]
                                                == State::Active
                                        {
                                            adj += 1
                                        }
                                    }
                                }
                            }
                        }
                        if grid[x as usize][y as usize][z as usize][w as usize] == State::Active
                            && adj != 2
                            && adj != 3
                        {
                            next_grid[x as usize][y as usize][z as usize][w as usize] =
                                State::Inactive;
                        } else if grid[x as usize][y as usize][z as usize][w as usize]
                            == State::Inactive
                            && adj == 3
                        {
                            next_grid[x as usize][y as usize][z as usize][w as usize] =
                                State::Active;
                        } else {
                            next_grid[x as usize][y as usize][z as usize][w as usize] =
                                grid[x as usize][y as usize][z as usize][w as usize]
                        }
                    }
                }
            }
        }
        grid = next_grid;
    }
    let mut ans = 0;
    for x in 0..20 {
        for y in 0..20 {
            for z in 0..20 {
                for w in 0..20 {
                    if grid[x][y][z][w] == State::Active {
                        ans += 1;
                    }
                }
            }
        }
    }
    println!("{}", ans);
}
