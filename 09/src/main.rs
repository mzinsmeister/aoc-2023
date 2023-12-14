use std::fs::read_to_string;


fn calc_diffs(values: &Vec<i64>) -> Vec<i64> {
    let mut diffs: Vec<i64> = Vec::new();
    for i in 0..values.len() - 1 {
        diffs.push(values[i + 1] - values[i]);
    }
    diffs
}

fn calc_all_diffs(values: &Vec<i64>) -> Vec<Vec<i64>> {
    let mut all_diffs: Vec<Vec<i64>> = Vec::new();
    let mut diffs = calc_diffs(values);
    while diffs.iter().any(|&x| x != 0) {
        let new_diffs = calc_diffs(&diffs);
        all_diffs.push(diffs);
        diffs = new_diffs;
    }
    all_diffs.push(diffs);
    all_diffs
}

fn predict_next(values: &Vec<i64>) -> i64 {
    let diffs = calc_all_diffs(values);
    let mut next_cur_diff = 0;
    for d in diffs.iter().rev() {
        next_cur_diff = next_cur_diff + d.last().unwrap();
    }
    next_cur_diff + values.last().unwrap()
}

fn predict_prev(values: &Vec<i64>) -> i64 {
    let diffs = calc_all_diffs(values);
    let mut next_cur_diff = 0;
    for d in diffs.iter().rev() {
        next_cur_diff = d.first().unwrap() - next_cur_diff;
    }
    values.first().unwrap() - next_cur_diff
}

fn main() {
    let input_str = read_to_string("input.txt").unwrap();

    let values: Vec<Vec<i64>> = input_str
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| l.split_whitespace().map(|n| n.parse::<i64>().unwrap()).collect())
        .collect();

    let result = values.iter().map(|v| predict_next(v)).sum::<i64>();

    println!("Result: {}", result);

    let result2 = values.iter().map(|v| predict_prev(v)).sum::<i64>();

    println!("Result2: {}", result2);
}
