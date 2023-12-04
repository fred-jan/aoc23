use std::cmp::max;
use std::fs;

#[derive(Debug)]
struct ScratchCard {
    win_numbers: Vec<u32>,
    card_numbers: Vec<u32>,
}

impl ScratchCard {
    fn matches(&self) -> Vec<&u32> {
        self.card_numbers
            .iter()
            .filter(|number| self.win_numbers.contains(number))
            .collect()
    }

    fn points(&self) -> u32 {
        self.matches().iter().fold(0, |acc, _| max(1, acc * 2))
    }
}

#[derive(Debug)]
struct Puzzle {
    cards: Vec<ScratchCard>,
}

impl Puzzle {
    fn load(contents: String) -> Self {
        Self {
            cards: contents
                .lines()
                .map(|line| line.split_once(":").unwrap())
                .map(|(_, line)| {
                    let (winning_numbers, card_numbers) = line.split_once(" | ").unwrap();

                    ScratchCard {
                        win_numbers: winning_numbers
                            .to_string()
                            .split_ascii_whitespace()
                            .map(|num| num.parse::<u32>().unwrap())
                            .collect(),
                        card_numbers: card_numbers
                            .to_string()
                            .split_ascii_whitespace()
                            .map(|num| num.parse::<u32>().unwrap())
                            .collect(),
                    }
                })
                .collect(),
        }
    }

    fn pt1(&self) -> u32 {
        self.cards.iter().map(|card| card.points()).sum()
    }
}

fn main() {
    println!(
        "Part 1: {}",
        Puzzle::load(fs::read_to_string("inputs/day4.txt").expect("Failed to read input")).pt1()
    );
}

#[test]
fn test_common() {
    assert_eq!(
        ScratchCard {
            win_numbers: vec![1, 2, 3, 4, 5, 6, 7],
            card_numbers: vec![2, 4, 5, 9]
        }
        .points(),
        4
    )
}

#[test]
fn test_part1() {
    assert_eq!(
        Puzzle::load(fs::read_to_string("samples/day4.txt").expect("Failed to read input")).pt1(),
        13
    );
}
