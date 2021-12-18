use itertools::Itertools;
use ndarray::Array2;
use std::{
    collections::{HashSet, VecDeque},
    io,
};

fn adjacents(x: usize, y: usize, w: usize, h: usize) -> impl Iterator<Item = (usize, usize)> {
    [
        if x > 0 { Some((x - 1, y)) } else { None },
        if x < w - 1 { Some((x + 1, y)) } else { None },
        if y > 0 { Some((x, y - 1)) } else { None },
        if y < h - 1 { Some((x, y + 1)) } else { None },
    ]
    .into_iter()
    .flatten()
}

fn get_low_points(input: &Array2<u32>) -> impl Iterator<Item = (usize, usize)> + '_ {
    let (w, h) = (input.ncols(), input.nrows());
    (0..w).cartesian_product(0..h).filter(move |&(x, y)| {
        let val = input[(y, x)];
        adjacents(x, y, w, h)
            .into_iter()
            .map(|(x, y)| input[(y, x)])
            .all(|v| v > val)
    })
}

fn part1(input: &Array2<u32>) -> u32 {
    get_low_points(input)
        .map(|(x, y)| input[(y, x)] + 1)
        .sum::<u32>()
}

fn part2(input: &Array2<u32>) -> usize {
    let (w, h) = (input.ncols(), input.nrows());

    get_low_points(input)
        .map(|(lpx, lpy)| {
            let mut visited = HashSet::new();
            let mut to_visit = VecDeque::new();
            to_visit.push_back((lpx, lpy));
            while let Some((xx, yy)) = to_visit.pop_front() {
                visited.insert((xx, yy));
                let new = adjacents(xx, yy, w, h)
                    .filter(|&(x, y)| !visited.contains(&(x, y)) && input[(y, x)] != 9);
                to_visit.extend(new);
            }
            visited.len()
        })
        .sorted()
        .rev()
        .take(3)
        .product()
}

fn main() -> io::Result<()> {
    let input = aoc2021::read_input_lines()?;
    let line_width = input[0].len();
    let input = input
        .iter()
        .flat_map(|l| l.chars())
        .map(|c: char| c.to_digit(10).unwrap())
        .collect_vec();
    let input = Array2::from_shape_vec((input.len() / line_width, line_width), input).unwrap();

    let p1 = part1(&input);
    println!("Part 1: {}", p1);

    let p2 = part2(&input);
    println!("Part 2: {}", p2);

    Ok(())
}
