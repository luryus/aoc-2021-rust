use itertools::Itertools;
use std::io;

enum NavRes {
    Incomplete(Vec<char>),
    Corrupted(char),
}

fn analyze(l: &str) -> NavRes {
    let mut stack = vec![];
    let valid_pairs = [('(', ')'), ('[', ']'), ('{', '}'), ('<', '>')];
    for c in l.chars() {
        if "([{<".contains(c) {
            stack.push(c);
        } else {
            if !valid_pairs.iter().contains(&(*stack.last().unwrap(), c)) {
                return NavRes::Corrupted(c);
            }
            stack.pop();
        }
    }

    NavRes::Incomplete(stack)
}

fn part1(input: &[String]) -> usize {
    input
        .iter()
        .filter_map(|l| match analyze(l) {
            NavRes::Corrupted(ill) => Some(ill),
            _ => None,
        })
        .map(|ill| match ill {
            ')' => 3,
            ']' => 57,
            '}' => 1197,
            '>' => 25137,
            _ => panic!("Invalid illegal char"),
        })
        .sum()
}

fn part2(input: &[String]) -> usize {
    let scores: Vec<_> = input
        .iter()
        .filter_map(|l| match analyze(l) {
            NavRes::Incomplete(stack) => Some(stack),
            _ => None,
        })
        .map(|s| {
            s.into_iter().rev().fold(0, |acc, c| {
                acc * 5
                    + match c {
                        '(' => 1,
                        '[' => 2,
                        '{' => 3,
                        '<' => 4,
                        _ => panic!("Invalid char in stack"),
                    }
            })
        })
        .sorted()
        .collect();

    scores[scores.len() / 2]
}

fn main() -> io::Result<()> {
    let input = aoc2021::read_input_lines()?;

    let p1 = part1(&input);
    println!("Part 1: {}", p1);

    let p2 = part2(&input);
    println!("Part 2: {}", p2);

    Ok(())
}
