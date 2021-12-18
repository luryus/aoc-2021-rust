use itertools::Itertools;
use std::io;

fn part1(input: &[usize]) -> usize {
    input
        .iter()
        .tuple_windows()
        .map(|(a, b)| (b > a) as usize)
        .sum()
}

fn part2(input: &[usize]) -> usize {
    input
        .iter()
        .tuple_windows()
        .map(|(a, b, c)| a + b + c)
        .tuple_windows()
        .map(|(a, b)| (b > a) as usize)
        .sum()
}

fn main() -> io::Result<()> {
    let input = aoc2021::read_input_ints(false)?;

    let p1 = part1(&input);
    println!("Part 1: {}", p1);

    let p2 = part2(&input);
    println!("Part 2: {}", p2);

    Ok(())
}
