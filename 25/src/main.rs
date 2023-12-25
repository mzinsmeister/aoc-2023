use std::{fs::read_to_string, collections::BTreeMap};

fn parse_line(adj_list: &mut BTreeMap<String, Vec<String>>, line: &str) {
    let (left, right_str) = line.split_once(": ").unwrap();
    let right = right_str.split(" ").map(|s| s.to_string()).collect::<Vec<_>>();

    adj_list.entry(left.to_string()).or_insert_with(Vec::new).extend_from_slice(&right);

    for right in right {
        adj_list.entry(right).or_insert_with(Vec::new).push(left.to_string());
    }
}

fn stoer_wagner(adj_list: BTreeMap<String, Vec<String>>) -> (i32, Vec<usize>) {
    let mut mat = vec![vec![0; adj_list.len()]; adj_list.len()];
    for (i, (_, value)) in adj_list.iter().enumerate() {
        for v in value {
            let value_index = adj_list.iter().position(|(k, _)| k == v).unwrap();
            mat[i][value_index] = 1;
        }
    }

    let mut best = (i32::MAX, vec![]);
    let n = mat.len();
    let mut co = vec![vec![]; n];

    for i in 0..n {
        co[i] = vec![i];
    }

    for ph in 1..n {
        let mut w = mat[0].clone();
        let mut s = 0;
        let mut t = 0;
        for _ in 0..n - ph {
            w[t] = i32::MIN;
            s = t;
            t = w.iter().enumerate().max_by_key(|(_, v)| *v).unwrap().0;
            for i in 0..n {
                w[i] += mat[t][i];
            }
        }
        best = std::cmp::min(best, (w[t] - mat[t][t], co[t].clone()));
        let co_t = co[t].clone();
        co[s].extend_from_slice(&co_t);
        for i in 0..n {
            mat[s][i] += mat[t][i];
        }
        for i in 0..n {
            mat[i][s] = mat[s][i];
        }
        mat[0][t] = i32::MIN;
    }

    (best.0, best.1)
}



fn main() {
    let input_str = read_to_string("input.txt").unwrap();

    let mut adj_list = BTreeMap::new();
    input_str.lines().filter(|line| !line.is_empty()).for_each(|line| parse_line(&mut adj_list, line));

    let (_min_cut, min_cut_vertices) = stoer_wagner(adj_list.clone());

    assert_eq!(_min_cut, 3);

    println!("result: {}", min_cut_vertices.len() * (adj_list.len() - min_cut_vertices.len()));
}
