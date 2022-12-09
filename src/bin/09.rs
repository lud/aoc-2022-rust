use std::cmp::Ordering;
use std::collections::HashSet;
pub fn part_one(input: &str) -> Option<usize> {
    let moves = input
        .trim()
        .split('\n')
        .map(parse_line)
        .flat_map(expand_step);

    let total_places = run_sim_simple(moves.collect());

    Some(total_places)
}

pub fn part_two(input: &str) -> Option<usize> {
    let moves = input
        .trim()
        .split('\n')
        .map(parse_line)
        .flat_map(expand_step);

    let total_places = run_sim_rope(moves.collect());

    Some(total_places)
}

fn run_sim_simple(moves: Vec<Mv>) -> usize {
    let mut set: HashSet<(i32, i32)> = HashSet::new();
    let mut h_pos = (0, 0);
    let mut t_pos = (0, 0);

    set.insert(t_pos);

    for mv in moves {
        h_pos = apply_move(mv, h_pos);
        if !touching(&h_pos, &t_pos) {
            t_pos = seek_pos(&t_pos, &h_pos);
        }
        set.insert(t_pos);
    }

    set.len()
}

fn run_sim_rope(moves: Vec<Mv>) -> usize {
    let mut set: HashSet<(i32, i32)> = HashSet::new();
    let mut h_pos = (0, 0);
    let mut rope = [
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
        (0, 0),
    ];

    set.insert(h_pos);

    for mv in moves {
        h_pos = apply_move(mv, h_pos);
        scan_moves(h_pos, &mut rope);
        set.insert(rope[8]);
    }

    set.len()
}

fn scan_moves(h_pos: (i32, i32), rope: &mut [(i32, i32); 9]) {
    let mut prev = h_pos;

    for knot in rope {
        if !touching(&prev, knot) {
            *knot = seek_pos(knot, &prev);
        }
        prev = *knot;
    }
}

fn seek_pos((xt, yt): &(i32, i32), (xh, yh): &(i32, i32)) -> (i32, i32) {
    // this function is only called when there is an actual movement of the tail
    // to perform, so we can safely seek towards the same y and x as the head.
    let xdiff = abs_diff(xt, xh);
    let ydiff = abs_diff(yt, yh);

    if ydiff == xdiff {
        let new_xt = advance_towards_diff(*xt, *xh, 1);
        let new_yt = advance_towards_diff(*yt, *yh, 1);
        (new_xt, new_yt)
    } else if ydiff > 1 {
        let new_yt = advance_towards_diff(*yt, *yh, 1);
        let new_xt = advance_towards_diff(*xt, *xh, 0);
        (new_xt, new_yt)
    } else if xdiff > 1 {
        let new_xt = advance_towards_diff(*xt, *xh, 1);
        let new_yt = advance_towards_diff(*yt, *yh, 0);
        (new_xt, new_yt)
    } else {
        panic!("unexpected seek");
    }
}

fn advance_towards_diff(t: i32, h: i32, min_dist: i32) -> i32 {
    let mut new_t = t;
    let mut stopper = 0;
    match h.cmp(&t) {
        Ordering::Greater => {
            while abs_diff(&h, &new_t) > min_dist {
                stopper += 1;
                if stopper > 100 {
                    panic!("infinite loop");
                }
                new_t += 1;
            }
            new_t
        }
        Ordering::Less => {
            while abs_diff(&h, &new_t) > min_dist {
                stopper += 1;
                if stopper > 100 {
                    panic!("infinite loop");
                }
                new_t -= 1;
            }
            new_t
        }
        _ => {
            assert!(t == h);
            new_t
        }
    }
}

fn touching((xa, ya): &(i32, i32), (xb, yb): &(i32, i32)) -> bool {
    let xdiff = abs_diff(xa, xb);

    if xdiff > 1 {
        return false;
    }

    let ydiff = abs_diff(ya, yb);
    if ydiff > 1 {
        return false;
    }

    true
}

fn abs_diff(xa: &i32, xb: &i32) -> i32 {
    if (xa > &0 && xb > &0) || (xa < &0 && xb < &0) {
        (xa.abs() - xb.abs()).abs()
    } else {
        xa.abs() + xb.abs()
    }
}

fn apply_move(mv: Mv, (x, y): (i32, i32)) -> (i32, i32) {
    if mv.amount != 1 {
        panic!("only moves of 1 are supported");
    }

    match mv.direction {
        Direction::U => (x, y - 1),
        Direction::D => (x, y + 1),
        Direction::L => (x - 1, y),
        Direction::R => (x + 1, y),
    }
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    U,
    D,
    L,
    R,
}
#[derive(Debug)]
struct Mv {
    direction: Direction,
    amount: u32,
}

fn parse_line(s: &str) -> Mv {
    let (s_move, steps) = s.split_once(' ').unwrap();
    let amount: u32 = steps.parse().unwrap();
    let direction = match s_move {
        "R" => Direction::R,
        "U" => Direction::U,
        "D" => Direction::D,
        "L" => Direction::L,
        &_ => panic!("unexpected char"),
    };

    Mv { direction, amount }
}

fn expand_step(mv: Mv) -> Vec<Mv> {
    let mut vec = vec![];
    for _ in 1..=mv.amount {
        vec.push(Mv {
            direction: mv.direction,
            amount: 1,
        });
    }
    vec
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 9);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_one(&input), Some(13));
    }

    #[test]
    fn test_part_one_real() {
        let input = advent_of_code::read_file("inputs", 9);
        assert_eq!(part_one(&input), Some(6498));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_two(&input), Some(1));
    }
}
