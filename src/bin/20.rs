
use ndarray::{s, Array2};
use std::io;

fn run(algo: &Vec<bool>, img: &Array2<bool>, outside: bool) -> (Array2<bool>, bool) {
    let new_outside = if outside {
        *algo.last().unwrap()
    } else {
        algo[0]
    };

    let mut res = Array2::default((img.nrows() + 2, img.ncols() + 2));
    let img = pad(img, outside);

    for y in 0..=img.nrows() - 3 {
        for x in 0..=img.ncols() - 3 {
            let win = img.slice(s![y..y + 3, x..x + 3]);
            let index = win.iter().fold(0usize, |acc, i| acc << 1 | *i as usize);
            res[(y, x)] = algo[index];
        }
    }

    (res, new_outside)
}

fn part1(algo: &Vec<bool>, img: &Array2<bool>) -> usize {
    // First
    let (img, outside) = run(algo, img, false);

    // Second
    let (img, outside) = run(algo, &img, outside);

    assert_eq!(false, outside);

    img.into_iter().filter(|x| *x).count()
}

fn part2(algo: &Vec<bool>, img: Array2<bool>) -> usize {
    let img = (0..50)
        .fold((img, false), |(img, outside), _| run(algo, &img, outside))
        .0;
    img.into_iter().filter(|x| *x).count()
}

fn pad<T: Default + Clone + Copy>(arr: &Array2<T>, val: T) -> Array2<T> {
    let mut padded = Array2::default((arr.nrows() + 4, arr.ncols() + 4));
    padded.fill(val);
    let mut center_view = padded.slice_mut(s![2..-2, 2..-2]);
    center_view.assign(arr);
    padded
}

#[test]
fn test_pad() {
    let a = array![[1, 2]];

    let p = pad(&a, 0);

    assert_eq!(
        array![
            [0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0],
            [0, 0, 1, 2, 0, 0],
            [0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0]
        ],
        p
    );
}

fn parse_input(input: Vec<String>) -> (Vec<bool>, Array2<bool>) {
    let algo = input[0].chars().map(|c| c == '#').collect();

    let img_w = input[1].len();
    let img_h = input.len() - 1;
    let img: Vec<_> = input
        .iter()
        .skip(1)
        .flat_map(|s| s.chars())
        .map(|c| c == '#')
        .collect();

    (algo, Array2::from_shape_vec((img_w, img_h), img).unwrap())
}

fn main() -> io::Result<()> {
    let input = aoc2021::read_input_lines()?;
    let (algo, img) = parse_input(input);

    let p1 = part1(&algo, &img);
    println!("Part 1: {}", p1);

    let p2 = part2(&algo, img);
    println!("Part 2: {}", p2);

    Ok(())
}
