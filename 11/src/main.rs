use std::{fs::read_to_string, cmp::{max, min}, collections::BTreeSet};

const PART_2_GALAXY_FACTOR: usize = 1_000_000;

fn main() {
    let input: Vec<Vec<char>> = read_to_string("input.txt").unwrap().lines().filter(|l| !l.is_empty()).map(|l| l.chars().collect()).collect();
    
    let mut empty_rows = BTreeSet::new();

    for (i, row) in input.iter().enumerate() {
        if row.iter().all(|&c| c == '.') {
            empty_rows.insert(i);
        }
    }

    let mut empty_cols = BTreeSet::new();
    for i in 0..input[0].len() {
        if input.iter().all(|row| row[i] == '.') {
            empty_cols.insert(i);
        }
    }

    let mut new_input = vec![];

    for (i, row) in input.iter().enumerate() {
        if empty_rows.contains(&i) {
            new_input.push(vec!['.'; row.len() + empty_cols.len()]);
        }

        let mut new_row = vec![];
        for (j, &c) in row.iter().enumerate() {
            if empty_cols.contains(&j) {
                new_row.push('.');
            }
            new_row.push(c);
        }
        new_input.push(new_row);
    }

    let mut galaxy_positions = vec![];

    for (i, row) in new_input.iter().enumerate() {
        for (j, &c) in row.iter().enumerate() {
            if c == '#' {
                galaxy_positions.push((i, j));
            }
        }
    }

    let mut galaxy_distances = 0;

    for (i, pos) in galaxy_positions.iter().enumerate() {
        for other_pos in galaxy_positions.iter().skip(i + 1) {
            let (x1, y1) = pos;
            let (x2, y2) = other_pos;
            let dist = x1.abs_diff(*x2) + y1.abs_diff(*y2);
            galaxy_distances += dist;
        }
    }

    println!("result 1: {}", galaxy_distances);

    let mut galaxy_positions = vec![];

    for (i, row) in input.iter().enumerate() {
        for (j, &c) in row.iter().enumerate() {
            if c == '#' {
                galaxy_positions.push((i, j));
            }
        }
    }

    let mut galaxy_distances = 0;

    // Not super fast but ok

    for (i, pos) in galaxy_positions.iter().enumerate() {
        for other_pos in galaxy_positions.iter().skip(i + 1) {
            let (y1, x1) = pos;
            let (y2, x2) = other_pos;
            let mut dist = 0;
            for y in min(y1+1, y2+1)..=max(*y1, *y2) {
                if empty_rows.contains(&y) {
                    dist += PART_2_GALAXY_FACTOR;
                } else {
                    dist += 1;
                }
            }
            for x in min(x1+1, x2+1)..=*max(x1, x2) {
                if empty_cols.contains(&x) {
                    dist += PART_2_GALAXY_FACTOR;
                } else {
                    dist += 1;
                }
            }
            galaxy_distances += dist;
        }
    }

    println!("result 2: {}", galaxy_distances);
}
