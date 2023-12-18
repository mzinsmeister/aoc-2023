use std::{fs::read_to_string, collections::{HashMap, BTreeMap}};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right
}

struct DigInstruction {
    direction: Direction,
    distance: u32,
}

impl DigInstruction {
    fn parse(input: &str) -> Self {
        let dir_char = input.chars().nth(0).unwrap();
        let (dist_str, rest) = input[2..].split_once(" ").unwrap();
        let color = rest[2..9].to_string();
        let direction = match dir_char {
            'U' => Direction::Up,
            'D' => Direction::Down,
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => panic!("Invalid direction")
        };
        let distance = dist_str.parse().unwrap();
        Self {
            direction,
            distance
        }
    }

    fn parse2(input: &str) -> Self {
        let (_, rest) = input[2..].split_once(" ").unwrap();
        let color = rest[2..9].to_string();
        let direction2 = match &color[5..6] {
            "0" => Direction::Right,
            "1" => Direction::Down,
            "2" => Direction::Left,
            "3" => Direction::Up,
            _ => panic!("Invalid direction {}", color[5..6].to_string())
        };
        let distance2 = u32::from_str_radix(&color[..5], 16).unwrap();
        Self {
            direction: direction2,
            distance: distance2
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct LineEntry {
    id: usize,
    column: i32,
    row: i32,
    direction: Direction,
    length: usize,
}

fn get_area(input: &Vec<DigInstruction>) -> usize {
    let mut row_map: HashMap<i32, Vec<LineEntry>> = HashMap::new();
    let mut row_map_end: HashMap<i32, Vec<usize>> = HashMap::new();

    let mut cur_coords = (0, 0);
    let mut id_counter = 0;
    let mut min_row = 0;
    let mut max_row = 0;
    let mut result = 0;
    for inst in input {
        match inst.direction {
            Direction::Up => {
                let line_entry = LineEntry {
                    id: id_counter,
                    direction: Direction::Up,
                    column: cur_coords.1,
                    row: cur_coords.0 - inst.distance as i32,
                    length: inst.distance as usize
                };
                id_counter += 1;
                result += inst.distance as usize + 1;
                //row_map_end.entry(cur_coords.0 + 1).or_insert(line_entry.id);
                row_map_end.entry(cur_coords.0 + 1).or_default().push(line_entry.id);
                cur_coords = (cur_coords.0 - inst.distance as i32, cur_coords.1);
                row_map.entry(cur_coords.0).or_default().push(line_entry);
            },
            Direction::Down => {
                let line_entry = LineEntry {
                    id: id_counter,
                    direction: Direction::Down,
                    column: cur_coords.1,
                    row: cur_coords.0,
                    length: inst.distance as usize
                };
                result += inst.distance as usize + 1;
                id_counter += 1;
                row_map.entry(cur_coords.0).or_default().push(line_entry);
                cur_coords = (cur_coords.0 + inst.distance as i32, cur_coords.1);
                row_map_end.entry(cur_coords.0 + 1).or_default().push(line_entry.id);
            },
            Direction::Left => {
                cur_coords = (cur_coords.0, cur_coords.1 - inst.distance as i32);
            },
            Direction::Right => {
                cur_coords = (cur_coords.0, cur_coords.1 + inst.distance as i32);
            }
        }
        if cur_coords.0 < min_row {
            min_row = cur_coords.0;
        }
        if cur_coords.0 > max_row {
            max_row = cur_coords.0;
        }
    }

    let mut active_lines: Vec<LineEntry> = Vec::new();
    // Looping thorugh all rows is not the most efficient way,
    // but it runs in under a second with --release
    // More effient would be just looking at the rows where the lines start/end
    // and infering the rows between that from those
    for row in min_row..=max_row {
        let mut line_result = 0;
        if let Some(entry) = row_map.get(&row) {
            active_lines.extend_from_slice(entry);
            active_lines.sort_by_key(|e| e.column);
        }
        if let Some(id) = row_map_end.get(&row) {
            active_lines.retain(|e| !id.contains(&e.id));
        }

        let entry = active_lines[0];
        let mut last_col = entry.column;
        let mut counter: i32 = 0;
        let start_end = entry.row == row || entry.row + entry.length as i32 == row;
        if entry.direction == Direction::Up {
            if start_end {
                counter += 1;
            } else {
                counter += 2;
            }
        } else {
            if start_end {
                counter -= 1;
            } else {
                counter -= 2;
            }
        }

        for entry in active_lines.iter().skip(1) {
            if counter != 0 {
                line_result += (entry.column - last_col  - 1 ).abs() as usize;
            }
            let start_end = entry.row == row || entry.row + entry.length as i32 == row;
            if entry.direction == Direction::Up {
                if start_end {
                    counter += 1;
                } else {
                    counter += 2;
                }
            } else {
                if start_end {
                    counter -= 1;
                } else {
                    counter -= 2;
                }
            }
            last_col = entry.column;
        }
        result += line_result;
    }
    result
}

fn main() {
    let input_str = read_to_string("input.txt").unwrap();

    let input = input_str.lines()   
                .filter(|l| !l.is_empty())
                .map(|line| DigInstruction::parse(line))
                .collect::<Vec<_>>();
    
    let result1 = get_area(&input);

    println!("Result 1: {}", result1);

    let input2 = input_str.lines()   
                .filter(|l| !l.is_empty())
                .map(|line| DigInstruction::parse2(line))
                .collect::<Vec<_>>();

    let result2 = get_area(&input2);

    println!("Result 2: {}", result2);
}
