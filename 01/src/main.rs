use std::fs::read_to_string;

use regex::{Regex, RegexSet};

fn main() {
    let input = read_to_string("input.txt").unwrap();

    let mut sum = 0;
    for line in input.split("\n").filter(|l| !l.is_empty()) {
        let mut first_num = -1;
        let mut last_num = -1;
        for c in line.chars() {
            if let Some(num) = c.to_digit(10) {
                last_num = num as i32;
                if first_num < 0 {
                    first_num = num as i32;
                }
            }
        }
        sum += first_num * 10 + last_num;
    }
    println!("result1: {}", sum);

    let mut sum = 0;
    let regex = Regex::new(r"(one|two|three|four|five|six|seven|eight|nine|1|2|3|4|5|6|7|8|9)").unwrap();
    for line in input.split("\n").filter(|l| !l.is_empty()) {
        let items_iter = regex.find_iter(line);
        let mut first_num = -1;
        let mut last_num = -1;
        let mut last_match = -1;
        for m in items_iter {
            let num = match m.as_str() {
                "one" => 1,
                "two" => 2,
                "three" => 3,
                "four" => 4,
                "five" => 5,
                "six" => 6,
                "seven" => 7,
                "eight" => 8,
                "nine" => 9,
                _ => m.as_str().parse().unwrap()
            };
            last_num = num;
            last_match = m.start() as i32;
            if first_num < 0 {
                first_num = num;
            }
        }
        while let Some(m) = regex.find_at(line, last_match as usize + 1) {
            let num = match m.as_str() {
                "one" => 1,
                "two" => 2,
                "three" => 3,
                "four" => 4,
                "five" => 5,
                "six" => 6,
                "seven" => 7,
                "eight" => 8,
                "nine" => 9,
                _ => m.as_str().parse().unwrap()
            };
            last_num = num;
            last_match = m.start() as i32;
        }
        sum += first_num * 10 + last_num;
    }
    println!("result2: {}", sum);
}