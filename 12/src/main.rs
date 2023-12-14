use std::{fs::read_to_string, collections::BTreeMap};

use itertools::Itertools;

struct Line {
    positions: Vec<char>,
    numbers: Vec<usize>,
}

impl Line {
    fn check(&self, line: &Vec<char>) -> bool {
        let mut line_iter = line.iter();
        for i in self.numbers.iter() {
            while line_iter.next() == Some(&'.') {}
            let mut len = 1;
            while line_iter.next() == Some(&'#') {
                len += 1;
            }
            if len != *i {
                return false;
            }
        }
        true
    }

    fn count_possibilities(&self) -> usize {
        let mut count = 0;
        let base = self.positions.clone();
        let num_hash = self.positions.iter().filter(|x| **x == '#').count();
        if num_hash > self.numbers.iter().sum::<usize>() {
            return 0;
        }
        let sum_set = self.numbers.iter().sum::<usize>() - num_hash;
        let quest_positions: Vec<usize> = self.positions.iter().enumerate().filter(|(_, x)| **x == '?').map(|(i, _)| i).collect();
        // we need to set sum_set '?' positions to #, the others to . 
        for i in (0..quest_positions.len()).combinations(sum_set) {
            let mut new_line = base.clone();
            for j in &i {
                new_line[quest_positions[*j]] = '#';
            }
            new_line.iter_mut().filter(|x| **x == '?').for_each(|x| *x = '.');
            if self.check(&new_line) {
                count += 1;
            }
        }
        count
    }

    fn count_possibilities2(&self) -> usize {
        // It's DP. It's not beautiful, it could probably be faster, but it works.
        let mut dp_table = BTreeMap::new();
        dp_table.insert((self.positions.len(), self.numbers.len()), 1);
        dp_table.insert((self.positions.len()+1, self.numbers.len()), 1);
        for i in (0..self.positions.len()).rev() {
            if self.positions[i] != '#' && dp_table.contains_key(&(i+1, self.numbers.len())) {
                dp_table.insert((i, self.numbers.len()), 1);
            }
            'inner:
            for j in (0..self.numbers.len()).rev() {
                if i > 0 && self.positions[i-1] == '#' {
                    if dp_table.contains_key(&(i+1, j)) {
                        dp_table.insert((i, j), *dp_table.get(&(i+1, j)).unwrap());
                    }
                    continue;
                }
                let sub_positions = &self.positions[i..];
                let number = &self.numbers[j];
                let mut new_line = sub_positions.to_vec();
                let mut new_line_i = 0;
                if self.positions[i] == '.' && dp_table.contains_key(&(i+1, j)) {
                    dp_table.insert((i, j), *dp_table.get(&(i+1, j)).unwrap());
                    continue;
                }
                let mut len = 0;
                while new_line_i + len < new_line.len() && new_line[new_line_i + len] != '.'  {
                    len += 1;
                }
                if len >= *number {
                    new_line[new_line_i..new_line_i + *number].iter_mut().for_each(|x| *x = '#');
                    new_line_i += *number;
                    if dp_table.contains_key(&(i + new_line_i + 1, j + 1)) && self.positions.get(i + new_line_i).unwrap_or(&'?') != &'#' {
                        let new_n = if self.positions[i] == '?' {
                            dp_table.get(&(i + new_line_i + 1, j + 1)).unwrap() + dp_table.get(&(i + 1, j)).unwrap_or(&0)
                        } else {
                            *dp_table.get(&(i + new_line_i + 1, j + 1)).unwrap()
                        };
                        dp_table.insert((i, j), new_n);
                    } else if self.positions[i] == '?' && dp_table.contains_key(&(i+1, j))  {
                        dp_table.insert((i, j), *dp_table.get(&(i+1, j)).unwrap());
                    } else {
                        continue 'inner;
                    }
                } else if self.positions[i] == '?' && dp_table.contains_key(&(i+1, j)) {
                    dp_table.insert((i, j), *dp_table.get(&(i+1, j)).unwrap());
                }
            }
        }
        *dp_table.get(&(0, 0)).unwrap_or(&0)
    }

    fn unfold(&self) -> Line {
        let mut new_positions = Vec::new();
        let mut new_numbers = Vec::new();
        for i in 0..5 {
            new_positions.extend(self.positions.clone());
            if i < 4 {
                new_positions.push('?');
            }
            new_numbers.extend(self.numbers.clone());
        }
        Line { positions: new_positions, numbers: new_numbers }
    }
}

fn main() {
    let input_str = read_to_string("input.txt").unwrap();
    
    let input: Vec<Line> = input_str
        .split("\n")
        .filter(|x| !x.is_empty())
        .map(|x| {
            let (positions_str, numbers_str) = x.split_once(" ").unwrap();
            let positions = positions_str.chars().collect();
            let numbers = numbers_str.split(",").map(|x| x.parse::<usize>().unwrap()).collect();
            Line { positions, numbers }
        })
        .collect();

    let result = input.iter().map(|x| x.count_possibilities()).sum::<usize>();

    println!("result 1: {}", result);

    let input2 = input.iter().map(|x| x.unfold()).collect::<Vec<Line>>();

    let result2 = input2.iter().map(|x| x.count_possibilities2()).sum::<usize>();

    println!("result 2: {}", result2);
}
