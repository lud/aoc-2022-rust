use std::str::FromStr;

pub fn part_one(input: &str) -> Option<i32> {
    let prog: Vec<Instr> = input.trim().split('\n').map(parse).collect();
    let sum = run_prog(prog);
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let prog: Vec<Instr> = input.trim().split('\n').map(parse).collect();
    draw(prog);
    Some(0)
}

fn draw(prog: Vec<Instr>) {
    let mut reg_x: i32 = 1;
    let mut crt_pos = 0;
    for ins in prog {
        for _ic in 0..n_cycles(&ins) {
            let is_light: bool = check_light(&reg_x, &crt_pos);
            if is_light {
                print!("▮")
            } else {
                print!(" ")
            };
            crt_pos = next_pos(&crt_pos);
            if crt_pos == 0 {
                println!();
            }
        }

        match ins {
            Instr::Noop => {}
            Instr::AddX(v) => {
                reg_x += v;
            }
        };
    }
}

fn next_pos(crt_pos: &i32) -> i32 {
    if *crt_pos == 39 {
        0
    } else {
        crt_pos + 1
    }
}

fn check_light(reg_x: &i32, crt_pos: &i32) -> bool {
    crt_pos == reg_x || *crt_pos == (*reg_x - 1) || *crt_pos == (*reg_x + 1)
}

fn run_prog(prog: Vec<Instr>) -> i32 {
    let mut cycle: usize = 0;
    let mut reg_x: i32 = 1;
    let mut watch_cycle: usize = 20;
    let mut sum = 0;
    for ins in prog {
        if cycle == 20 && reg_x != 21 {
            panic!("expected 21, got: {:?}", reg_x);
        }
        // println!("-------------------");
        for _ic in 0..n_cycles(&ins) {
            cycle += 1;
            if cycle == watch_cycle {
                let signal = (cycle as i32) * reg_x;
                sum += signal;
                watch_cycle = next_watch(&cycle);
            }
        }

        match ins {
            Instr::Noop => {}
            Instr::AddX(v) => {
                reg_x += v;
            }
        };
    }
    sum
}

fn next_watch(cycle: &usize) -> usize {
    cycle + 40
}

fn parse(s: &str) -> Instr {
    s.parse().unwrap()
}

fn n_cycles(ins: &Instr) -> usize {
    match ins {
        Instr::Noop => 1,
        Instr::AddX(_) => 2,
    }
}

#[derive(Debug)]
enum Instr {
    AddX(i32),
    Noop,
}

impl FromStr for Instr {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "noop" {
            Ok(Instr::Noop)
        } else if s.starts_with("addx ") {
            let (_, v) = s.split_once(' ').unwrap();
            Ok(Instr::AddX(v.parse().unwrap()))
        } else {
            Err(format!("unknown instruction {:?}", s))
        }
    }
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 10);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 10);
        assert_eq!(part_one(&input), Some(13140));
    }
}
