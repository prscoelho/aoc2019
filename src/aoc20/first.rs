use std::collections::HashMap;

use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::usize;

use super::parse::Portal;

#[derive(Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    node: Portal,
}

impl Ord for State {
    fn cmp(&self, other: &State) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for State {
    fn partial_cmp(&self, other: &State) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// Dijkstra's shortest path algorithm.
// adapted from https://doc.rust-lang.org/std/collections/binary_heap/index.html
pub fn shortest_path(
    graph: HashMap<Portal, HashMap<Portal, usize>>,
    start: Portal,
    goal: Portal,
) -> Option<usize> {
    // dist[node] = current shortest distance from `start` to `node`
    let mut dist: HashMap<Portal, usize> =
        graph.iter().map(|(k, _)| (k.clone(), usize::MAX)).collect();

    let mut heap = BinaryHeap::new();

    // We're at `start`, with a zero cost
    dist.insert(start.clone(), 0);
    heap.push(State {
        cost: 0,
        node: start,
    });

    // Examine the frontier with lower cost nodes first (min-heap)
    while let Some(State { cost, node }) = heap.pop() {
        // Alternatively we could have continued to find all shortest paths
        if node == goal {
            return Some(cost);
        }

        // Important as we may have already found a better way
        if cost > dist[&node] {
            continue;
        }

        // For each node we can reach, see if we can find a way with
        // a lower cost going through this node
        for (next_node, travel_cost) in &graph[&node] {
            let next = State {
                cost: cost + travel_cost,
                node: next_node.clone(),
            };

            // If so, add it to the frontier and continue
            if next.cost < dist[&next.node] {
                // Relaxation, we have now found a better way
                dist.insert(next.node.clone(), next.cost);
                heap.push(next);
            }
        }
    }

    // Goal not reachable
    None
}
