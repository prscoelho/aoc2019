use std::collections::HashMap;

fn read_orbit(input: &str) -> HashMap<String, String> {
    let mut edges = HashMap::new();

    for line in input.lines() {
        let mut split_it = line.split(')');
        let left = split_it.next().unwrap();
        let right = split_it.next().unwrap();

        // right orbits left
        edges.insert(right.to_owned(), left.to_owned());
    }

    edges
}

fn count_orbits(edges: HashMap<String, String>) -> i32 {
    let mut memo: HashMap<String, i32> = HashMap::new();
    for edge in edges.keys() {
        visit(&edges, edge, &mut memo);
    }
    let mut total = 0;
    for depth in memo.values() {
        total += depth;
    }
    total
}

fn visit(edges: &HashMap<String, String>, from: &str, memo: &mut HashMap<String, i32>) -> i32 {
    if let Some(depth) = memo.get(from) {
        *depth
    } else if let Some(to) = edges.get(from) {
        let depth = 1 + visit(edges, to, memo);
        memo.insert(from.to_owned(), depth);
        depth
    } else {
        0
    }
}

pub fn solve_first(input: &str) -> i32 {
    let edges = read_orbit(input);
    count_orbits(edges)
}

fn paths(edges: &HashMap<String, String>, start: &str) -> Vec<String> {
    let mut result = Vec::new();

    let mut curr = start;

    while let Some(from) = edges.get(curr) {
        result.push(from.to_owned());
        curr = from;
    }
    result
}

pub fn solve_second(input: &str) -> i32 {
    let edges = read_orbit(input);
    let mut paths_you = paths(&edges, "YOU");
    let mut paths_san = paths(&edges, "SAN");

    while paths_you.last() == paths_san.last() {
        paths_you.pop();
        paths_san.pop();
    }
    (paths_san.len() + paths_you.len()) as i32
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_read() {
        let input = "A)B\nB)C\n";
        let edges = read_orbit(input);
        assert_eq!(edges.get("B"), Some(&"A".to_owned()));
        assert_eq!(edges.get("C"), Some(&"B".to_owned()));
    }

    #[test]
    fn test_count() {
        let input = r"COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L";
        let edges = read_orbit(input);
        assert_eq!(count_orbits(edges), 42);
    }
}
