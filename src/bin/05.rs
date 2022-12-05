use std::vec;

pub fn part_one(input: &str) -> Option<String> {
    let (map, movelist) = input.split_once("\n\n").unwrap();
    let mut state = parse_map(map);
    let moves = parse_moves(movelist);
    apply_moves_9000(&mut state, moves);
    let message: String = collect_last_chars(state);

    Some(message)
}

pub fn part_two(input: &str) -> Option<String> {
    let (map, movelist) = input.split_once("\n\n").unwrap();
    let mut state = parse_map(map);
    let moves = parse_moves(movelist);
    apply_moves_9001(&mut state, moves);
    let message: String = collect_last_chars(state);

    Some(message)
}

fn collect_last_chars(state: Vec<Vec<char>>) -> String {
    let mut topchars: Vec<char> = vec![];
    for v in state {
        if !v.is_empty() {
            topchars.push(*(v.last().unwrap()));
        }
    }
    topchars.into_iter().collect()
}

fn parse_map(map: &str) -> Vec<Vec<char>> {
    let mut state: Vec<Vec<char>> = Default::default();
    let lines: Vec<Vec<char>> = map.split('\n').rev().skip(1).map(parse_map_line).collect();

    for line in lines {
        for (i, c) in line.iter().enumerate() {
            if *c != '_' {
                if state.len() <= i {
                    state.push(vec![]);
                }
                state[i].push(*c);
            }
        }
    }

    state
}

fn apply_moves_9000(state: &mut [Vec<char>], moves: Vec<Move>) {
    for Move(amount, from, to) in moves {
        for _ in 1..=amount {
            let val = state[from].pop().unwrap();
            state[to].push(val);
        }
    }
}

fn apply_moves_9001(state: &mut [Vec<char>], moves: Vec<Move>) {
    for Move(amount, from, to) in moves {
        let mut temp = vec![];
        for _ in 1..=amount {
            let val = state[from].pop().unwrap();
            temp.push(val);
        }
        temp.reverse();
        state[to].append(&mut temp);
    }
}

fn parse_map_line(line: &str) -> Vec<char> {
    let mut letters: Vec<char> = Default::default();

    let iter = Vec::from_iter(line.chars());
    let chunks = iter.chunks(4);

    for ch in chunks {
        if ch[1] == ' ' {
            letters.push('_');
        } else {
            letters.push(ch[1]);
        }
    }
    letters
}

#[derive(Debug)]
struct Move(usize, usize, usize);

fn parse_moves(raw: &str) -> Vec<Move> {
    raw.trim().split('\n').map(parse_move).collect()
}

fn parse_move(line: &str) -> Move {
    // "move 1 from 1 to 2"

    let parts = Vec::from_iter(line.split(' '));
    if parts[0] != "move" {
        panic!("expected move");
    }
    if parts[2] != "from" {
        panic!("expected from");
    }
    if parts[4] != "to" {
        panic!("expected to");
    }
    let num_parts: usize = parts[1].parse().unwrap();
    let stack_from: usize = parts[3].parse().unwrap();
    let stack_to: usize = parts[5].parse().unwrap();

    Move(num_parts, stack_from - 1, stack_to - 1)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 5);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn chars() {
        let str = "   hello";
        let chars = str.chars();
        dbg!(&chars);
        assert_eq!(8, chars.count());
    }

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_one(&input), Some(String::from("CMZ")));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_two(&input), Some(String::from("MCD")));
    }
}
