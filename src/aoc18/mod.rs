use std::cmp::Ordering;
use std::collections::{BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Coordinate(i32, i32);

impl Coordinate {
    fn neighbours(&self) -> [Coordinate; 4] {
        [
            Coordinate(self.0 - 1, self.1),
            Coordinate(self.0 + 1, self.1),
            Coordinate(self.0, self.1 - 1),
            Coordinate(self.0, self.1 + 1),
        ]
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Tile {
    Wall,
    Empty,
    Node(char),
}

fn parse_grid(input: &str) -> HashMap<Coordinate, Tile> {
    let mut grid = HashMap::new();
    let mut height = 0;
    for line in input.trim().lines() {
        let mut width = 0;
        for c in line.chars() {
            let tile = match c {
                '#' => Tile::Wall,
                '.' => Tile::Empty,
                _ => Tile::Node(c),
            };
            grid.insert(Coordinate(width, height), tile);
            width += 1;
        }
        height += 1;
    }
    grid
}

// build a graph from a grid
fn graph(grid: &HashMap<Coordinate, Tile>) -> HashMap<char, HashMap<char, usize>> {
    let mut graph = HashMap::new();
    for (coord, tile) in grid.iter() {
        if let Tile::Node(c) = tile {
            let pos_edges = reachable_from(grid, *coord);
            graph.insert(*c, pos_edges);
        }
    }

    graph
}

// returns vertices reachable from a coordinate
fn reachable_from(grid: &HashMap<Coordinate, Tile>, coord: Coordinate) -> HashMap<char, usize> {
    let mut visited = HashSet::new();
    let mut result = HashMap::new();

    let mut queue = VecDeque::new();
    queue.push_back((coord, 0));

    visited.insert(coord);
    while let Some((current, steps)) = queue.pop_front() {
        for neighbour in &current.neighbours() {
            if let Some(tile) = grid.get(neighbour) {
                if !visited.contains(neighbour) {
                    visited.insert(*neighbour);
                    match tile {
                        Tile::Empty => {
                            queue.push_back((*neighbour, steps + 1));
                        }
                        Tile::Node(c) => {
                            result.insert(*c, steps + 1);
                        }
                        Tile::Wall => {}
                    }
                }
            }
        }
    }
    result
}
#[derive(PartialEq, Eq)]
struct State {
    steps: usize,
    node: char,
    keys: BTreeSet<char>,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .steps
            .cmp(&self.steps)
            .then(self.keys.len().cmp(&other.keys.len()))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn search(graph: HashMap<char, HashMap<char, usize>>, start: char) -> usize {
    let mut priority_queue = BinaryHeap::new();
    let key_count = graph.iter().filter(|(k, _)| k.is_lowercase()).count();

    // keep track of best cost at (robot_position, keys collected)
    let mut distances: HashMap<(char, BTreeSet<char>), usize> = HashMap::new();
    distances.insert((start, BTreeSet::new()), 0);

    let start = State {
        steps: 0,
        node: start,
        keys: BTreeSet::new(),
    };

    priority_queue.push(start);

    // search keys cache, avoid recomputing search keys for the same (position, keys collected)
    let mut cache: HashMap<(char, BTreeSet<char>), Vec<(char, usize)>> = HashMap::new();

    while let Some(current) = priority_queue.pop() {
        if current.keys.len() == key_count {
            return current.steps;
        }

        if let Some(&best_steps) = distances.get(&(current.node, current.keys.clone())) {
            if current.steps > best_steps {
                continue;
            }
        }

        let cache_key = (current.node, current.keys.clone());

        let cached_entry = cache
            .entry(cache_key)
            .or_insert_with(|| search_keys(&graph, &current.keys, current.node));

        for &(next_node, cost) in cached_entry.iter() {
            let mut next_keys = current.keys.clone();
            next_keys.insert(next_node);
            let next_steps = current.steps + cost;

            let distances_entry = distances
                .entry((next_node, next_keys.clone()))
                .or_insert(usize::max_value());

            if next_steps < *distances_entry {
                *distances_entry = next_steps;

                let next_state = State {
                    steps: current.steps + cost,
                    node: next_node,
                    keys: next_keys,
                };

                priority_queue.push(next_state);
            }
        }
    }
    // no path found
    usize::max_value()
}

#[derive(PartialEq, Eq)]
struct DijkstraState {
    cost: usize,
    current: char,
}

impl Ord for DijkstraState {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.current.cmp(&other.current))
    }
}

impl PartialOrd for DijkstraState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// adapted from https://doc.rust-lang.org/std/collections/binary_heap/index.html
// dijkstra search for reachable new keys from start node
fn search_keys(
    graph: &HashMap<char, HashMap<char, usize>>,
    keys: &BTreeSet<char>,
    start: char,
) -> Vec<(char, usize)> {
    // dist[node] = current shortest distance from `start` to `node`
    let mut dist = HashMap::new();
    for &key in graph.keys() {
        dist.insert(key, usize::max_value());
    }

    let mut heap = BinaryHeap::new();

    *dist.get_mut(&start).unwrap() = 0;
    heap.push(DijkstraState {
        cost: 0,
        current: start,
    });
    // keep track of which new keys we can reach
    let mut reach = HashSet::new();

    while let Some(DijkstraState { cost, current }) = heap.pop() {
        if current.is_lowercase() && !keys.contains(&current) {
            reach.insert(current);
            continue;
        }

        // Important as we may have already found a better way
        if cost > dist[&current] {
            continue;
        }

        for (&next_node, &next_cost) in graph.get(&current).unwrap().iter() {
            // check if we have permission to pass
            if next_node.is_uppercase() && !keys.contains(&next_node.to_ascii_lowercase()) {
                continue;
            }

            let next = DijkstraState {
                cost: cost + next_cost,
                current: next_node,
            };

            if next.cost < dist[&next_node] {
                dist.insert(next_node, next.cost);
                heap.push(next);
            }
        }
    }
    // return a tuple of (new keys, cost to reach)
    reach.into_iter().map(|node| (node, dist[&node])).collect()
}

pub fn solve_first(input: &str) -> usize {
    let grid = parse_grid(input);
    let graph = graph(&grid);

    search(graph, '@')
}

// modify grid to split map into 4 sections
// add 4 robots on each section
fn four_robots(grid: &mut HashMap<Coordinate, Tile>) {
    let robot_coord = grid
        .iter()
        .find(|(_, &v)| v == Tile::Node('@'))
        .map(|(k, _)| k.clone())
        .unwrap();

    grid.insert(robot_coord, Tile::Wall);
    for &neighbour in &robot_coord.neighbours() {
        grid.insert(neighbour, Tile::Wall);
    }
    grid.insert(
        Coordinate(robot_coord.0 - 1, robot_coord.1 - 1),
        Tile::Node('@'),
    );
    grid.insert(
        Coordinate(robot_coord.0 - 1, robot_coord.1 + 1),
        Tile::Node('='),
    );

    grid.insert(
        Coordinate(robot_coord.0 + 1, robot_coord.1 + 1),
        Tile::Node('%'),
    );
    grid.insert(
        Coordinate(robot_coord.0 + 1, robot_coord.1 - 1),
        Tile::Node('$'),
    );
}

fn search_four(graph: HashMap<char, HashMap<char, usize>>) -> usize {
    let mut priority_queue = BinaryHeap::new();
    let key_count = graph.iter().filter(|(k, _)| k.is_lowercase()).count();

    // keep track of best cost at (robot_positions, keys collected)
    let mut distances: HashMap<([char; 4], BTreeSet<char>), usize> = HashMap::new();
    let robots = ['@', '=', '%', '$'];

    distances.insert((robots.clone(), BTreeSet::new()), 0);

    let start = FourState {
        steps: 0,
        robots: robots,
        keys: BTreeSet::new(),
    };

    priority_queue.push(start);

    // search keys cache, avoid recomputing search keys for the same (position, keys collected)
    let mut cache: HashMap<(char, BTreeSet<char>), Vec<(char, usize)>> = HashMap::new();

    while let Some(current) = priority_queue.pop() {
        if current.keys.len() == key_count {
            return current.steps;
        }

        if let Some(&best_steps) = distances.get(&(current.robots, current.keys.clone())) {
            if current.steps > best_steps {
                continue;
            }
        }

        for (robot_number, &robot_location) in current.robots.iter().enumerate() {
            let cache_key = (robot_location, current.keys.clone());

            let cached_entry = cache
                .entry(cache_key)
                .or_insert_with(|| search_keys(&graph, &current.keys, robot_location));

            for &(next_node, cost) in cached_entry.iter() {
                let mut next_keys = current.keys.clone();
                next_keys.insert(next_node);

                let mut next_robots = current.robots.clone();
                next_robots[robot_number] = next_node;

                let next_steps = current.steps + cost;

                let distances_entry = distances
                    .entry((next_robots.clone(), next_keys.clone()))
                    .or_insert(usize::max_value());

                if next_steps < *distances_entry {
                    *distances_entry = next_steps;
                    let next_state = FourState {
                        steps: next_steps,
                        robots: next_robots,
                        keys: next_keys,
                    };

                    priority_queue.push(next_state);
                }
            }
        }
    }
    // no path found
    usize::max_value()
}

#[derive(PartialEq, Eq)]
struct FourState {
    steps: usize,
    robots: [char; 4],
    keys: BTreeSet<char>,
}

impl Ord for FourState {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .steps
            .cmp(&self.steps)
            .then(self.keys.len().cmp(&other.keys.len()))
    }
}

impl PartialOrd for FourState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn solve_second(input: &str) -> usize {
    let mut grid = parse_grid(input);
    four_robots(&mut grid);
    let graph = graph(&grid);

    search_four(graph)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example1() {
        let input = include_str!("example1");
        let expected = 8;
        assert_eq!(solve_first(input), expected);
    }

    #[test]
    fn example2() {
        let input = include_str!("example2");
        let expected = 86;
        assert_eq!(solve_first(input), expected);
    }

    #[test]
    fn example3() {
        let input = include_str!("example3");
        let expected = 132;
        assert_eq!(solve_first(input), expected);
    }

    #[test]
    fn example4() {
        let input = include_str!("example4");
        let expected = 136;
        assert_eq!(solve_first(input), expected);
    }

    #[test]
    fn example5() {
        let input = include_str!("example5");
        let expected = 81;
        assert_eq!(solve_first(input), expected);
    }

    #[test]
    fn next_keys() {
        let input = include_str!("example3");
        let graph = graph(&parse_grid(input));
        let keys = BTreeSet::new();
        let result = search_keys(&graph, &keys, '@');
        assert!(result.contains(&('a', 2)));
        assert!(result.contains(&('b', 22)));
    }

    #[test]
    fn first() {
        let input = include_str!("input");
        let expected = 3832;
        assert_eq!(solve_first(input), expected);
    }

    #[test]
    fn second() {
        let input = include_str!("input");
        let expected = 1724;
        assert_eq!(solve_second(input), expected);
    }
}
