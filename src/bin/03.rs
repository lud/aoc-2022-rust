use std::{collections::HashSet, slice::Chunks};

pub fn part_one(input: &str) -> Option<u32> {
    let x = input.trim().split('\n').map(handle_line).sum();
    Some(x)
}

fn handle_line(line: &str) -> u32 {
    let chars: Vec<char> = line.chars().collect();
    let half: usize = chars.len() / 2;
    let (left, right) = chars.split_at(half);
    let common = common_char(vec![left.into(), right.into()]);
    score_of(common)
}

fn common_char(lists: Vec<Vec<char>>) -> char {
    **lists
        .iter()
        .map(|char_a| -> HashSet<&char> { char_a.iter().collect() })
        .reduce(|acc, hash| -> HashSet<&char> { acc.intersection(&hash).copied().collect() })
        .unwrap()
        .iter()
        .next()
        .unwrap()
}

fn score_of(c: char) -> u32 {
    let d = c as u32;
    if ('A'..='Z').contains(&c) {
        d - 38
    } else if ('a'..='z').contains(&c) {
        d - 96
    } else {
        panic!("unknown char {}", c)
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines = input.trim().split('\n').collect::<Vec<&str>>();
    let chunks: Chunks<'_, &str> = lines.chunks(3);

    let sum = chunks
        .map(|chunk| chunk.iter().map(|line| line.chars().collect()).collect())
        .map(common_char)
        .map(score_of)
        .sum();

    Some(sum)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 3);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_one(&input), Some(157));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_two(&input), Some(70));
    }
}
