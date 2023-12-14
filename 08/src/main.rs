use std::{fs::read_to_string, collections::BTreeMap};

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a%b)
    }
}

fn main() {
    let input_str = read_to_string("input.txt").unwrap();
    
    let (instructions_str, network) = input_str.split_once("\n\n").unwrap();

    let instructions = instructions_str.as_bytes();

    let mapping: BTreeMap<String, (String, String)> = network
        .lines()
        .filter(|l| !l.is_empty())
        .map(|line| {
            let (key, value) = line.split_once(" = ").unwrap();
            let values = value[1..value.len()-1].split_once(", ").unwrap();
            (key.to_string(), (values.0.to_owned(), values.1.to_owned()))
        })
        .collect();

    let mut cur = "AAA".to_string();

    let mut i = 0;

    while cur != "ZZZ" {
        let (left, right) = mapping.get(&cur).unwrap();
        cur = if instructions[i% instructions.len()] == b'R' {
            right.to_owned()
        } else {
            left.to_owned()
        };
        i += 1;
    }

    println!("result1: {}", i);

    let init: Vec<String> = mapping.keys().filter(|k| k.ends_with("A")).cloned().collect();

    let mut initials = Vec::<usize>::new();
    let mut cycles = Vec::<usize>::new();

    for c in init {
        let mut cur = c;
        let mut i = 0;
        while !cur.ends_with("Z") {
            let (left, right) = mapping.get(&cur).unwrap();
            cur = if instructions[i% instructions.len()] == b'R' {
                right.to_owned()
            } else {
                left.to_owned()
            };
            i += 1;
        }
        initials.push(i);
        let (left, right) = mapping.get(&cur).unwrap();
        cur = if instructions[i% instructions.len()] == b'R' {
            right.to_owned()
        } else {
            left.to_owned()
        };
        i += 1;
        while !cur.ends_with("Z") {
            let (left, right) = mapping.get(&cur).unwrap();
            cur = if instructions[i% instructions.len()] == b'R' {
                right.to_owned()
            } else {
                left.to_owned()
            };
            i += 1;
        }
        cycles.push(i - initials.last().unwrap());
    }

    let mut master_init = initials[0];
    let mut master_cycle = cycles[0];

    for i in 1..initials.len() {
        let cur_cycle = cycles[i];
        let mut master_i = master_init;
        let mut cur_i = initials[i];
        while master_i != cur_i {
            if master_i < cur_i {
                master_i += ((cur_i - master_i + (master_cycle -1)) / master_cycle) * master_cycle;
            } else {
                cur_i += ((master_i - cur_i + (cur_cycle -1)) / cur_cycle) * cur_cycle;
            }
        }
        master_cycle = (master_cycle * cur_cycle) / gcd(master_cycle, cur_cycle);
        master_init = master_i;
    }

    println!("result2: {}", master_init);
}
