use std::{collections::BTreeMap, ops::Add, fs::read_to_string, cmp::Ordering};



#[derive(PartialEq, PartialOrd, Eq, Debug, Clone, Copy, Ord)]
enum Card {
    A, K, Q, J, T, N9, N8, N7, N6, N5, N4, N3, N2
}

fn parse_hand(s: &str) -> Vec<Card> {
    s.chars().map(parse_card).collect()
}

fn parse_card(c: char) -> Card {
    match c {
        'A' => Card::A,
        'K' => Card::K,
        'Q' => Card::Q,
        'J' => Card::J,
        'T' => Card::T,
        '9' => Card::N9,
        '8' => Card::N8,
        '7' => Card::N7,
        '6' => Card::N6,
        '5' => Card::N5,
        '4' => Card::N4,
        '3' => Card::N3,
        '2' => Card::N2,
        _ => panic!("Invalid card")
    }
}

#[derive(PartialEq, PartialOrd, Eq, Debug, Clone, Copy, Ord)]
enum Card2 {
    A, K, Q, T, N9, N8, N7, N6, N5, N4, N3, N2, J
}

fn parse_hand2(s: &str) -> Vec<Card2> {
    s.chars().map(parse_card2).collect()
}

fn parse_card2(c: char) -> Card2 {
    match c {
        'A' => Card2::A,
        'K' => Card2::K,
        'Q' => Card2::Q,
        'J' => Card2::J,
        'T' => Card2::T,
        '9' => Card2::N9,
        '8' => Card2::N8,
        '7' => Card2::N7,
        '6' => Card2::N6,
        '5' => Card2::N5,
        '4' => Card2::N4,
        '3' => Card2::N3,
        '2' => Card2::N2,
        _ => panic!("Invalid card")
    }
}

#[derive(PartialEq, Eq)]
struct Hand {
    cards: Vec<Card>,
    bid: usize,
}

impl Hand {
    fn of_a_kind(&self) -> Vec<usize> {
        let mut counters = BTreeMap::<Card, usize>::new();
        for card in self.cards.iter() {
            *(counters.entry(*card).or_default()) += 1;
        }
        let mut result: Vec<usize> = counters.values().filter(|n| **n > 0).copied().collect();
        result.sort_by(|a,b| b.cmp(a));
        result
    }

    fn score(&self) -> usize {
        let of_a_kind = self.of_a_kind();
        if of_a_kind[0] == 5 {
            110
        } else if of_a_kind[0] == 4 {
            100
        } else if of_a_kind[0] == 3 && of_a_kind[1] == 2 {
            90
        } else if of_a_kind[0] == 3 {
            80
        } else if of_a_kind[0] == 2 && of_a_kind[1] == 2{
            70
        } else if of_a_kind[0] == 2 {
            60
        } else {
            0
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Hand) -> Option<Ordering> {
        let score1: usize = self.score();
        let score2 = other.score();

        if score1 == score2 {
            self.cards.partial_cmp(&other.cards)
        } else {
            score2.partial_cmp(&score1)
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Hand) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

#[derive(PartialEq, Eq, Clone)]
struct Hand2 {
    cards: Vec<Card2>,
    bid: usize,
    jokers: usize,
}

impl Hand2 {
    fn of_a_kind(&self) -> Vec<usize> {
        let mut counters = BTreeMap::<Card2, usize>::new();
        for card in self.cards.iter() {
            if *card != Card2::J {
                *(counters.entry(*card).or_default()) += 1;
            }
        }
        let mut result: Vec<usize> = counters.values().copied().collect();
        result.sort_by(|a,b| b.cmp(a));
        result.push(0);
        result.push(0);
        result
    }

    fn score(&self) -> usize {
        let of_a_kind = self.of_a_kind();
        if of_a_kind[0] + self.jokers == 5 {
            7
        } else if of_a_kind[0] + self.jokers == 4 {
            6
        } else if of_a_kind[0] + self.jokers == 3 && of_a_kind[1] == 2  {
            5
        } else if of_a_kind[0] + self.jokers == 3 {
            4
        } else if of_a_kind[0] == 2 && of_a_kind[1] == 2 {
            3
        } else if of_a_kind[0] + self.jokers == 2 {
            2
        } else {
            1
        }
    }
}

impl PartialOrd for Hand2 {
    fn partial_cmp(&self, other: &Hand2) -> Option<Ordering> {
        let score1: usize = self.score();
        let score2 = other.score();

        if score1 == score2 {
            self.cards.partial_cmp(&other.cards)
        } else {
            score2.partial_cmp(&score1)
        }
    }
}

impl Ord for Hand2 {
    fn cmp(&self, other: &Hand2) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

fn main() {
    let input_str = read_to_string("input.txt").unwrap();
    let mut input = input_str.lines().filter(|l| !l.is_empty()).map(|l| {
        let (cards_str, rank_str) = l.split_once(" ").unwrap();
        Hand { cards: parse_hand(cards_str), bid: rank_str.parse().unwrap() }
    }).collect::<Vec<Hand>>();
    input.sort();

    let result = input.iter().rev().enumerate().fold(0, |acc, (i, hand)| acc + (i+1) * hand.bid);

    println!("result1: {}", result);

    let mut input2 = input_str.lines().filter(|l| !l.is_empty()).map(|l| {
        let (cards_str, rank_str) = l.split_once(" ").unwrap();
        let cards = parse_hand2(cards_str);
        Hand2 { bid: rank_str.parse().unwrap(), jokers: cards.iter().filter(|c| **c == Card2::J).count(), cards }
    }).collect::<Vec<Hand2>>();

    input2.sort();

    let result2 = input2.iter().rev().enumerate().fold(0, |acc, (i, hand)| acc + (i+1) * hand.bid);

    println!("result2: {}", result2);
}

