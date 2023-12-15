use std::fmt::Debug;
use std::fs;

#[derive(Debug)]
struct Step {
    line: String,
}

impl Step {
    pub fn load(line: &str) -> Self {
        Self {
            line: line.to_string(),
        }
    }

    pub fn hash(&self) -> usize {
        self.line
            .chars()
            .fold(0, |acc, char| (acc + char as usize) * 17)
            % 256
    }
}

#[derive(Debug)]
struct Sequence {
    steps: Vec<Step>,
}

impl Sequence {
    pub fn load(contents: String) -> Self {
        Self {
            steps: contents
                .trim()
                .split(',')
                .map(|line| Step::load(line))
                .collect(),
        }
    }

    pub fn part1(&self) -> usize {
        self.steps.iter().fold(0, |acc, step| acc + step.hash())
    }
}

fn main() {
    let platform = Sequence::load(fs::read_to_string("inputs/day15.txt").unwrap());

    // 514394
    println!("Part 1: {}", platform.part1());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(Step::load("HASH").hash(), 52);
        assert_eq!(Step::load("rn=1").hash(), 30);
        assert_eq!(Step::load("cm-").hash(), 253);
        assert_eq!(Step::load("qp=3").hash(), 97);
        assert_eq!(Step::load("cm=2").hash(), 47);
        assert_eq!(Step::load("qp-").hash(), 14);
        assert_eq!(Step::load("pc=4").hash(), 180);
        assert_eq!(Step::load("ot=9").hash(), 9);
        assert_eq!(Step::load("ab=5").hash(), 197);
        assert_eq!(Step::load("pc-").hash(), 48);
        assert_eq!(Step::load("pc=6").hash(), 214);
        assert_eq!(Step::load("ot=7").hash(), 231);

        assert_eq!(
            Sequence::load(fs::read_to_string("samples/day15.txt").unwrap()).part1(),
            1320
        );
    }
}
