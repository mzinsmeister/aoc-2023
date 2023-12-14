use std::{fs::read_to_string, collections::BTreeSet};

struct Card {
    winning: BTreeSet<u8>,
    have: BTreeSet<u8>
}

impl Card {
    fn count_winning(&self) -> usize {
        self.have.iter().filter(|n| self.winning.contains(n)).count()
    }

    fn score(&self) -> usize {
        let winning_count = self.count_winning();
        if winning_count > 0 {
            1 << (winning_count - 1)
        } else {
            0
        }
    }
}

fn parse_line(input: &str) -> Card {
    let (_, input) = input.split_once(": ").unwrap();
    let (winning_input, have_input) = input.split_once(" | ").unwrap();
    let winning = winning_input.split(" ").filter(|n| !n.is_empty()).map(|n| n.parse().unwrap()).collect();
    let have = have_input.split(" ").filter(|n| !n.is_empty()).map(|n| n.parse().unwrap()).collect();
    Card {
        winning,
        have
    }
}

fn main() {
    let input_str = read_to_string("input.txt").unwrap();
    
    let input = input_str.split("\n").filter(|l| !l.is_empty()).map(parse_line).collect::<Vec<Card>>();

    let result: usize = input.iter().map(Card::score).sum();

    println!("result1: {}", result);


    let mut nums: Vec<usize> = vec![1; input.len()];

    for (i, c) in input.iter().enumerate() {
        let winning = c.count_winning();

        for j in i+1..i+1+winning {
            nums[j] += nums[i];
        }
    }

    let result2: usize = nums.iter().sum();

    println!("result 2: {}", result2);

}
