use std::fs::read_to_string;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn opposite(&self) -> Direction {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }

    fn get_diff(&self) -> (isize, isize) {
        match self {
            Direction::Up => (-1, 0),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
            Direction::Right => (0, 1)
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    position: (usize, usize),
    direction: Direction,
    steps_direction: usize,
}

// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap
// instead of a max-heap.
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other.cost.cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
struct DistKey {
    position: (usize, usize),
    direction: Direction,
    steps_direction: usize,
}

// Dijkstra's shortest path algorithm.

// Start at `start` and use `dist` to track the current shortest distance
// to each node. This implementation isn't memory-efficient as it may leave duplicate
// nodes in the queue. It also uses `usize::MAX` as a sentinel value,
// for a simpler implementation.
fn shortest_path(field: &Vec<Vec<usize>>, start: (usize, usize), goal: (usize, usize)) -> Option<usize> {
    // dist[node] = current shortest distance from `start` to `node`
    let mut dist: HashMap<DistKey, usize> = HashMap::new();

    let mut heap = BinaryHeap::new();

    let directions = vec![Direction::Up, Direction::Down, Direction::Left, Direction::Right];

    // We're at `start`, with a zero cost
    dist.insert(DistKey { position: start, direction: Direction::Right, steps_direction: 0 }, 0);
    dist.insert(DistKey { position: start, direction: Direction::Down, steps_direction: 0 }, 0);
    heap.push(State { cost: 0, position: start, direction: Direction::Right, steps_direction: 0  });

    // Examine the frontier with lower cost nodes first (min-heap)
    while let Some(State { cost, position, direction, steps_direction }) = heap.pop() {
        // Alternatively we could have continued to find all shortest paths
        if position == goal { return Some(cost); }

        // Important as we may have already found a better way
        let dist_key = DistKey { position, direction, steps_direction };
        if dist.contains_key(&dist_key) && cost > dist[&dist_key] { continue; }

        // For each node we can reach, see if we can find a way with
        // a lower cost going through this node
        for dir in directions.iter().filter(|&d| *d != direction.opposite()) {
            let (diff_x, diff_y) = dir.get_diff();
            if position.0 as isize + diff_x < 0 
                    || position.1 as isize + diff_y < 0 
                    || position.0 as isize + diff_x >= field.len() as isize 
                    || position.1 as isize + diff_y >= field[0].len() as isize {
                continue;
            }

            let new_position = ((position.0 as isize + diff_x) as usize, (position.1 as isize + diff_y) as usize);


            let next = State { position: new_position, 
                               direction: *dir,
                               steps_direction: if *dir == direction { steps_direction + 1 } else { 1 }, 
                               cost: cost + field[new_position.0][new_position.1] };

            let dist_key = DistKey { position: new_position, direction: *dir, steps_direction: next.steps_direction };
            // If so, add it to the frontier and continue
            if next.steps_direction <= 3 && (!dist.contains_key(&dist_key) || next.cost < dist[&dist_key]) {
                heap.push(next);
                // Relaxation, we have now found a better way
                dist.insert(dist_key, next.cost);
            }
        }
    }

    // Goal not reachable
    None
}

fn shortest_path2(field: &Vec<Vec<usize>>, start: (usize, usize), goal: (usize, usize)) -> Option<usize> {
    // dist[node] = current shortest distance from `start` to `node`
    let mut dist: HashMap<DistKey, usize> = HashMap::new();

    let mut heap = BinaryHeap::new();

    let directions = vec![Direction::Up, Direction::Down, Direction::Left, Direction::Right];

    // We're at `start`, with a zero cost
    dist.insert(DistKey { position: start, direction: Direction::Right, steps_direction: 0 }, 0);
    dist.insert(DistKey { position: start, direction: Direction::Down, steps_direction: 0 }, 0);
    heap.push(State { cost: 0, position: start, direction: Direction::Right, steps_direction: 0  });

    // Examine the frontier with lower cost nodes first (min-heap)
    while let Some(State { cost, position, direction, steps_direction }) = heap.pop() {
        // Alternatively we could have continued to find all shortest paths
        if position == goal && steps_direction >= 4 { return Some(cost); }

        // Important as we may have already found a better way
        let dist_key = DistKey { position, direction, steps_direction };
        if dist.contains_key(&dist_key) && cost > dist[&dist_key] { continue; }

        // For each node we can reach, see if we can find a way with
        // a lower cost going through this node
        for dir in directions.iter().filter(|&d| *d != direction.opposite()) {
            let (diff_x, diff_y) = dir.get_diff();
            if position.0 as isize + diff_x < 0 
                    || position.1 as isize + diff_y < 0 
                    || position.0 as isize + diff_x >= field.len() as isize 
                    || position.1 as isize + diff_y >= field[0].len() as isize {
                continue;
            }

            let new_position = ((position.0 as isize + diff_x) as usize, (position.1 as isize + diff_y) as usize);


            let next = State { position: new_position, 
                               direction: *dir,
                               steps_direction: if *dir == direction { steps_direction + 1 } else { 1 }, 
                               cost: cost + field[new_position.0][new_position.1] };

            let dist_key = DistKey { position: new_position, direction: *dir, steps_direction: next.steps_direction };
            // If so, add it to the frontier and continue
            if (direction == *dir || steps_direction >= 4) && next.steps_direction <= 10 && (!dist.contains_key(&dist_key) || next.cost < dist[&dist_key]) {
                heap.push(next);
                // Relaxation, we have now found a better way
                dist.insert(dist_key, next.cost);
            }
        }
    }

    // Goal not reachable
    None
}

fn main() {
    let input = read_to_string("input.txt").unwrap()
                    .lines()
                    .filter(|line| !line.is_empty())
                    .map(|line| line.chars().map(|c| c.to_digit(10).unwrap() as usize).collect::<Vec<_>>())
                    .collect::<Vec<_>>();

    let shortest_path = shortest_path(&input, (0, 0), (input.len() - 1, input[0].len() - 1)).unwrap();
    println!("result 1: {}", shortest_path);

    let shortest_path2 = shortest_path2(&input, (0, 0), (input.len() - 1, input[0].len() - 1)).unwrap();
    println!("result 2: {}", shortest_path2);
    
}
