use std::collections::HashSet;

pub fn part_one(input: &str) -> Option<u32> {
    solve(input, 4)
}
pub fn part_two(input: &str) -> Option<u32> {
    solve(input, 14)
}

fn solve(input: &str, required_len: usize) -> Option<u32> {
    let mut stack: Vec<u32> = Default::default();
    let chars = input.trim().chars();
    let mut i = 0;
    for c in chars {
        i += 1;
        stack.push(c as u32);
        if is_mark(&stack, required_len) {
            return Some(i);
        }
    }
    None
}

fn is_mark(stack: &[u32], required_len: usize) -> bool {
    if stack.len() < required_len {
        return false;
    };

    let max = stack.len() - 1;
    let mut set = HashSet::new();
    for i in 0..required_len {
        set.insert(stack[max - i]);
    }

    set.len() == required_len
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 6);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_one(&input), Some(7));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_two(&input), Some(19));
    }
}
