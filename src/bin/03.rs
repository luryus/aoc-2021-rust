use itertools::Itertools;
use std::io;

fn positional_bit_counts(input: &Vec<String>) -> Vec<i32> {
    input
        .iter()
        .map(|l| {
            l.chars().map(|x| match x {
                '0' => -1,
                '1' => 1,
                _ => panic!("Invalid character"),
            })
        })
        .fold(vec![0; input[0].len()], |acc, b| {
            acc.into_iter().zip(b).map(|(a, b)| a + b).collect()
        })
}

fn get_bitmask(s: usize) -> usize {
    (1 << s) - 1
}

fn part1(input: &Vec<String>) -> usize {
    let counts = positional_bit_counts(input);

    let gamma = counts
        .iter()
        .map(|x| if *x > 0 { '1' } else { '0' })
        .collect::<String>();
    let gamma = usize::from_str_radix(&gamma, 2).unwrap();
    let epsilon = (!gamma) & get_bitmask(input[0].len());

    gamma * epsilon
}

fn part2(input: &Vec<String>) -> usize {
    let mut oxygen = None;

    let mut oxygen_candidates = input.clone();
    for i in 0..oxygen_candidates[0].len() {
        let counts = positional_bit_counts(&oxygen_candidates);
        oxygen_candidates.retain(|x| {
            if counts[i] >= 0 {
                x.chars().nth(i).unwrap() == '1'
            } else {
                x.chars().nth(i).unwrap() == '0'
            }
        });
        if oxygen_candidates.len() == 1 {
            oxygen = Some(oxygen_candidates[0].clone());
            break;
        }
    }
    let oxygen = oxygen.unwrap();

    let mut co2 = None;
    let mut co2_candidates = input.clone();
    for i in 0..co2_candidates[0].len() {
        let counts = positional_bit_counts(&co2_candidates);
        co2_candidates.retain(|x| {
            if counts[i] < 0 {
                x.chars().nth(i).unwrap() == '1'
            } else {
                x.chars().nth(i).unwrap() == '0'
            }
        });
        if co2_candidates.len() == 1 {
            co2 = Some(co2_candidates[0].clone());
            break;
        }
    }
    let co2 = co2.unwrap();

    let oxygen = usize::from_str_radix(&oxygen, 2).unwrap();
    let co2 = usize::from_str_radix(&co2, 2).unwrap();

    oxygen * co2
}

fn main() -> io::Result<()> {
    let input = aoc2021::read_input_lines()?;

    let p1 = part1(&input);
    println!("Part 1: {}", p1);

    let p2 = part2(&input);
    println!("Part 2: {}", p2);

    Ok(())
}
