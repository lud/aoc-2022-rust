use std::collections::HashMap;

pub fn part_one(input: &str) -> Option<u32> {
    let map = gen_map(input);

    let sum = map
        .iter()
        .filter(|(_, v)| **v <= 100_000)
        .fold(0, |acc: u32, (_, v)| v + acc);
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let map = gen_map(input);

    let total_size: u32 = 70_000_000;
    let used_size = *map.get("/").unwrap();
    let remaining_size = total_size - used_size;
    let needed_size = 30_000_000;
    let freeable_size = needed_size - remaining_size;

    map.iter()
        .filter(|(_, v)| **v >= freeable_size)
        .map(|(_, v)| *v)
        .min()
}

pub fn gen_map(input: &str) -> HashMap<String, u32> {
    let mut map: HashMap<String, u32> = HashMap::new();
    input
        .trim()
        .split('\n')
        .map(tokenize)
        .fold(vec![], |x: Vec<&str>, tok| reduce_sizes(x, tok, &mut map));
    map
}

fn reduce_sizes<'a>(
    path: Vec<&'a str>,
    token: Token<'a>,
    map: &mut HashMap<String, u32>,
) -> Vec<&'a str> {
    match token {
        Token::CD("/") => {
            vec![""]
        }
        Token::CD("..") => {
            let mut p = path.clone();
            p.pop();
            p
        }
        Token::CD(other) => {
            let mut p = path.clone();
            p.push(other);
            p
        }
        Token::File(size) => {
            let allpaths = calc_paths(&path);
            for subpath in allpaths {
                let sum = map.entry(subpath).or_insert(0);
                // unwrap_or_else(|v| v.insert(0));
                *sum += size;
            }
            path
        }
        _ => path,
    }
}

fn calc_paths(path: &[&str]) -> Vec<String> {
    path.iter()
        .scan(String::from(""), |state, dir| {
            if state != "/" {
                state.push('/');
            }
            state.push_str(dir);
            let out = state.to_owned();
            Some(out)
        })
        .collect()
}

// fn reduce_sizes(x: &mut HashMap<&str, u32>, y: Token) -> &mut HashMap<&str, u32> {}s

#[derive(Debug)]
enum Token<'a> {
    CD(&'a str),
    File(u32),
    Ignore,
}

fn tokenize(line: &str) -> Token {
    if line.starts_with('$') {
        if line.starts_with("$ cd") {
            let split = line.splitn(3, ' ');
            return Token::CD(split.last().unwrap());
        }
    } else if !line.starts_with("dir ") {
        let (len, _fname) = line.split_once(' ').unwrap();
        let size: u32 = len.parse().unwrap();
        return Token::File(size);
    }
    Token::Ignore
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 7);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_one(&input), Some(95437));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_two(&input), Some(24933642));
    }
}
