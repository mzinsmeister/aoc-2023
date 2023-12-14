use std::fs::read_to_string;

struct Game {
    // Each set is RGB
    id: usize,
    rounds: Vec<[usize; 3]>
}

impl Game {
    fn check(&self, cubes: &[usize; 3]) -> bool {
        for set in &self.rounds {
            if set.iter().zip(cubes.iter()).any(|(n,max)| n > max) {
                return false
            }
        }
        true
    }

    fn fewest(&self) -> [usize; 3] {
        let mut cur = [0; 3];
        for round in &self.rounds {
            for i in 0..3 {
                if round[i] > cur[i] {
                    cur[i] = round[i];
                }
            }
        }
        return cur;
    }
}

fn parse_game(input: &str) -> Game {
    let (header, data) = input.split_once(": ").unwrap();
    let num: usize = header[5..header.len()].parse().unwrap();
    let rounds = data.split(";").map(parse_round).collect();

    Game {
        id: num,
        rounds
    }
}

fn parse_round(input: &str) -> [usize; 3] {
    let mut out = [0; 3];
    for s in input.split(",") {
        let (num, color) = s.trim().split_once(" ").unwrap();
        let index = match color {
            "red" => 0,
            "green" => 1,
            "blue" => 2,
            _ => unreachable!("unknown color {}", color)
        };
        out[index] = num.parse().unwrap();
    }
    return out;
}

fn main() {
    let input_str = read_to_string("input.txt").unwrap();

    let maximums = [12, 13, 14];

    let result: usize = input_str.split("\n")
        .filter(|l| !l.is_empty())
        .map(parse_game)
        .filter(|g| g.check(&maximums))
        .map(|g| g.id)
        .sum();

    println!("result 1: {}", result);

    let result2: usize = input_str.split("\n")
    .filter(|l| !l.is_empty())
    .map(parse_game)
    .map(|g| {
        let fewest = g.fewest();
        fewest[0] * fewest[1] * fewest[2]
    })
    .sum();

    println!("result 2: {}", result2);

}
