// https://adventofcode.com/2019/day/20
mod first;
mod parse;
mod second;

use parse::Portal;
use parse::{parse_graph, parse_input, parse_nodes};

use first::shortest_path;
use second::shortest_path_depth;

pub fn solve_first(input: &str) -> usize {
    let grid = parse_input(input);

    let nodes = parse_nodes(&grid);
    let graph = parse_graph(&grid, &nodes);
    if let Some(u) = shortest_path(
        graph,
        Portal::Outer(String::from("AA")),
        Portal::Outer(String::from("ZZ")),
    ) {
        u
    } else {
        usize::MAX
    }
}

pub fn solve_second(input: &str) -> usize {
    let grid = parse_input(input);

    let nodes = parse_nodes(&grid);
    let graph = parse_graph(&grid, &nodes);

    if let Some(u) = shortest_path_depth(
        graph,
        Portal::Outer(String::from("AA")),
        Portal::Outer(String::from("ZZ")),
    ) {
        u
    } else {
        usize::MAX
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn nodes() {
        use std::collections::HashMap;

        let input = include_str!("example1");
        let map = parse_input(input);
        let nodes = parse_nodes(&map);
        let mut expected = HashMap::new();
        expected.insert((2, 9), Portal::Outer(String::from("AA")));
        expected.insert((8, 2), Portal::Outer(String::from("BC")));
        expected.insert((13, 2), Portal::Outer(String::from("DE")));
        expected.insert((15, 2), Portal::Outer(String::from("FG")));
        expected.insert((16, 13), Portal::Outer(String::from("ZZ")));
        expected.insert((6, 9), Portal::Inner(String::from("BC")));
        expected.insert((10, 6), Portal::Inner(String::from("DE")));
        expected.insert((12, 11), Portal::Inner(String::from("FG")));

        assert_eq!(nodes, expected);
    }

    #[test]
    fn example1() {
        let input = include_str!("example1");
        assert_eq!(solve_first(input), 23);
    }
    #[test]
    fn example2() {
        let input = include_str!("example2");
        assert_eq!(solve_first(input), 58);
    }

    #[test]
    fn example3() {
        let input = include_str!("example3");
        assert_eq!(solve_second(input), 396);
    }

    #[test]
    fn first() {
        let input = include_str!("input");
        assert_eq!(solve_first(input), 588);
    }

    #[test]
    fn second() {
        let input = include_str!("input");
        assert_eq!(solve_second(input), 6834);
    }
}
