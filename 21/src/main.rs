use std::{fs::read_to_string, collections::{HashSet, HashMap, VecDeque}};

fn dfs_rec(memo_table: &mut HashSet<((isize, isize), usize)>, grid: &Vec<Vec<char>>, pos: (isize, isize), steps_left: usize, end_positions: &mut HashSet<(isize, isize)>) {
    if memo_table.contains(&(pos, steps_left)) {
        return;
    }
    let (row, col) = pos;
    if grid[row.rem_euclid(grid.len() as isize) as usize][col.rem_euclid(grid.len() as isize) as usize] == '#' {
        return;
    }
    if steps_left == 0 {
        end_positions.insert(pos);
        return;
    }
        dfs_rec(memo_table, grid, (row-1, col), steps_left - 1, end_positions);
    dfs_rec(memo_table, grid, (row+1, col), steps_left - 1, end_positions);
    dfs_rec(memo_table, grid, (row, col-1), steps_left - 1, end_positions);
    dfs_rec(memo_table, grid, (row, col+1), steps_left - 1, end_positions);
    memo_table.insert((pos, steps_left));
}

fn bfs2(grid: &Vec<Vec<char>>, start_position: (usize, usize)) -> HashMap<(usize, usize), usize> {
    // Number of grid elements not '#'
    let mut visited_table: HashMap<(usize, usize), usize> = HashMap::new();

    let mut queue: VecDeque<((usize, usize), usize)> = VecDeque::new();
    queue.push_back((start_position, 0));
    while !queue.is_empty() {
        let ((row, col), steps_taken) = queue.pop_front().unwrap();
        if grid[row][col] == '#' {
            continue;
        } 
        if visited_table.contains_key(&(row, col)) {
            continue;
        } else {
            visited_table.insert((row, col), steps_taken);
        }
        if row > 0 {
            queue.push_back(((row - 1, col), steps_taken + 1));
        }
        if row < grid.len() - 1 {
            queue.push_back(((row + 1, col), steps_taken + 1));
        }
        if col > 0 {
            queue.push_back(((row, col - 1), steps_taken + 1));
        }
        if col < grid[0].len() - 1 {
            queue.push_back(((row, col + 1), steps_taken + 1));
        }
    }
    visited_table
}

fn main() {
    let input: Vec<Vec<char>> = read_to_string("input.txt").unwrap()
                .lines()
                .filter(|l| !l.is_empty())
                .map(|l| l.chars().collect())
                .collect();
    
    // find the position of the S in the input
    let start_position = input.iter()
            .enumerate()
            .find_map(|(row, line)| line.iter().position(|&c| c == 'S').map(|col| (row, col)))
            .unwrap();

    //let start_position = (0, );

    let mut end_positions: HashSet<(isize, isize)> = HashSet::new();
    let mut memo_table: HashSet<((isize, isize), usize)> = HashSet::new();

    dfs_rec(&mut memo_table, &input, (start_position.0 as isize, start_position.1 as isize), 64, &mut end_positions);

    println!("result 1: {}", end_positions.len());

    // write part 1 result to a file with the map with reachable positions marked with x
    let mut output = String::new();
    for (row, line) in input.iter().enumerate() {
        for (col, c) in line.iter().enumerate() {
            if end_positions.contains(&(row as isize , col as isize)) {
                output.push('x');
            } else {
                output.push(*c);
            }
        }
        output.push('\n');
    }
    std::fs::write("output.txt", output).unwrap();

    let num_steps_2 = 26501365;

    // Approach: Per start position on the outer border of the grid, we calculate the minimum number of steps with which we can reach
    // all the positions inside the grid.

    let min_distances_start = bfs2(&input, start_position);

    // Check whether all of them are equal to the manhattan distance
    for (pos, distance) in min_distances_start {
        let (row, col) = pos;
        let manhattan_distance = (start_position.0 as isize - row as isize).abs() + (start_position.1 as isize - col as isize).abs();
        if manhattan_distance != distance as isize {
            //println!("({}, {}) {} - {}", row, col, distance, manhattan_distance)
        }
    }

    // All positions are reachable within manhattan distance from the start and 
    // all positions on the outer border can reach all positions within manhattan distance too
    // Therefore the furthest fields we can reach are a huge diamond shape with the start position in the middle
    // and then we need to determine the 

    let mut pos_even = 0usize;
    let mut pos_odd = 0usize;
    let mut pos_without_diamond_even = 0usize;
    let mut pos_diamond_odd =  memo_table.iter().filter(|(_, s)| *s == 0).count();

    for (row, line) in input.iter().enumerate() {
        for (col, c) in line.iter().enumerate() {
            if *c != '#' {
                if (row + col) % 2 == 0 {
                    pos_even += 1;
                } else {
                    pos_odd += 1;
                }
                let manhattan_distance = (start_position.0 as isize - row as isize).abs() + (start_position.1 as isize - col as isize).abs();
                if manhattan_distance > 65 {
                    if (row + col) % 2 == 0 {
                        pos_without_diamond_even += 1;
                    }
                } else {
                    if (row + col) % 2 == 1 {
                        pos_diamond_odd += 1;
                    }
                }
            }
        }
    }

    /* 
      for rows smaller than middle row add (num = number of even patches of middle row minus 1):
      even: 2*(num-1)
      odd: 2*(num-1)+1
      even without diamond: 1
      odd diamond: 1
    */

    let mut result = 0usize;

    let num = (num_steps_2 / input[0].len());

    // add middle row
    result += num * (pos_even + pos_odd) + pos_diamond_odd;

    // add rows above middle row
    for row in 1..=num {
        result += 2 * (num - row) * (pos_even + pos_odd) + pos_odd + pos_without_diamond_even + pos_diamond_odd;
    }

    // I have absolutely no idea where this correction formula comes from. 
    // I just figured it out experimentally by comparing against the result of the brute force (first part) solution
    result -= 3 * num*num + 2 * num;

    println!("result 2: {}", result);
}
