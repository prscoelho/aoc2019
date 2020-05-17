use std::collections::{BTreeMap, HashMap, HashSet, VecDeque};

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Portal {
    Outer(String),
    Inner(String),
}

pub fn parse_input(input: &str) -> BTreeMap<(i32, i32), char> {
    let mut result = BTreeMap::new();
    let mut y = 0;
    for line in input.lines() {
        let mut x = 0;
        for c in line.chars() {
            result.insert((y, x), c);
            x += 1;
        }
        y += 1;
    }
    result
}

fn inside_box(position: (i32, i32), top_left: (i32, i32), bottom_right: (i32, i32)) -> bool {
    position.0 >= top_left.0
        && position.1 >= top_left.1
        && position.0 <= bottom_right.0
        && position.1 <= bottom_right.1
}

pub fn parse_nodes(map: &BTreeMap<(i32, i32), char>) -> HashMap<(i32, i32), Portal> {
    // we can split the maze into three rectangle areas:
    // outer  - empty area containing outer portal names
    // maze   - square containing wall or path values
    // inner  - square containing inner portal names
    // the idea is to scan the outer and inner edges of the maze for open tiles

    // outer bounds is from (0,0) to last coord
    let outer_bottom_right = *map.iter().last().unwrap().0;

    // maze is (2,2) to outer - 2
    let maze_top_left = (2, 2);
    let maze_bottom_right = (outer_bottom_right.0 - 2, outer_bottom_right.1 - 2);

    // find every inner position,
    // inner_top_left should be first element
    // and inner_bottom_right should be last element
    let mut inner_space = map.iter().filter(|(&pos, &c)| {
        inside_box(pos, maze_top_left, maze_bottom_right) && c != '#' && c != '.'
    });
    let inner_top_left = *inner_space.next().unwrap().0;
    let inner_bottom_right = *inner_space.rev().next().unwrap().0;

    let mut result = HashMap::new();

    // outside edge: inside_box(maze_top_left, maze_bottom_right) but not inside_box(maze_top_left + 1, maze_bottom_right - 1)
    for (&pos, &tile) in map.iter().filter(|(&pos, _)| {
        inside_box(pos, maze_top_left, maze_bottom_right)
            && !inside_box(
                pos,
                (maze_top_left.0 + 1, maze_top_left.1 + 1),
                (maze_bottom_right.0 - 1, maze_bottom_right.1 - 1),
            )
    }) {
        if tile == '.' {
            let name = portal_name(&map, pos);
            result.insert(pos, Portal::Outer(name));
        }
    }

    // inside edge: inside_box(inner_top_left - 1, inner_bottom_right + 1) but not inside_box(inner_top_left, inner_top_right)
    for (&pos, &tile) in map.iter().filter(|(&pos, _)| {
        inside_box(
            pos,
            (inner_top_left.0 - 1, inner_top_left.1 - 1),
            (inner_bottom_right.0 + 1, inner_bottom_right.1 + 1),
        ) && !inside_box(pos, inner_top_left, inner_bottom_right)
    }) {
        if tile == '.' {
            let name = portal_name(&map, pos);
            result.insert(pos, Portal::Inner(name));
        }
    }
    result
}

fn portal_name(map: &BTreeMap<(i32, i32), char>, pos: (i32, i32)) -> String {
    let dirs: &[(i32, i32)] = &[
        //left, up (reverse order)
        (-1, 0),
        (0, -1),
        // right, down (normal order)
        (1, 0),
        (0, 1),
    ];
    let mut result = Vec::with_capacity(2);
    for (idx, &dir) in dirs.iter().enumerate() {
        let next = (pos.0 + dir.0, pos.1 + dir.1);
        if let Some(&c1) = map.get(&next) {
            if c1.is_ascii_alphabetic() {
                result.push(c1);
                if let Some(&c2) = map.get(&(next.0 + dir.0, next.1 + dir.1)) {
                    result.push(c2);
                }
                if idx < 2 {
                    result.reverse();
                }
                break;
            }
        }
    }

    result.into_iter().collect()
}

pub fn parse_graph(
    map: &BTreeMap<(i32, i32), char>,
    nodes: &HashMap<(i32, i32), Portal>,
) -> HashMap<Portal, HashMap<Portal, usize>> {
    let mut result = HashMap::with_capacity(nodes.len());
    for (_, portal) in nodes.iter() {
        result.insert(portal.clone(), HashMap::new());
    }

    let dirs = &[(0, -1), (0, 1), (-1, 0), (1, 0)];
    for (portal_pos, portal) in nodes.iter() {
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        queue.push_back((0usize, *portal_pos));
        visited.insert(*portal_pos);

        while let Some((steps, pos)) = queue.pop_front() {
            for dir in dirs.iter() {
                let next = (pos.0 + dir.0, pos.1 + dir.1);

                if visited.contains(&next) {
                    continue;
                }

                if let Some(connected_portal) = nodes.get(&next) {
                    result
                        .get_mut(portal)
                        .unwrap()
                        .entry(connected_portal.clone())
                        .or_insert(steps + 1);

                    continue;
                }

                if let Some(&c) = map.get(&next) {
                    if c == '.' {
                        queue.push_back((steps + 1, next));
                        visited.insert(next);
                    }
                }
            }
        }
    }

    let portals: HashSet<Portal> = result.clone().into_iter().map(|(k, _)| k).collect();

    for (portal, adj) in result.iter_mut() {
        let other = match portal {
            Portal::Inner(name) => Portal::Outer(name.clone()),
            Portal::Outer(name) => Portal::Inner(name.clone()),
        };

        if portals.contains(&other) {
            adj.insert(other, 1);
        }
    }

    result
}

#[test]
fn portal() {
    let input = include_str!("example1");
    let map = parse_input(input);
    assert_eq!(portal_name(&map, (2, 9)), "AA");

    assert_eq!(portal_name(&map, (8, 2)), "BC");
}
