use std::fs::read_to_string;

fn calc(time: usize, hold_time: usize, ) -> usize {
    let run_time = time - hold_time;
    run_time * hold_time
}

// Analytical solution exists but the numbers are small enough for brute force to work almost instantly
fn main() {
    let input = read_to_string("input.txt").unwrap();
    let (times_in, distances_in) = input.split_once("\n").unwrap();

    let times = times_in[5..times_in.len()].split_whitespace().map(|n| n.parse().unwrap()).collect::<Vec<usize>>();
    let distances = distances_in[10..distances_in.len()].split_whitespace().map(|n| n.parse().unwrap()).collect::<Vec<usize>>();

    let mut result = 1;

    for (time, dist) in times.iter().zip(distances.iter()) {
        let mut count = 0;
        for i in 1..*time {
            if calc(*time, i) > *dist {
                count += 1;
            }
        }
        result *= count;
    }

    println!("result1: {}", result);

    let time:usize = times_in[5..times_in.len()].trim().replace(" ", "").parse().unwrap();
    let distance:usize = distances_in[10..distances_in.len()].trim().replace(" ", "").parse().unwrap();

    let mut count = 0;
    for i in 1..time {
        if calc(time, i) > distance {
            count += 1;
        }
    }

    println!("result2: {}", count);
}
