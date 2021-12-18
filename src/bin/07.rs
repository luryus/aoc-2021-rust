use itertools::Itertools;
use std::io;

fn part1(input: &[i32]) -> i32 {
    let (&min, &max) = input.iter().minmax().into_option().unwrap();

    (min..=max)
        .map(|i| input.iter().map(|x| (i - x).abs()).sum())
        .min()
        .unwrap()
}

fn part2(input: &[i32]) -> i32 {
    let (&min, &max) = input.iter().minmax().into_option().unwrap();

    (min..=max)
        .map(|i| {
            input
                .iter()
                .map(|x| {
                    let d = (i - x).abs();
                    d * (d + 1) / 2
                })
                .sum()
        })
        .min()
        .unwrap()
}

fn main() -> io::Result<()> {
    let input = aoc2021::read_input_ints::<i32>(false)?;

    let p1 = part1(&input);
    println!("Part 1: {}", p1);

    let p2 = part2(&input);
    println!("Part 2: {}", p2);

    Ok(())
}
