use std::{fs::read_to_string, collections::HashMap};

fn rotate_north(field: &mut Vec<Vec<char>>) {
    for row in 1..field.len() {
        'colloop:
        for col in 0..field[row].len() {
            if field[row][col] == 'O' && field[row - 1][col] == '.' {
                for i in (0..row).rev() {
                    if field[i][col] != '.' {
                        field[i + 1][col] = 'O';
                        field[row][col] = '.';
                        continue 'colloop;
                    }
                }
                field[0][col] = 'O';
                field[row][col] = '.';
            }
        }
    }
}

fn rotate_south(field: &mut Vec<Vec<char>>) {
    let field_len = field.len();
    for row in (0..field.len() - 1).rev() {
        'colloop:
        for col in 0..field[row].len() {
            if field[row][col] == 'O' && field[row + 1][col] == '.' {
                for i in row + 1..field.len() {
                    if field[i][col] != '.' {
                        field[i - 1][col] = 'O';
                        field[row][col] = '.';
                        continue 'colloop;
                    }
                }
                field[field_len - 1][col] = 'O';
                field[row][col] = '.';
            }
        }
    }
}

fn rotate_east(field: &mut Vec<Vec<char>>) {
    let row_len = field[0].len();
    for col in (0..field[0].len() - 1).rev() {
        'rowloop:
        for row in 0..field.len() {
            if field[row][col] == 'O' && field[row][col + 1] == '.' {
                for i in col + 1..field[row].len() {
                    if field[row][i] != '.' {
                        field[row][i - 1] = 'O';
                        field[row][col] = '.';
                        continue 'rowloop;
                    }
                }
                field[row][row_len - 1] = 'O';
                field[row][col] = '.';
            }
        }
    }
}

fn rotate_west(field: &mut Vec<Vec<char>>) {
    for col in 1..field[0].len() {
        'rowloop:
        for row in 0..field.len() {
            if field[row][col] == 'O' && field[row][col - 1] == '.' {
                for i in (0..col).rev() {
                    if field[row][i] != '.' {
                        field[row][i + 1] = 'O';
                        field[row][col] = '.';
                        continue 'rowloop;
                    }
                }
                field[row][0] = 'O';
                field[row][col] = '.';
            }
        }
    }
}

fn main() {
    let input_str = read_to_string("input.txt").unwrap();

    let input: Vec<Vec<char>> = input_str
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.chars().collect())
        .collect();

    let mut field = input.clone();

    rotate_north(&mut field);

    let result: usize = field
        .iter()
        .enumerate()
        .map(|(i, row)| row.iter().filter(|&&c| c == 'O').count() * (field.len() - i))
        .sum();

    println!("result1: {}", result);

    let mut field = input.clone();

    let mut cache: HashMap<Vec<Vec<char>>, usize> = HashMap::new();
    cache.insert(field.clone(), 0);

    for i in 0..1_000_000_000 {
        rotate_north(&mut field);
        rotate_west(&mut field);
        rotate_south(&mut field);
        rotate_east(&mut field);
        if let Some(cache_i) = cache.get(&field) {
            let cycle = i + 1 - cache_i;
            let remaining = (1_000_000_000 - i - 1) % cycle;
            for _ in 0..remaining {
                rotate_north(&mut field);
                rotate_west(&mut field);
                rotate_south(&mut field);
                rotate_east(&mut field);
            }
            break;
        }
        cache.insert(field.clone(), i + 1);
    }

    let result: usize = field
        .iter()
        .enumerate()
        .map(|(i, row)| row.iter().filter(|&&c| c == 'O').count() * (field.len() - i))
        .sum();

    println!("result2: {}", result);

}
