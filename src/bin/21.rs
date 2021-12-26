use itertools::Itertools;
use std::{collections::HashMap, io};

fn part1(mut p1_pos: u32, mut p2_pos: u32) -> u32 {
    let mut p1 = 0;
    let mut p2 = 0;
    let mut turn = false;

    let moves = (1..=100).cycle().chunks(3);
    let moves = moves.into_iter().map(|c| c.into_iter().sum::<u32>());
    for (i, m) in moves.enumerate() {
        if turn {
            p2_pos = (p2_pos + m) % 10;
            if p2_pos == 0 {
                p2_pos = 10;
            }
            p2 += p2_pos;
            if p2 >= 1000 {
                return i as u32 * 3 * p1;
            }
        } else {
            p1_pos = (p1_pos + m) % 10;
            if p1_pos == 0 {
                p1_pos = 10;
            }
            p1 += p1_pos;
            println!("p1 to {}, score {}", p1_pos, p1);
            if p1 >= 1000 {
                return (1 + i) as u32 * 3 * p2;
            }
        }
        turn = !turn;
    }

    unreachable!()
}

fn p2_run(
    p1: u8,
    p2: u8,
    p1_pos: u8,
    p2_pos: u8,
    turn: bool,
    cache: &mut HashMap<(u8, u8, u8, u8, bool), (usize, usize)>,
) -> (usize, usize) {
    if let Some(cached) = cache.get(&(p1, p2, p1_pos, p2_pos, turn)) {
        return *cached;
    }

    let mut p1_wins = 0;
    let mut p2_wins = 0;
    for ((a, b), c) in (1..=3).cartesian_product(1..=3).cartesian_product(1..=3) {
        if turn {
            let mut new_p2_pos = (p2_pos + a + b + c) % 10;
            if new_p2_pos == 0 {
                new_p2_pos = 10;
            }
            let new_p2 = p2 + new_p2_pos;
            if new_p2 >= 21 {
                p2_wins += 1;
            } else {
                let (p1w, p2w) = p2_run(p1, new_p2, p1_pos, new_p2_pos, !turn, cache);
                p1_wins += p1w;
                p2_wins += p2w;
            }
        } else {
            let mut new_p1_pos = (p1_pos + a + b + c) % 10;
            if new_p1_pos == 0 {
                new_p1_pos = 10;
            }
            let new_p1 = p1 + new_p1_pos;
            if new_p1 >= 21 {
                p1_wins += 1;
            } else {
                let (p1w, p2w) = p2_run(new_p1, p2, new_p1_pos, p2_pos, !turn, cache);
                p1_wins += p1w;
                p2_wins += p2w;
            }
        }
    }

    cache.insert((p1, p2, p1_pos, p2_pos, turn), (p1_wins, p2_wins));
    (p1_wins, p2_wins)
}

fn part2(p1_pos: u32, p2_pos: u32) -> usize {
    let (p1_wins, p2_wins) = p2_run(0, 0, p1_pos as u8, p2_pos as u8, false, &mut HashMap::new());
    p1_wins.max(p2_wins)
}

fn main() -> io::Result<()> {
    let input = aoc2021::read_input_lines()?;
    let p1_start = input[0].chars().last().unwrap().to_digit(10).unwrap();
    let p2_start = input[1].chars().last().unwrap().to_digit(10).unwrap();

    let p1 = part1(p1_start, p2_start);
    println!("Part 1: {}", p1);

    let p2 = part2(p1_start, p2_start);
    println!("Part 2: {}", p2);

    Ok(())
}
