pub fn part_one(input: &str) -> Option<u32> {
    input.trim().split("\n\n").map(sum_ints).max()
}

fn sum_ints(x: &str) -> u32 {
    let iter = x.split('\n').map(|snum| -> u32 { snum.parse().unwrap() });
    iter.sum()
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut all: Vec<u32> = input.trim().split("\n\n").map(sum_ints).collect();
    all.sort();
    all.reverse();
    Some(all.iter().take(3).sum())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_one(&input), Some(24000));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), Some(45000));
    }
}
