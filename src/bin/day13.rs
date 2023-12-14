use std::fmt::Debug;
use std::fs;

#[derive(Debug)]
struct Pattern {
    contents: String,
    rows: Vec<String>,
    cols: Vec<String>,
}

impl Pattern {
    pub fn load(contents: String) -> Self {
        Self {
            contents: contents.to_string(),
            rows: contents.lines().map(|line| line.to_string()).collect(),
            cols: (0..contents.lines().nth(0).unwrap().len())
                .map(|col| {
                    contents
                        .lines()
                        .map(|line| line[col..col + 1].to_string())
                        .collect::<String>()
                })
                .collect(),
        }
    }

    pub fn reflection_rows(&self) -> usize {
        self.reflection_index(&self.rows)
            .and_then(|index| Some(index + 1))
            .unwrap_or(0)
    }

    pub fn reflection_cols(&self) -> usize {
        self.reflection_index(&self.cols)
            .and_then(|index| Some(index + 1))
            .unwrap_or(0)
    }

    pub fn fix_smudge(&self) -> Self {
        self.contents
            .chars()
            .enumerate()
            .filter(|(_, char)| ".#".contains(*char)) // no newlines
            .find_map(|(i, char)| {
                let fixed_pattern = Self::load(
                    self.contents
                        .chars()
                        .enumerate()
                        .map(|(i2, char2)| {
                            return if i2 == i {
                                match char2 {
                                    '.' => '#',
                                    _ => '.',
                                }
                            } else {
                                char2
                            };
                        })
                        .collect(),
                );

                if fixed_pattern.summary() > 0
                    && (fixed_pattern.reflection_rows() != self.reflection_rows()
                        || fixed_pattern.reflection_cols() != self.reflection_cols())
                {
                    return Some(fixed_pattern);
                }

                return None;
            })
            .unwrap()
        // .unwrap_or(Self {
        //     contents: self.contents.clone(),
        //     rows: self.rows.clone(),
        //     cols: self.cols.clone(),
        // })
    }

    pub fn summary(&self) -> usize {
        self.reflection_index(&self.rows)
            .and_then(|index| Some((index + 1) * 100))
            .or(self
                .reflection_index(&self.cols)
                .and_then(|index| Some(index + 1)))
            .unwrap_or(0)

        // self.reflection_cols() + self.reflection_rows() * 100
    }

    fn reflection_index(&self, lines: &Vec<String>) -> Option<usize> {
        lines[..lines.len() - 1]
            .iter()
            .enumerate()
            .find_map(|(index, _)| {
                let reflected_count = lines[0..index + 1]
                    .iter()
                    .rev()
                    .zip(&lines[index + 1..])
                    .filter(|(left, right)| left == right)
                    .count();
                let lines_left = index + 1;
                let lines_right = lines.len() - (index + 1);

                // Left lines exhausted -> perfect reflection
                if reflected_count == lines_left || reflected_count == lines_right {
                    return Some(index);
                }

                None
            })
    }
}

#[derive(Debug)]
struct Puzzle {
    patterns: Vec<Pattern>,
}

impl Puzzle {
    pub fn load(contents: String) -> Self {
        Self {
            patterns: contents
                .split("\n\n")
                .map(|section| Pattern::load(section.to_string()))
                .collect(),
        }
    }

    pub fn part1(&self) -> usize {
        self.patterns
            .iter()
            .fold(0, |acc, pattern| acc + pattern.summary())
    }

    pub fn part2(&self) -> usize {
        self.patterns
            .iter()
            .fold(0, |acc, pattern| acc + pattern.fix_smudge().summary())
    }
}

fn main() {
    // 29846
    println!(
        "Part 1: {}",
        Puzzle::load(fs::read_to_string("inputs/day13.txt").unwrap()).part1()
    );
    // Wrong: 38923 (too high)
    println!(
        "Part 2: {}",
        Puzzle::load(fs::read_to_string("inputs/day13.txt").unwrap()).part2()
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reflection() {
        let puzzle = &Puzzle::load(fs::read_to_string("samples/day13a.txt").unwrap());

        assert_eq!(puzzle.patterns[0].reflection_cols(), 5);
        assert_eq!(puzzle.patterns[1].reflection_rows(), 4);

        let puzzle = &Puzzle::load(fs::read_to_string("samples/day13b.txt").unwrap());

        assert_eq!(puzzle.patterns[0].reflection_rows(), 0);
        assert_eq!(puzzle.patterns[0].reflection_cols(), 3);

        assert_eq!(puzzle.patterns[1].reflection_rows(), 0);
        assert_eq!(puzzle.patterns[1].reflection_cols(), 11);

        assert_eq!(puzzle.patterns[2].reflection_rows(), 0);
        assert_eq!(puzzle.patterns[2].reflection_cols(), 1);
    }

    #[test]
    fn test_part1() {
        assert_eq!(
            Puzzle::load(fs::read_to_string("samples/day13a.txt").unwrap()).part1(),
            405
        );
    }

    #[test]
    fn test_part2() {
        let puzzle = &Puzzle::load(fs::read_to_string("samples/day13a.txt").unwrap());

        assert_eq!(puzzle.patterns[0].fix_smudge().reflection_rows(), 3);
        assert_eq!(puzzle.patterns[1].fix_smudge().reflection_rows(), 1);

        assert_eq!(puzzle.part2(), 400);
    }
}
