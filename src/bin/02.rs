#[derive(Clone, Copy, Debug)]
enum RPS {
    Rock,
    Paper,
    Scissor,
}

enum Wanted {
    Draw,
    IWin,
    ILose,
}

fn draw() -> u32 {
    3
}
fn iwin() -> u32 {
    6
}
fn ilose() -> u32 {
    0
}

fn score((elf, me): (RPS, RPS)) -> u32 {
    match (elf, me) {
        (RPS::Rock, RPS::Rock) => draw() + my_score(me),
        (RPS::Paper, RPS::Paper) => draw() + my_score(me),
        (RPS::Scissor, RPS::Scissor) => draw() + my_score(me),
        (RPS::Rock, RPS::Scissor) => ilose() + my_score(me),
        (RPS::Paper, RPS::Rock) => ilose() + my_score(me),
        (RPS::Scissor, RPS::Paper) => ilose() + my_score(me),
        (RPS::Rock, RPS::Paper) => iwin() + my_score(me),
        (RPS::Paper, RPS::Scissor) => iwin() + my_score(me),
        (RPS::Scissor, RPS::Rock) => iwin() + my_score(me),
    }
}

fn my_score(me: RPS) -> u32 {
    match me {
        RPS::Rock => 1,
        RPS::Paper => 2,
        RPS::Scissor => 3,
    }
}

fn parse_str(c: &str) -> RPS {
    match c {
        "A" => RPS::Rock,
        "B" => RPS::Paper,
        "C" => RPS::Scissor,
        "X" => RPS::Rock,
        "Y" => RPS::Paper,
        "Z" => RPS::Scissor,
        _ => panic!("unknown string {}", c),
    }
}

fn parse_elf(c: &str) -> RPS {
    match c {
        "A" => RPS::Rock,
        "B" => RPS::Paper,
        "C" => RPS::Scissor,
        _ => panic!("unknown string {}", c),
    }
}

fn parse_wanted(c: &str) -> Wanted {
    match c {
        "X" => Wanted::ILose,
        "Y" => Wanted::Draw,
        "Z" => Wanted::IWin,
        _ => panic!("unknown string {}", c),
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let x = input
        .trim()
        .split('\n')
        .map(|line| line.split(' ').map(parse_str).collect::<Vec<_>>())
        .map(|vec| (vec[0], vec[1]))
        .map(score)
        .sum();
    Some(x)
}

pub fn part_two(input: &str) -> Option<u32> {
    let x = input.trim().split('\n').map(parse_round).map(score).sum();
    Some(x)
}

fn parse_round(round: &str) -> (RPS, RPS) {
    let vec: Vec<&str> = round.split(' ').collect();
    let elf = parse_elf(vec[0]);
    let wanted = parse_wanted(vec[1]);
    (elf, what_to_play(elf, wanted))
}

fn what_to_play(elf: RPS, wanted: Wanted) -> RPS {
    match (elf, wanted) {
        (_, Wanted::Draw) => elf,
        (RPS::Rock, Wanted::IWin) => RPS::Paper,
        (RPS::Paper, Wanted::IWin) => RPS::Scissor,
        (RPS::Scissor, Wanted::IWin) => RPS::Rock,

        (RPS::Rock, Wanted::ILose) => RPS::Scissor,
        (RPS::Paper, Wanted::ILose) => RPS::Rock,
        (RPS::Scissor, Wanted::ILose) => RPS::Paper,
    }
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_one(&input), Some(15));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), Some(12));
    }
}
