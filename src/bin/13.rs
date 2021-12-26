use itertools::Itertools;
use ndarray::Array2;
use std::io;

fn fold(paper: &Array2<bool>, fold_up: bool, fold_pos: usize) -> Array2<bool> {
    let mut folded = if fold_up {
        Array2::default((fold_pos, paper.ncols()))
    } else {
        Array2::default((paper.nrows(), fold_pos))
    };

    paper
        .indexed_iter()
        .filter(|(_, v)| **v)
        .map(|((y, x), _)| match fold_up {
            true if y > fold_pos => (2 * fold_pos - y, x),
            false if x > fold_pos => (y, 2 * fold_pos - x),
            _ => (y, x),
        })
        .for_each(|c| folded[c] = true);

    folded
}

fn part1(paper: &Array2<bool>, folds: &Vec<(bool, usize)>) -> usize {
    let (fold_up, fold_pos) = *folds.first().unwrap();
    fold(paper, fold_up, fold_pos)
        .into_iter()
        .filter(|v| *v)
        .count()
}

fn part2(paper: Array2<bool>, folds: &Vec<(bool, usize)>) {
    let folded = folds
        .iter()
        .fold(paper, |p, &(fold_up, fold_pos)| fold(&p, fold_up, fold_pos));

    for r in folded.rows() {
        println!(
            "{}",
            r.into_iter().map(|&v| if v { '#' } else { ' ' }).join("")
        );
    }
}

fn main() -> io::Result<()> {
    let input = aoc2021::read_input_lines()?;
    let (paper, folds) = parse_input(input);

    let p1 = part1(&paper, &folds);
    println!("Part 1: {}", p1);

    println!("Part 2:");
    part2(paper, &folds);

    Ok(())
}

fn parse_input(input: Vec<String>) -> (Array2<bool>, Vec<(bool, usize)>) {
    let points: Vec<(usize, usize)> = input
        .iter()
        .take_while(|l| matches!(l.chars().next(), Some(c) if c.is_digit(10)))
        .map(|l| {
            aoc2021::read_ints_from_string(l, false)
                .into_iter()
                .collect_tuple()
                .unwrap()
        })
        .collect();

    let max_x = points.iter().max_by_key(|(x, _)| x).unwrap().0;
    let max_y = points.iter().max_by_key(|(_, y)| y).unwrap().1;

    let mut arr: Array2<bool> = Array2::default((max_y + 1, max_x + 1));
    for (x, y) in points {
        arr[(y, x)] = true;
    }

    let folds = input
        .into_iter()
        .skip_while(|l| !l.starts_with('f'))
        .map(|l| {
            let (l, r) = l.split_once('=').unwrap();
            (l.ends_with('y'), r.parse().unwrap())
        })
        .collect();

    (arr, folds)
}
