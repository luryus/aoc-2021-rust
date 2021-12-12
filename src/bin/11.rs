use std::io;
use itertools::Itertools;
use ndarray::Array2;

fn adjacents(x: usize, y: usize, w: usize, h: usize) -> impl Iterator<Item = (usize, usize)> {
    (x.saturating_sub(1)..=(x+1).min(w-1))
        .cartesian_product(y.saturating_sub(1)..=(y+1).min(h-1))
        .filter(move |c| *c != (x, y))
}


fn part1(input: &Array2<usize>) -> usize {
    let mut arr = input.clone();
    let mut total_flashes = 0;
    for i in 0..100 {
        // dbg!(&arr);
        arr += 1;
        let mut flashes: Array2<bool> = Array2::default(arr.raw_dim());

        loop {
            let new_flashes: Vec<_> = arr.indexed_iter()
                .filter(|&(c, v)| *v > 9 && !flashes[c])
                .map(|(c, _)| c)
                .collect();
            // dbg!(&new_flashes);
            if new_flashes.is_empty() {
                break;
            }
            for c in new_flashes {
                flashes[c] = true;
                for (ax, ay) in adjacents(c.1, c.0, 10, 10) {
                    // dbg!(&(ax, ay));
                    arr[(ay, ax)] += 1;
                }
            }
        }

        for (c, _) in flashes.indexed_iter().filter(|(_, f)| **f) {
            arr[c] = 0;
            total_flashes += 1;
        }
    }

    total_flashes
}


fn part2(input: &Array2<usize>) -> usize {
    let mut arr = input.clone();
    for i in 0.. {
        // dbg!(&arr);
        arr += 1;
        let mut flashes: Array2<bool> = Array2::default(arr.raw_dim());

        loop {
            let new_flashes: Vec<_> = arr.indexed_iter()
                .filter(|&(c, v)| *v > 9 && !flashes[c])
                .map(|(c, _)| c)
                .collect();
            // dbg!(&new_flashes);
            if new_flashes.is_empty() {
                break;
            }
            for c in new_flashes {
                flashes[c] = true;
                for (ax, ay) in adjacents(c.1, c.0, 10, 10) {
                    // dbg!(&(ax, ay));
                    arr[(ay, ax)] += 1;
                }
            }
        }

        if flashes.iter().filter(|f| **f).count() == flashes.len() {
                return i + 1
        }

        for (c, _) in flashes.indexed_iter().filter(|(_, f)| **f) {
            arr[c] = 0;
        }
    }

    unreachable!()
}

fn main() -> io::Result<()> {
    let input = aoc2021::read_input_lines()?;
    let line_width = input[0].len();
    let input = input
        .iter()
        .flat_map(|l| l.chars())
        .map(|c: char| c.to_digit(10).unwrap() as usize)
        .collect_vec();
    let input = Array2::from_shape_vec((input.len() / line_width, line_width), input).unwrap();

    let p1 = part1(&input);
    println!("Part 1: {}", p1);

    let p2 = part2(&input);
    println!("Part 2: {}", p2);

    Ok(())
}
