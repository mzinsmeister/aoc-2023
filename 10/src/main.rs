use std::fs::read_to_string;

const NW: char = 'J';
const NE: char = 'L';
const SW: char = '7';
const SE: char = 'F';

fn find_start(input: &Vec<Vec<char>>) -> (usize, usize) {
    for (i, row) in input.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            if *c == 'S' {
                return (i, j);
            }
        }
    }
    panic!("No start found");
}

fn find_next(input: &Vec<Vec<char>>, path: &Vec<(usize, usize)>) -> (usize, usize) {
    let (y, x) = path[path.len() - 1];
    let c = input[y][x];
    match c {
        '|' => if path[path.len() - 2].0 < y { (y + 1, x) } else { (y - 1, x) },
        '-' => if path[path.len() - 2].1 < x { (y, x + 1) } else { (y, x - 1) },
        NW => if path[path.len() - 2].0 == y { (y - 1, x) } else { (y, x - 1) },
        NE => if path[path.len() - 2].0 == y { (y - 1, x) } else { (y, x + 1) },
        SW => if path[path.len() - 2].0 == y { (y + 1, x) } else { (y, x - 1) },
        SE => if path[path.len() - 2].0 == y { (y + 1, x) } else { (y, x + 1) },
        _ => panic!("Invalid character"),
    }
}

fn main() {
    let input_str = read_to_string("input.txt").unwrap();
    
    let input:  Vec<Vec<char>> = input_str.split("\n").filter(|l| !l.is_empty()).map(|x| x.chars().collect()).collect();

    let start = find_start(&input);

    let mut path: Vec<(usize, usize)> = Vec::new();

    path.push(start);

    if start.0 < input.len() && (
            input[start.0+1][start.1] == '|' 
            || input[start.0+1][start.1] == NW
            || input[start.0+1][start.1] == NE) {
        path.push((start.0 + 1, start.1));
    } else if start.0 > 0 && (
            input[start.0-1][start.1] == '|' 
            || input[start.0-1][start.1] == SW
            || input[start.0-1][start.1] == SE) {
        path.push((start.0 - 1, start.1));
    } else if start.1 < input[0].len() && (
            input[start.0][start.1+1] == '-' 
            || input[start.0][start.1+1] == NW
            || input[start.0][start.1+1] == SW) {
        path.push((start.0, start.1 + 1));
    } else if start.1 > 0 && (
            input[start.0][start.1-1] == '-' 
            || input[start.0][start.1-1] == NE
            || input[start.0][start.1-1] == SE) {
        path.push((start.0, start.1 - 1));
    } else {
        panic!("Invalid start");
    }

    loop {
        let next = find_next(&input, &path);
        if input[next.0][next.1] == 'S' {
            break;
        }
        path.push(next);
    }

    println!("result 1: {}", path.len() / 2);

    let mut loop_positions: Vec<Vec<char>> = Vec::new();

    for _ in 0..input.len() {
        let mut row: Vec<char> = Vec::new();
        for _ in 0..input[0].len() {
            row.push('.');
        }
        loop_positions.push(row);
    }

    for p in path.iter() {
        loop_positions[p.0][p.1] = input[p.0][p.1];
    }

    // replace S by appropriate part
    if loop_positions[start.0 - 1][start.1] != '.' && loop_positions[start.0 + 1][start.1] != '.' {
        loop_positions[start.0][start.1] = '|';
    } else if loop_positions[start.0][start.1 - 1] != '.' && loop_positions[start.0][start.1 + 1] != '.' {
        loop_positions[start.0][start.1] = '-';
    } else if loop_positions[start.0 - 1][start.1] != '.' && loop_positions[start.0][start.1 - 1] != '.' {
        loop_positions[start.0][start.1] = NW;
    } else if loop_positions[start.0 - 1][start.1] != '.' && loop_positions[start.0][start.1 + 1] != '.' {
        loop_positions[start.0][start.1] = NE;
    } else if loop_positions[start.0 + 1][start.1] != '.' && loop_positions[start.0][start.1 - 1] != '.' {
        loop_positions[start.0][start.1] = SW;
    } else if loop_positions[start.0 + 1][start.1] != '.' && loop_positions[start.0][start.1 + 1] != '.' {
        loop_positions[start.0][start.1] = SE;
    } else {
        panic!("Invalid start");
    }

    let mut inside_count = 0;

    for (y, row) in input.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            if loop_positions[y][x] == '.' {
                let mut left_count = 0;
                for left in 0..x {
                    if loop_positions[y][left] == '|' || loop_positions[y][left] == NE || loop_positions[y][left] == NW {
                        left_count += 1;
                    }
                }
                let mut right_count = 0;
                for right in x+1..input[0].len() {
                    if loop_positions[y][right] == '|' || loop_positions[y][right] == NE || loop_positions[y][right] == NW {
                        right_count += 1;
                    }                
                }
                if left_count % 2 == 1 && right_count % 2 == 1 {
                    inside_count += 1;
                }
            }
        }
    }
   

    println!("result 2: {}", inside_count);

    let mut test: Vec<Vec<Option<i32>>> = Vec::new();

    for _ in 0..input.len() {
        let mut row: Vec<Option<i32>> = Vec::new();
        for _ in 0..input[0].len() {
            row.push(Some(0));
        }
        test.push(row);
    }

}