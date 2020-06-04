mod first;
mod second;

use std::collections::BTreeSet;

pub fn solve_first(input: &str) -> u32 {
    let mut states = BTreeSet::new();

    let mut current = first::read_input(input);

    // insert returns false when value is already in the set
    while states.insert(current) != false {
        current = first::step(current);
    }
    current
}

pub fn solve_second(input: &str) -> u32 {
    let mut grid = second::read_input(input);

    for _ in 0..200 {
        grid = second::step(grid);
    }
    grid.len() as u32
}
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example() {
        let input = include_str!("example1");
        assert_eq!(solve_first(input), 2129920);
    }

    #[test]
    fn first() {
        let input = include_str!("input");
        assert_eq!(solve_first(input), 1151290);
    }
    #[test]
    fn second() {
        let input = include_str!("input");
        assert_eq!(solve_second(input), 1953);
    }
}
