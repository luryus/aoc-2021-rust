use std::io;
use itertools::Itertools;

fn part1(input: &Vec<usize>) -> usize {
    input.iter().tuple_windows()
        .map(|(a, b)| (b > a) as usize)
        .sum()
}

fn part2(input: &Vec<usize>) -> usize {
    input.iter()
        .tuple_windows()
        .map(|(a, b, c)| a + b + c)
        .tuple_windows()
        .map(|(a, b)| (b > a) as usize)
        .sum()
}

fn main() -> io::Result<()> {
    let input = aoc2021::read_input_ints()?;

    let p1 = part1(&input);
    println!("Part 1: {}", p1);

    let p2 = part2(&input);
    println!("Part 2: {}", p2);

    Ok(())
}
