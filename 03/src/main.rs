use std::cmp::{min, max};
use std::fs::read_to_string;
use std::str;

fn part_1() {
    let input_str = read_to_string("input.txt").unwrap();

    let lines = input_str.split("\n").filter(|l| !l.is_empty()).map(|l| l.as_bytes()).collect::<Vec<&[u8]>>();
    let line_len = lines[0].len();

    let has_adjacent_symbol = |line: usize, start: usize, len: usize| -> bool {
        if start > 0 && lines[line][start-1] != b'.' || start + len < line_len && lines[line][start+len] != b'.' {
            return true;
        }
        let search_start = if start > 0 { start - 1 } else { start };
        if line > 0 {
            for i in max(0, search_start)..min(start+len+1, line_len) {
                if lines[line-1][i] != b'.' && !lines[line-1][i].is_ascii_digit() {
                    return true;
                }
            }
        }
        if line < lines.len() - 1 {
            for i in max(0,search_start)..min(start+len+1, line_len) {
                if lines[line+1][i] != b'.' && !lines[line+1][i].is_ascii_digit() {
                    return true;
                }
            }
        }
        false
    };

    let mut part_nums = vec![];

    for (i, l) in lines.iter().enumerate() {
        let mut cur_num_start = Option::None;
        for (j, c) in l.iter().enumerate() {
            if c.is_ascii_digit() {
                if cur_num_start.is_none() {
                    cur_num_start = Some(j);
                }
            } else if let Some(cur_num_start_val) = cur_num_start {
                let num: usize = str::from_utf8(&l[cur_num_start_val..j]).unwrap().parse().unwrap();
                if has_adjacent_symbol(i, cur_num_start_val, j - cur_num_start_val) {
                    part_nums.push(num);
                }
                cur_num_start = None;
            }
        }
        if let Some(cur_num_start) = cur_num_start {
            let num: usize = str::from_utf8(&l[cur_num_start..line_len]).unwrap().parse().unwrap();
            if has_adjacent_symbol(i, cur_num_start, line_len - cur_num_start) {
                part_nums.push(num);
            }
        }
    }

    println!("result 1: {}", part_nums.iter().sum::<usize>());
}

fn part_2() {
    let input_str = read_to_string("input.txt").unwrap();

    let lines = input_str.split("\n").filter(|l| !l.is_empty()).map(|l| l.as_bytes()).collect::<Vec<&[u8]>>();
    let line_len = lines[0].len();

    let mut stars = vec![];

    for line in &lines {
        let mut stars_line = vec![];
        for _ in line.iter() {
            stars_line.push(Vec::<usize>::new());
        }
        stars.push(stars_line);
    }

    let mut check_stars = |line: usize, start: usize, len: usize, num: usize| {
        if start > 0 && lines[line][start-1] == b'*' {
            stars[line][start-1].push(num);
        }
        if start + len < line_len && lines[line][start+len] == b'*' {
            stars[line][start+len].push(num);
        }
        let search_start = if start > 0 { start - 1 } else { start };
        if line > 0 {
            for i in max(0, search_start)..min(start+len+1, line_len) {
                if lines[line-1][i] == b'*' {
                    stars[line-1][i].push(num);
                }
            }
        }
        if line < lines.len() - 1 {
            for i in max(0,search_start)..min(start+len+1, line_len) {
                if lines[line+1][i] == b'*' {
                    stars[line+1][i].push(num);
                }
            }
        }
    };

    for (i, l) in lines.iter().enumerate() {
        let mut cur_num_start = Option::None;
        for (j, c) in l.iter().enumerate() {
            if c.is_ascii_digit() {
                if cur_num_start.is_none() {
                    cur_num_start = Some(j);
                }
            } else if let Some(cur_num_start_val) = cur_num_start {
                let num: usize = str::from_utf8(&l[cur_num_start_val..j]).unwrap().parse().unwrap();
                check_stars(i, cur_num_start_val, j - cur_num_start_val, num);
                cur_num_start = None;
            }
        }
        if let Some(cur_num_start) = cur_num_start {
            let num: usize = str::from_utf8(&l[cur_num_start..line_len]).unwrap().parse().unwrap();
            check_stars(i, cur_num_start, line_len - cur_num_start, num);
        }
    }

    let result: usize = stars.iter()
        .map(|l| l.iter())
        .flatten()
        .filter(|s| s.len() == 2)
        .map(|s| s[0] * s[1])
        .sum();

    println!("result 2: {}", result);
}

fn main() {
    part_1();
    part_2();   
}
