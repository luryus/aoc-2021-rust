use itertools::Itertools;
use ndarray::Array2;
use std::io;

fn adjacents_with_diagonal(
    c: (usize, usize),
    w: usize,
    h: usize,
) -> impl Iterator<Item = (usize, usize)> {
    let (y, x) = c;
    (y.saturating_sub(1)..=(y + 1).min(h - 1))
        .cartesian_product(x.saturating_sub(1)..=(x + 1).min(w - 1))
        .filter(move |c| *c != (y, x))
}

fn flash(arr: &mut Array2<u32>) -> Array2<bool> {
    let mut flashes: Array2<bool> = Array2::default(arr.raw_dim());

    loop {
        let new_flashes: Vec<_> = arr
            .indexed_iter()
            .filter(|&(c, v)| *v > 9 && !flashes[c])
            .map(|(c, _)| c)
            .collect();
        if new_flashes.is_empty() {
            return flashes;
        }
        for c in new_flashes {
            flashes[c] = true;
            for ac in adjacents_with_diagonal(c, 10, 10) {
                arr[ac] += 1;
            }
        }
    }
}

fn part1(input: &Array2<u32>) -> usize {
    let mut arr = input.clone();
    let mut total_flashes = 0;
    for _ in 0..100 {
        arr += 1;

        let flashes = flash(&mut arr);

        total_flashes += flashes.iter().filter(|f| **f).count();
        arr = &arr * flashes.map(|f| !f as u32);
    }

    total_flashes
}

fn part2(input: &Array2<u32>) -> usize {
    let mut arr = input.clone();
    for i in 0.. {
        arr += 1;

        let flashes = flash(&mut arr);

        if flashes.iter().all(|f| *f) {
            return i + 1;
        }

        arr = &arr * flashes.map(|f| !f as u32);
    }

    unreachable!()
}

fn main() -> io::Result<()> {
    let input = aoc2021::read_input_int_matrix()?;

    let p1 = part1(&input);
    println!("Part 1: {}", p1);

    let p2 = part2(&input);
    println!("Part 2: {}", p2);

    Ok(())
}
