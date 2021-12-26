use itertools::Itertools;
use ndarray::Array2;
use std::{collections::BinaryHeap, io};

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

#[derive(PartialEq, Eq)]
struct DistanceCoord(u32, (usize, usize));
impl PartialOrd for DistanceCoord {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        other.0.partial_cmp(&self.0)
    }
}
impl Ord for DistanceCoord {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.partial_cmp(&other.0).unwrap()
    }
}

fn run(input: &Array2<u32>) -> u32 {
    let mut visited: Array2<bool> = Array2::default(input.raw_dim());

    let mut pq = BinaryHeap::new();
    pq.push(DistanceCoord(0, (0, 0)));

    while let Some(DistanceCoord(d, c)) = pq.pop() {
        if visited[c] {
            continue;
        }
        visited[c] = true;

        if c == (input.nrows() - 1, input.ncols() - 1) {
            return d;
        }

        let adjacents = adjacents(c.1, c.0, input.ncols(), input.nrows())
            .map(|(x, y)| (y, x))
            .filter(|c| !visited[*c]);
        for ac in adjacents {
            let ad = d + input[ac];
            pq.push(DistanceCoord(ad, ac));
        }
    }

    unreachable!()
}

fn part2(input: &Array2<u32>) -> u32 {
    let mut full_input = Array2::zeros((input.ncols() * 5, input.nrows() * 5));
    for x in 0..5 {
        for y in 0..5 {
            for ((ix, iy), v) in input.indexed_iter() {
                full_input[(y * input.ncols() + iy, x * input.nrows() + ix)] =
                    v + x as u32 + y as u32;
            }
        }
    }

    full_input %= 9;
    full_input += 1;

    //for v in full_input.iter_mut() {
    //    if *v >= 10 {
    //        *v -= *v / 9 * 9;
    //    }
    // }

    run(&full_input)
}

fn main() -> io::Result<()> {
    let input = aoc2021::read_input_lines()?;
    let input = input
        .into_iter()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect_vec())
        .collect_vec();
    let input = aoc2021::make_2d_array(input).unwrap();

    let p1 = run(&input);
    println!("Part 1: {}", p1);

    let p2 = part2(&input);
    println!("Part 2: {}", p2);

    Ok(())
}
