use itertools::Itertools;
use ndarray::Array2;
use std::{
    collections::{HashSet, VecDeque},
    io,
};

fn adjacents(c: (usize, usize), w: usize, h: usize) -> impl Iterator<Item = (usize, usize)> {
    let (y, x) = c;
    [
        if x > 0 { Some((y, x - 1)) } else { None },
        if x < w - 1 { Some((y, x + 1)) } else { None },
        if y > 0 { Some((y - 1, x)) } else { None },
        if y < h - 1 { Some((y + 1, x)) } else { None },
    ]
    .into_iter()
    .flatten()
}

fn get_low_points(input: &Array2<u32>) -> impl Iterator<Item = ((usize, usize), u32)> + '_ {
    let (w, h) = (input.ncols(), input.nrows());
    input.indexed_iter()
        .filter(move |&(c, &val)| {
            adjacents(c, w, h).map(|ac| input[ac]).all(|v| v > val)
        })
        .map(|(c, v)| (c, *v))
}

fn part1(input: &Array2<u32>) -> u32 {
    get_low_points(input)
        .map(|(_, v)| v + 1)
        .sum::<u32>()
}

fn part2(input: &Array2<u32>) -> usize {
    let (w, h) = (input.ncols(), input.nrows());

    get_low_points(input)
        .map(|(lpc, _)| {
            let mut visited = HashSet::new();
            let mut to_visit = VecDeque::new();
            to_visit.push_back(lpc);
            while let Some(vc) = to_visit.pop_front() {
                visited.insert(vc);
                let new = adjacents(vc, w, h)
                    .filter(|&ac| !visited.contains(&ac) && input[ac] != 9);
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
    let input = aoc2021::read_input_int_matrix()?;

    let p1 = part1(&input);
    println!("Part 1: {}", p1);

    let p2 = part2(&input);
    println!("Part 2: {}", p2);

    Ok(())
}
