use std::io;
use itertools::Itertools;
use ndarray::Array2;


fn right(c: (usize, usize), w: usize) -> (usize, usize) {
    if c.1 == w - 1 {
        (c.0, 0)
    } else {
        (c.0, c.1 + 1)
    }
}

fn down(c: (usize, usize), h: usize) -> (usize, usize) {
    if c.0 == h - 1 {
        (0, c.1)
    } else {
        (c.0 + 1, c.1)
    }
}

fn part1(input: &Array2<char>) -> usize {
    let mut arr = input.clone();
    let w = arr.ncols();
    let h = arr.nrows();
    
    for rounds in 1.. {
        let to_move_right = arr.indexed_iter().filter(|&(_, c)| *c == '>')
            .filter(|&(i, _)| arr[right(i, w)] == '.')
            .map(|(i, _)| i)
            .collect::<Vec<_>>();
        for &c in &to_move_right {
            arr[c] = '.';
            arr[right(c, w)] = '>';
        }
        let to_move_down = arr.indexed_iter().filter(|&(_, c)| *c == 'v')
            .filter(|&(i, _)| arr[down(i, h)] == '.')
            .map(|(i, _)| i)
            .collect::<Vec<_>>();
        for &c in &to_move_down {
            arr[c] = '.';
            arr[down(c, h)] = 'v';
        }

        if to_move_down.is_empty() && to_move_right.is_empty() {
            return rounds;
        }
        //println!("Round {}", rounds);
        //dbg!(&arr);
    }

    unreachable!()
}


fn part2(input: &Array2<char>) -> usize {
    0
}

fn main() -> io::Result<()> {
    let input: Vec<Vec<_>> = aoc2021::read_input_lines()?.into_iter().map(|l| l.chars().collect()).collect();
    let input = Array2::from_shape_vec((input.len(), input[0].len()), input.into_iter().flatten().collect_vec()).unwrap();
    //dbg!(&input);

    let p1 = part1(&input);
    println!("Part 1: {}", p1);

    let p2 = part2(&input);
    println!("Part 2: {}", p2);

    Ok(())
}
