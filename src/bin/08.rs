use std::{collections::HashSet, vec};

pub fn part_one(input: &str) -> Option<usize> {
    let mut map = parse_grid(input);
    let mut visibles: HashSet<(i32, i32)> = calc_horizontal_visibles(&map);
    map = flip_matrix(map);
    let max_col = (map[0].len() - 1) as i32;
    let visibles_2: HashSet<(i32, i32)> = flip_set(calc_horizontal_visibles(&map), max_col);
    visibles.extend(&visibles_2);
    Some(visibles.len())
}

fn flip_set(set: HashSet<(i32, i32)>, max_col: i32) -> HashSet<(i32, i32)> {
    // we flipped -90° so row (y) becomes col (x), first row is first colum
    // but last column is now first row
    set.iter().map(|(x, y)| (max_col - (*y), *x)).collect()
}

fn flip_matrix(map: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    // rotate 90° to the left (-90°) so the rightmost column of the grid becomes
    // the first row

    let rows = map.len();
    let cols = (map[0]).len();

    let mut new_map = vec![];

    for icol in (0..cols).rev() {
        let mut flipped_row = vec![];
        for irow in (0..rows).rev() {
            flipped_row.push(map[irow][icol]);
        }
        flipped_row.reverse();
        new_map.push(flipped_row);
    }

    new_map
}

fn calc_horizontal_visibles(map: &[Vec<i32>]) -> HashSet<(i32, i32)> {
    let mut set: HashSet<(i32, i32)> = HashSet::new();
    for (irow, line) in map.iter().enumerate() {
        calc_line_horiz_visible(line, &(irow as i32), &mut set);
    }
    set
}

fn calc_line_horiz_visible(line: &[i32], irow: &i32, set: &mut HashSet<(i32, i32)>) {
    let mut max = -1;
    let mut icol = 0;
    for n in line {
        if n > &max {
            set.insert((icol, *irow));
            max = *n;
        }
        icol += 1;
    }

    max = -1;
    for n in line.iter().rev() {
        icol -= 1;
        if n > &max {
            max = *n;
            set.insert((icol, *irow));
        }
    }

    if icol != 0 {
        panic!("expected 0")
    }
}

fn parse_grid(input: &str) -> Vec<Vec<i32>> {
    input
        .trim()
        .split('\n')
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as i32)
                .collect()
        })
        .collect()
}

pub fn part_two(input: &str) -> Option<i32> {
    let map = parse_grid(input);
    let max_x = map.len() - 1;
    let max_y = map[0].len() - 1;

    let mut best: i32 = 0;
    for x in 0..=max_x {
        for y in 0..=max_y {
            let candidate = get_score(&map, &x, &y, &max_x, &max_y);
            if candidate > best {
                best = candidate;
            }
        }
    }

    Some(best)
}

#[allow(clippy::needless_range_loop)]
fn get_score(map: &[Vec<i32>], hx: &usize, hy: &usize, max_x: &usize, max_y: &usize) -> i32 {
    let house_height = map[*hy][*hx];

    let mut count_left = 0;
    let mut count_right = 0;
    let mut count_top = 0;
    let mut count_down = 0;

    if *hx > 0 {
        for x in (0..*hx).rev() {
            count_left += 1;
            if map[*hy][x] >= house_height {
                break;
            }
        }
    }

    if hx < max_x {
        for x in (hx + 1)..=*max_x {
            count_right += 1;
            if map[*hy][x] >= house_height {
                break;
            }
        }
    }

    if *hy > 0 {
        for y in (0..*hy).rev() {
            count_top += 1;
            if map[y][*hx] >= house_height {
                break;
            }
        }
    }

    if hy < max_y {
        for y in (hy + 1)..=*max_y {
            count_down += 1;
            if map[y][*hx] >= house_height {
                break;
            }
        }
    }

    count_left * count_right * count_down * count_top
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 8);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_one(&input), Some(21));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_two(&input), Some(8));
    }
}
