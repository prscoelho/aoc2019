use std::collections::HashMap;

use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::usize;

use super::parse::Portal;

#[derive(Clone, Eq, PartialEq)]
struct State {
    depth: usize,
    cost: usize,
    node: Portal,
}

impl Ord for State {
    fn cmp(&self, other: &State) -> Ordering {
        other
            .depth
            .cmp(&self.depth)
            .then_with(|| other.cost.cmp(&self.cost))
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for State {
    fn partial_cmp(&self, other: &State) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// Dijkstra's shortest path algorithm.
pub fn shortest_path_depth(
    graph: HashMap<Portal, HashMap<Portal, usize>>,
    start: Portal,
    goal: Portal,
) -> Option<usize> {
    // the difference here is we want to keep track of best cost
    // for the same portal at different depths
    // being at a node with more steps taken but less
    // depth should/could be the necessary step
    let mut dist: HashMap<(usize, Portal), usize> = HashMap::new();

    let mut heap = BinaryHeap::new();

    // We're at `start`, with a zero cost
    dist.insert((0, start.clone()), 0);
    heap.push(State {
        depth: 0,
        cost: 0,
        node: start.clone(),
    });

    // Examine the frontier with lower cost nodes first (min-heap)
    while let Some(State { depth, cost, node }) = heap.pop() {
        // Alternatively we could have continued to find all shortest paths
        if node == goal && depth == 0 {
            return Some(cost);
        }

        let current_key = (depth, node.clone());
        // Important as we may have already found a better way
        if let Some(&dist_cost) = dist.get(&current_key) {
            if cost > dist_cost {
                continue;
            }
        }

        for (next_node, &travel_cost) in &graph[&node] {
            // at depth 0 no Outer portals can be used
            // we identify portal usage by looking at the travel cost
            // walking from a portal to a portal will cost at
            // a minimum 4 steps if they're next to each other
            if travel_cost == 1 && depth == 0 {
                if let Portal::Outer(_) = node {
                    continue;
                }
            }

            let next_depth = if travel_cost == 1 {
                match node {
                    // this subtraction is safe as the previous if block prevents
                    // the condition that results in overflow
                    // otherwise we would use depth.wrapping_sub(1)
                    Portal::Outer(_) => depth - 1,
                    Portal::Inner(_) => depth + 1,
                }
            } else {
                depth
            };
            let next = State {
                depth: next_depth,
                cost: cost + travel_cost,
                node: next_node.clone(),
            };

            // If so, add it to the frontier and continue
            let dist_next = dist
                .entry((next_depth, next_node.clone()))
                .or_insert(usize::MAX);
            if next.cost < *dist_next {
                // Relaxation, we have now found a better way
                *dist_next = next.cost;
                heap.push(next);
            }
        }
    }
    // Goal not reachable
    None
}
