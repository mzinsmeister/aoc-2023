use std::{fs::read_to_string, ops::Range, cmp::min};

struct MapRange {
    range: Range<u64>,
    shift: i64
}

struct Map {
    ranges: Vec<MapRange>
}

impl Map {
    fn translate(&self, input: u64) -> u64 {
        for r in &self.ranges {
            if r.range.contains(&input) {
                return (input as i64 + r.shift) as u64;
            }
        }
        input
    }

    fn translate_range(&self, range: Range<u64>) -> Vec<Range<u64>> {
        let start = range.start;
        let mut result = vec![];
        let mut cur = start;
        for r in self.ranges.iter().skip_while(|r| r.range.end < start) {
            if r.range.start > cur {
                result.push(cur..min(range.end, r.range.start));
                cur = r.range.start;
            }
            if cur >= range.end {
                break;
            }
            result.push((cur as i64 + r.shift) as u64..(min(range.end, r.range.end) as i64 + r.shift) as u64);
            cur = r.range.end;
            if cur >= range.end {
                break;
            }
        }
        if cur < range.end {
            result.push(cur..range.end);
        }
        result
    }
}

fn parse_map(input: &str) -> Map {
    let (_, input) = input.split_once("\n").unwrap();
    let mut ranges: Vec<MapRange> = input.split("\n").filter(|l| !l.is_empty()).map(|l| {
        let (first_start, rest) = l.split_once(" ").unwrap();
        let (second_start, len) = rest.split_once(" ").unwrap();
        let first_start: u64 = first_start.parse().unwrap();
        let second_start: u64 = second_start.parse().unwrap();
        let len: u64 = len.parse().unwrap();
        MapRange {
            range: second_start..second_start+len,
            shift: first_start as i64 - second_start as i64
        }
    }).collect();
    ranges.sort_by(|a,b| a.range.start.cmp(&b.range.start));
    Map { ranges }
}

fn main() {
    let input_str = read_to_string("input.txt").unwrap();

    let (init_str, maps_str) = input_str.split_once("\n\n").unwrap();

    let init: Vec<u64> = init_str[7..init_str.len()].split(" ").map(|n| n.parse().unwrap()).collect();

    let maps = maps_str
        .split("\n\n")
        .filter(|l| !l.is_empty())
        .map(parse_map)
        .collect::<Vec<Map>>();

    let mut min = u64::MAX;

    for i in &init {
        let mut cur = *i;
        for m in &maps {
            cur = m.translate(cur);
        }
        if cur < min {
            min = cur;
        }
    }

    println!("result 1: {}", min);

    let mut min = u64::MAX;

    for i in (0..init.len()).step_by(2) {
        let mut cur = vec![init[i]..init[i]+init[i+1]];
        for m in &maps {
            let mut new = vec![];
            for r in cur {
                let mut result = m.translate_range(r);
                new.append(&mut result);
            }
            cur = new;
        }
        for r in cur {
            if r.start < min {
                min = r.start;
            }
        }
    }

    println!("result 2: {}", min);
}
