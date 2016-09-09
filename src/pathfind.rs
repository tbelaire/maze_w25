use std::cmp::Ordering;
use std::collections::BinaryHeap;


use grid::Grid;
use direction::Direction;
use direction::Direction::*;
use maze::Maze;
use posn::Posn;
use tile::Tile;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct Node {
    cost: Option<i32>,
    from: Direction,
}

impl Node {
    fn new() -> Node {
        Node {
            cost: None,
            from: Direction::North,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct HeapEntry {
    cost: i32,
    pos: Posn,
    from: Direction,
}

impl PartialOrd for HeapEntry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

impl Ord for HeapEntry {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}


pub fn pathfind(maze: &Maze, player: Posn) -> Vec<Posn> {
    let mut search_nodes = Grid::new(maze.map
        .iter()
        .map(|row| {
            row.iter()
                .map(|_| Node::new())
                .collect()
        })
        .collect());

    let mut heap = BinaryHeap::new();
    search_nodes[player].cost = None;

    heap.push(HeapEntry {
        cost: 0,
        pos: player,
        from: North,
    });
    let mut exit_pos: Option<Posn> = None;

    while let Some(HeapEntry { cost, pos, from }) = heap.pop() {
        trace!("Considering cell {:?}", pos);
        assert!(maze.in_bounds(&pos));
        if maze[&pos] == Tile::Exit {
            trace!("Found the exit at {:?}", pos);
            search_nodes[pos].from = from;
            exit_pos = Some(pos);
            break;
        }

        let node = search_nodes[pos];
        if let Some(old_cost) = node.cost {
            if old_cost <= cost {
                trace!("Old cost of {:?} was better than new cost {:?}",
                       old_cost,
                       cost);
                continue;
            }
        }
        search_nodes[pos].cost = Some(cost);
        search_nodes[pos].from = from;

        for dir in &[North, East, South, West] {
            let next_pos = pos + dir.numeric();
            // Also handles out of bounds.
            if maze[&next_pos] == Tile::Wall {
                continue;
            }
            assert!(maze.in_bounds(&next_pos));
            heap.push(HeapEntry {
                pos: next_pos,
                cost: cost + 1,
                from: dir.flip(),
            });
        }
    }

    if exit_pos == None {
        warn!("Couldn't find a path");
        return vec![];
    }
    let mut path = vec![];
    let mut curr_pos = exit_pos.unwrap();
    trace!("Path:");
    while curr_pos != player {
        path.push(curr_pos);
        trace!("{:?}", curr_pos);
        let next_dir = search_nodes[curr_pos].from;
        trace!("Stepping to the {:?}", next_dir);
        curr_pos = curr_pos + next_dir.numeric();
    }
    path.reverse();
    path
}

// fn shortest_path(adj_list: &Vec<Vec<Edge>>, start: usize, goal: usize) -> Option<usize> {
//     // dist[node] = current shortest distance from `start` to `node`
//     let mut dist: Vec<_> = (0..adj_list.len()).map(|_| usize::MAX).collect();


//     // We're at `start`, with a zero cost
//     dist[start] = 0;
//     heap.push(State { cost: 0, position: start });

//     // Examine the frontier with lower cost nodes first (min-heap)
//     while let Some(State { cost, position }) = heap.pop() {
//         // Alternatively we could have continued to find all shortest paths
//         if position == goal { return Some(cost); }

//         // Important as we may have already found a better way
//         if cost > dist[position] { continue; }

//         // For each node we can reach, see if we can find a way with
//         // a lower cost going through this node
//         for edge in &adj_list[position] {
//             let next = State { cost: cost + edge.cost, position: edge.node };

//             // If so, add it to the frontier and continue
//             if next.cost < dist[next.position] {
//                 heap.push(next);
//                 // Relaxation, we have now found a better way
//                 dist[next.position] = next.cost;
//             }
//         }
//     }

//     // Goal not reachable
//     None
// }
