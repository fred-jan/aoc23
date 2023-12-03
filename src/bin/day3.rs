use std::fs;

fn pt1_sum(contents: String) -> u32 {
    contents.lines()
        .enumerate()
        .map(|(i_line, line)| {
            line.chars()
                .enumerate()
                // Only digits with no preceding digit (start of a number)
                .filter(|(i_char, char)| {
                    char.is_digit(10) &&
                        (i_char == &0usize || !line.as_bytes()[i_char - 1].is_ascii_digit())
                })
                // Transform to tuples of numbers with their starting index
                .map(|(i_char, _char)| {
                    (i_char, line[i_char..].chars()
                        .take_while(|char| char.is_digit(10))
                        .collect::<String>()
                        .parse::<u32>()
                        .unwrap())
                })
                // Only keep numbers without neighbouring symbols
                .filter(|(i_number, number)| {
                    contents.lines()
                        .enumerate()
                        // Previous line until next line
                        .filter(|(i, _)| (i_line.saturating_sub(1)..i_line.checked_add(2).unwrap_or(i_line)).contains(i))
                        // Only keep lines that have one or more symbols in surrounding columns
                        .filter(|(_, line)| {
                            line.chars()
                                .skip(i_number.clone().saturating_sub(1)) // col start
                                .take(number.to_string().len() + 2) // col end
                                .filter(|char| !char.is_digit(10) && char.clone() != '.')
                                .count() > 0
                        })
                        // Only keep numbers with neighbouring symbols
                        .count() > 0
                })
                .map(|(_, number)| number)
                .sum::<u32>()
        })
        .sum()
}

fn main() {
    let contents = fs::read_to_string("inputs/day3.txt").expect("Failed to read input");

    println!("Part 1: {}", pt1_sum(contents));
}

#[test]
fn test_part1() {
    assert_eq!(pt1_sum(fs::read_to_string("samples/day3.txt").expect("Failed to read input")), 4361);
}