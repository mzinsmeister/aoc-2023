use std::fs::read_to_string;

// I know it's not very readable but i wanted to practice my iterators

fn diff_rows(field: &Vec<Vec<char>>, line: usize) -> usize {
    field.iter()
        .skip(line)
        .zip(field.iter().rev().skip(field.len() - line))
        .map(|(a, b)| a.iter().zip(b.iter()).filter(|(a, b)| a != b).count())
        .sum()
}

fn diff_cols(field: &Vec<Vec<char>>, line: usize) -> usize {
    field.iter()
        .map(|row| row.iter()
                                .skip(line)
                                .zip(row.iter().rev().skip(field[0].len() - line))
                                .filter(|(a, b)| a != b)
                                .count())
        .sum()
}

fn main() {

    let input: Vec<Vec<Vec<char>>> = read_to_string("input.txt").unwrap()
        .split("\n\n")
        .filter(|g| !g.is_empty())
        .map(|g| g.lines().map(|l|l.chars().collect()).collect())
        .collect();

    let mut result = 0;

    for field in &input {
        for i in 1..field.len() {
            if diff_rows(field, i) == 0 {
                result += 100 * i;
                break;
            }
        }
    
        for i in 1..field[0].len() {
            if diff_cols(field, i) == 0 {
                result += i;
                break;
            }
        }
    }

    println!("result 1: {}", result);

    let mut result = 0;

    for field in &input {
        for i in 1..field.len() {
            if diff_rows(field, i) == 1 {
                result += 100 * i;
                break;
            }
        }
    
        for i in 1..field[0].len() {
            if diff_cols(field, i) == 1 {
                result += i;
                break;
            }
        }
    }

    println!("result 2: {}", result);

}
