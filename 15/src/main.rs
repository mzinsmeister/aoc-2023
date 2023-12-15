use std::fs::read_to_string;

fn hash(str: &str) -> u8 {
    let mut hash: u64 = 0;
    for c in str.as_bytes() {
        hash += *c as u64;
        hash = (hash * 17) % 256;
    }
    hash as u8
}

#[derive(Clone, Debug)]
struct Lens {
    label: String,
    focal_length: u32,
}

fn main() {
    let input = read_to_string("input.txt").unwrap().replace("\n", "");

    let result = input.split(",").map(|s| hash(s) as u64).sum::<u64>();

    println!("{}", result);

    let mut boxes: Vec<Vec<Lens>> = vec![vec![]; 256];

    for step in input.split(",") {
        if let Some((label, operation)) = step.split_once("=") {
            let num = operation.parse::<u32>().unwrap();
            let label_hash = hash(label);
            let cur_box = &mut boxes[label_hash as usize];
            if let Some((i, _)) = cur_box.iter().enumerate().find(|(_, lens)| lens.label == label) {
                cur_box[i].focal_length = num;
            } else {
                cur_box.push(Lens {
                    label: label.to_string(),
                    focal_length: num,
                });
            }
        } else {
            let label = step.strip_suffix("-").unwrap();
            let label_hash = hash(label);
            let cur_box = &mut boxes[label_hash as usize];
            if let Some((i, _)) = cur_box.iter().enumerate().find(|(_, lens)| lens.label == label) {
                cur_box.remove(i);
            }
        }
    }

    let result = boxes
        .iter()
        .enumerate()
        .map(|(box_no, lens_box)| {
            lens_box
                .iter()
                .enumerate()
                .map(|(lens_no, lens)| lens.focal_length * (box_no as u32 + 1) * (lens_no as u32 + 1))
                .sum::<u32>()
        })
        .sum::<u32>();

    println!("{}", result);
}
