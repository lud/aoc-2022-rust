pub fn part_one(input: &str) -> Option<u32> {
    count_matching_pairs(input, is_any_containing)
}

pub fn part_two(input: &str) -> Option<u32> {
    count_matching_pairs(input, is_any_overlapping)
}

fn count_matching_pairs(input: &str, filter: fn(&(ContRange, ContRange)) -> bool) -> Option<u32> {
    let x: u32 = input
        .trim()
        .split('\n')
        .map(to_pair_of_ranges)
        .filter(filter)
        .count()
        .try_into()
        .unwrap();
    Some(x)
}

struct ContRange {
    from: u32,
    to: u32,
}

fn to_pair_of_ranges(x: &str) -> (ContRange, ContRange) {
    let mut raws = x.split(',');

    (
        parse_range(raws.next().unwrap()),
        parse_range(raws.next().unwrap()),
    )
}

fn parse_range(s: &str) -> ContRange {
    let mut left = s.split('-');
    let left_from: u32 = left.next().unwrap().parse().unwrap();
    let left_to: u32 = left.next().unwrap().parse().unwrap();

    ContRange {
        from: left_from,
        to: left_to,
    }
}

fn is_any_containing((left, right): &(ContRange, ContRange)) -> bool {
    (left.from <= right.from && left.to >= right.to)
        || (right.from <= left.from && right.to >= left.to)
}
fn is_any_overlapping((left, right): &(ContRange, ContRange)) -> bool {
    right.from >= left.from && right.from <= left.to
        || right.to >= left.from && right.to <= left.to
        || left.from >= right.from && left.from <= right.to
        || left.to >= right.from && left.to <= right.to
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 4);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_one(&input), Some(2));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_two(&input), Some(4));
    }
}
