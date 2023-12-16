use std::{fs::read_to_string, collections::HashSet};

use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Pos {
    row: usize,
    col: usize,
}

impl Pos {
    fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }

    fn move_by(&self, direction: (isize, isize)) -> Option<Pos> {
        let new_row = self.row as isize + direction.0;
        let new_col = self.col as isize + direction.1;

        if new_row < 0 || new_col < 0 {
            return None;
        }

        Some(Self::new(new_row as usize, new_col as usize))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct WorksetEntry {
    pos: Pos,
    direction: (isize, isize),
}

impl WorksetEntry {
    fn new(pos: (usize, usize), direction: (isize, isize)) -> Self {
        Self { pos: Pos::new(pos.0, pos.1), direction }
    }
}

fn slash(direction: (isize, isize)) -> (isize, isize) {
    (-direction.1, -direction.0)
}

fn backslash(direction: (isize, isize)) -> (isize, isize) {
    (direction.1, direction.0)
}

fn simulate(input: &Vec<Vec<char>>, initial: WorksetEntry) -> usize {
    let mut work_set: HashSet<WorksetEntry> = HashSet::new();
    work_set.insert(initial);

    let mut visited: HashSet<WorksetEntry> = HashSet::new();

    while !work_set.is_empty() {
        let entry = work_set.iter().next().unwrap().clone();
        work_set.remove(&entry);
        visited.insert(entry);

        match input[entry.pos.row][entry.pos.col] {
            '.' => {
                let next_pos = entry.pos.move_by(entry.direction);
                if let Some(next_pos) = next_pos {
                    let next_entry = WorksetEntry::new((next_pos.row, next_pos.col), entry.direction);
                    if next_pos.row < input.len() && next_pos.col < input[next_pos.row].len() && !visited.contains(&next_entry) {
                        work_set.insert(next_entry);
                    }
                }
            },
            '|' => {
                if entry.direction.0 != 0 {
                    // Like '.'
                    let next_pos = entry.pos.move_by(entry.direction);
                    if let Some(next_pos) = next_pos {
                        let next_entry = WorksetEntry::new((next_pos.row, next_pos.col), entry.direction);
                        if next_pos.row < input.len() && next_pos.col < input[next_pos.row].len() && !visited.contains(&next_entry) {
                            work_set.insert(next_entry);
                        }
                    }
                } else {
                    // Split into up and down beams
                    let next_pos = entry.pos.move_by((1, 0));
                    if let Some(next_pos) = next_pos {
                        let next_entry = WorksetEntry::new((next_pos.row, next_pos.col), (1, 0));
                        if next_pos.row < input.len() && next_pos.col < input[next_pos.row].len() && !visited.contains(&next_entry) {
                            work_set.insert(next_entry);
                        }
                    }
                    let next_pos = entry.pos.move_by((-1, 0));
                    if let Some(next_pos) = next_pos {
                        let next_entry = WorksetEntry::new((next_pos.row, next_pos.col), (-1, 0));
                        if next_pos.row < input.len() && next_pos.col < input[next_pos.row].len() && !visited.contains(&next_entry) {
                            work_set.insert(next_entry);
                        }
                    }
                }
            },
            '-' => {
                if entry.direction.1 != 0 {
                    // Like '.'
                    let next_pos = entry.pos.move_by(entry.direction);
                    if let Some(next_pos) = next_pos {
                        let next_entry = WorksetEntry::new((next_pos.row, next_pos.col), entry.direction);
                        if next_pos.row < input.len() && next_pos.col < input[next_pos.row].len() && !visited.contains(&next_entry) {
                            work_set.insert(next_entry);
                        }
                    }
                } else {
                    // Split into left and right beams
                    let next_pos = entry.pos.move_by((0, 1));
                    if let Some(next_pos) = next_pos {
                        let next_entry = WorksetEntry::new((next_pos.row, next_pos.col), (0, 1));
                        if next_pos.row < input.len() && next_pos.col < input[next_pos.row].len() && !visited.contains(&next_entry) {
                            work_set.insert(next_entry);
                        }
                    }
                    let next_pos = entry.pos.move_by((0, -1));
                    if let Some(next_pos) = next_pos {
                        let next_entry = WorksetEntry::new((next_pos.row, next_pos.col), (0, -1));
                        if next_pos.row < input.len() && next_pos.col < input[next_pos.row].len() && !visited.contains(&next_entry) {
                            work_set.insert(next_entry);
                        }
                    }
                }
            },
            '/' => {
                let next_dir = slash(entry.direction);
                let next_pos = entry.pos.move_by(next_dir);
                if let Some(next_pos) = next_pos {
                    let next_entry = WorksetEntry::new((next_pos.row, next_pos.col), next_dir);
                    if next_pos.row < input.len() && next_pos.col < input[next_pos.row].len() && !visited.contains(&next_entry) {
                        work_set.insert(next_entry);
                    }
                }
            },
            '\\' => {
                let next_dir = backslash(entry.direction);
                let next_pos = entry.pos.move_by(next_dir);
                if let Some(next_pos) = next_pos {
                    let next_entry = WorksetEntry::new((next_pos.row, next_pos.col), next_dir);
                    if next_pos.row < input.len() && next_pos.col < input[next_pos.row].len() && !visited.contains(&next_entry) {
                        work_set.insert(next_entry);
                    }
                }
            },
            _ => {
                unreachable!("{}", input[entry.pos.row][entry.pos.col]);
            }
        }
    }

    let visited_positions = visited.iter().map(|e| e.pos).collect::<HashSet<_>>();
    
    // print the map with # as visited positions
    /*for (row_idx, row) in input.iter().enumerate() {
        for (col_idx, col) in row.iter().enumerate() {
            if visited_positions.contains(&Pos::new(row_idx, col_idx)) {
                print!("#");
            } else {
                print!("{}", col);
            }
        }
        println!();
    }*/
    visited_positions.len()
}

fn main() {
    let input: Vec<Vec<char>> = read_to_string("input.txt").unwrap()
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| l.chars().collect())
        .collect();

    // Part 1

    println!("result 1: {}", simulate(&input, WorksetEntry::new((0, 0), (0, 1))));

    // Part 2 (brute force, runs in roughly a second on my laptop single threaded, 200ms parallelized with)

    let mut initials = Vec::new();

    for row_idx in 0..input.len() {
        initials.push(WorksetEntry::new((row_idx, 0), (0, 1)));
        initials.push(WorksetEntry::new((row_idx, input[row_idx].len() - 1), (0, -1)));
    }

    for col_idx in 0..input[0].len() {
        initials.push(WorksetEntry::new((0, col_idx), (1, 0)));
        initials.push(WorksetEntry::new((input.len() - 1, col_idx), (-1, 0)));
    }

    let result2 = initials.par_iter().map(|initial| simulate(&input, *initial)).max();

    println!("result 2: {}", result2.unwrap());
}
