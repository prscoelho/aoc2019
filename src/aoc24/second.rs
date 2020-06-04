use std::collections::BTreeSet;

type Grid = BTreeSet<(i32, i32, i32)>;

pub fn read_input(input: &str) -> Grid {
    let mut grid = Grid::new();

    //let mut idx = 0;
    for (row, line) in input.lines().enumerate() {
        for (column, c) in line.trim().chars().enumerate() {
            if c == '#' {
                grid.insert((0, row as i32, column as i32));
            }
        }
    }
    grid
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn flip(&self) -> Direction {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }

    fn move_tile(&self, row: i32, col: i32) -> (i32, i32) {
        match self {
            Direction::Left => (row, col - 1),
            Direction::Right => (row, col + 1),
            Direction::Up => (row - 1, col),
            Direction::Down => (row + 1, col),
        }
    }
}

fn count_edge(grid: &Grid, depth: i32, side: Direction) -> u32 {
    let mut result = 0;
    match side {
        Direction::Left => {
            for row in 0..5 {
                if grid.contains(&(depth, row, 0)) {
                    result += 1;
                }
            }
        }
        Direction::Right => {
            for row in 0..5 {
                if grid.contains(&(depth, row, 4)) {
                    result += 1;
                }
            }
        }
        Direction::Up => {
            for column in 0..5 {
                if grid.contains(&(depth, 0, column)) {
                    result += 1;
                }
            }
        }
        Direction::Down => {
            for column in 0..5 {
                if grid.contains(&(depth, 4, column)) {
                    result += 1;
                }
            }
        }
    }

    result
}

fn neighbours(grid: &Grid, depth: i32, row: i32, column: i32) -> u32 {
    let dirs = &[
        Direction::Up,
        Direction::Down,
        Direction::Left,
        Direction::Right,
    ];
    let mut result = 0;
    for dir in dirs.iter() {
        let (adj_row, adj_col) = dir.move_tile(row, column);
        if adj_col == 2 && adj_row == 2 {
            // center
            // count the number of infested tiles at depth + 1
            // on the edge of the opposite side of the direction we're moving to
            // (if we're moving to the left then we need to check the right edge of depth + 1)
            result += count_edge(grid, depth + 1, dir.flip());
        } else {
            // if row or column goes out of bound we should check
            // infested tile at the correct middle tile at depth - 1
            let tile = if adj_col < 0 || adj_col > 4 || adj_row < 0 || adj_row > 4 {
                // outer tile is calculated by moving from (2,2) in the direction that resulted in out of bounds move
                let (outer_row, outer_col) = dir.move_tile(2, 2);
                (depth - 1, outer_row, outer_col)
            } else {
                (depth, adj_row, adj_col)
            };

            if grid.contains(&tile) {
                result += 1;
            }
        }
    }
    result
}

pub fn step(grid: Grid) -> Grid {
    let mut next = Grid::new();
    let min_depth = grid.iter().next().unwrap().0 - 1;
    let max_depth = grid.iter().rev().next().unwrap().0 + 1;

    for depth in min_depth..=max_depth {
        for row in 0..5 {
            for col in 0..5 {
                if row == 2 && col == 2 {
                    continue;
                }
                let n = neighbours(&grid, depth, row, col);
                let infested = grid.contains(&(depth, row, col));
                match (infested, n) {
                    (true, 1) | (false, 1) | (false, 2) => {
                        // tile is infested on next grid
                        next.insert((depth, row, col));
                    }
                    _ => {}
                }
            }
        }
    }

    next
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn example() {
        let input = include_str!("example1");
        let mut grid = read_input(input);
        for _ in 0..10 {
            grid = step(grid);
        }
        assert_eq!(grid.iter().count(), 99);
    }
}
