use itertools::Itertools;
use ndarray::{array, Array1, Array2};
use std::{
    collections::{HashSet, VecDeque},
    io,
};

fn transformation_matrices() -> Vec<Array2<i32>> {
    let rotations = [
        // None
        array![[1, 0, 0], [0, 1, 0], [0, 0, 1]],
        // Around x
        array![[1, 0, 0], [0, 0, -1], [0, 1, 0]],
        array![[1, 0, 0], [0, -1, 0], [0, 0, -1]],
        array![[1, 0, 0], [0, 0, 1], [0, -1, 0]],
        // Around y
        array![[0, 0, 1], [0, 1, 0], [-1, 0, 0]],
        array![[-1, 0, 0], [0, 1, 0], [0, 0, -1]],
        array![[0, 0, -1], [0, 1, 0], [1, 0, 0]],
        // Around z
        array![[0, -1, 0], [1, 0, 0], [0, 0, 1]],
        array![[-1, 0, 0], [0, -1, 0], [0, 0, 1]],
        array![[0, 1, 0], [-1, 0, 0], [0, 0, 1]],
    ];

    let rot_combs = rotations
        .iter()
        .tuple_combinations()
        .map(|(a, b)| a.dot(b))
        .unique()
        .collect_vec();

    rot_combs
}

#[test]
fn test_transformation_matrices() {
    let ts = transformation_matrices();
    assert_eq!(24, ts.len());
}

fn transformed_scanners(scanner: &[Array1<i32>]) -> impl Iterator<Item = Vec<Array1<i32>>> + '_ {
    transformation_matrices()
        .into_iter()
        .map(|t| scanner.iter().map(|c| c.dot(&t)).collect_vec())
}

#[test]
fn test_transformed_scanners() {
    let init = vec![
        array![-1, -1, 1],
        array![-2, -2, 2],
        array![-3, -3, 3],
        array![-2, -3, 1],
        array![5, 6, -4],
        array![8, 0, 7],
    ];

    let ts = transformed_scanners(&init).collect_vec();

    assert!(ts.contains(&vec![
        array![1, -1, 1],
        array![2, -2, 2],
        array![3, -3, 3],
        array![2, -1, 3],
        array![-5, 4, -6],
        array![-8, -7, 0],
    ]));

    assert!(ts.contains(&vec![
        array![-1, -1, -1],
        array![-2, -2, -2],
        array![-3, -3, -3],
        array![-1, -3, -2],
        array![4, 6, 5],
        array![-7, 0, 8],
    ]));

    assert!(ts.contains(&vec![
        array![1, 1, -1],
        array![2, 2, -2],
        array![3, 3, -3],
        array![1, 3, -2],
        array![-4, -6, 5],
        array![7, 0, 8],
    ]));
    assert!(ts.contains(&vec![
        array![1, 1, 1],
        array![2, 2, 2],
        array![3, 3, 3],
        array![3, 1, 2],
        array![-6, -4, -5],
        array![0, 7, -8],
    ]));
}

fn dist(a: &Array1<i32>, b: &Array1<i32>) -> i32 {
    (a[0] - b[0]).abs() + (a[1] - b[1]).abs() + (a[2] - b[2]).abs()
}

fn find_translation(
    refspace: &HashSet<Array1<i32>>,
    ref_x_limits: (i32, i32),
    ref_y_limits: (i32, i32),
    ref_z_limits: (i32, i32),
    ref_xs: &HashSet<i32>,
    ref_ys: &HashSet<i32>,
    source: &[Array1<i32>],
) -> Option<Array1<i32>> {
    for x in ref_x_limits.0 - 1001..=ref_x_limits.1 + 1001 {
        let num_matching_x = source
            .iter()
            .map(|c| c[0] + x)
            .filter(|xx| ref_xs.contains(xx))
            .count();
        if num_matching_x < 12 {
            continue;
        }

        println!("Found x {}", x);
        for y in ref_y_limits.0 - 1001..=ref_y_limits.1 + 1001 {
            let num_matching_y = source
                .iter()
                .map(|c| c[1] + y)
                .filter(|yy| ref_ys.contains(yy))
                .count();
            if num_matching_y < 12 {
                continue;
            }

            println!("Found y {}", y);

            for z in ref_z_limits.0 - 1001..=ref_z_limits.1 + 1001 {
                let num_matching_z = source
                    .iter()
                    .map(|c| c + array![x, y, z])
                    .filter(|mc| refspace.contains(mc))
                    .count();
                if num_matching_z >= 12 {
                    return Some(array![x, y, z]);
                }
            }
        }
    }

    None
}

fn part1(input: &[Vec<Array1<i32>>]) -> (usize, Vec<Array1<i32>>) {
    let mut refspace: HashSet<Array1<i32>> = input[0].iter().cloned().collect();
    let mut translations = vec![array![0, 0, 0]];

    let (mut ref_x_limits, mut ref_y_limits, mut ref_z_limits) = get_min_max_axis(&refspace);
    let mut ref_xs = refspace.iter().map(|c| c[0]).collect();
    let mut ref_ys = refspace.iter().map(|c| c[1]).collect();

    let mut scanner_queue: VecDeque<_> = input.iter().enumerate().skip(1).collect();

    'scanner_loop: while let Some((i, scanner)) = scanner_queue.pop_front() {
        for tscanner in transformed_scanners(scanner) {
            if let Some(t) = find_translation(
                &refspace,
                ref_x_limits,
                ref_y_limits,
                ref_z_limits,
                &ref_xs,
                &ref_ys,
                &tscanner,
            ) {
                for c in tscanner {
                    if refspace.insert(&c + &t) {
                        ref_xs.insert(c[0] + t[0]);
                        ref_ys.insert(c[1] + t[1]);
                    }
                }
                let new_limits = get_min_max_axis(&refspace);
                ref_x_limits = new_limits.0;
                ref_y_limits = new_limits.1;
                ref_z_limits = new_limits.2;
                translations.push(t);
                println!("Scanner {} mapped!", i);
                continue 'scanner_loop;
            }
        }

        // Not found, put back in queue
        scanner_queue.push_back((i, scanner));
    }

    (refspace.len(), translations)
}

fn get_min_max_axis(refspace: &HashSet<Array1<i32>>) -> ((i32, i32), (i32, i32), (i32, i32)) {
    let r = refspace.iter().fold(
        (0, 0, 0, 0, 0, 0),
        |(minx, maxx, miny, maxy, minz, maxz), c| {
            (
                minx.min(c[0]),
                maxx.max(c[0]),
                miny.min(c[1]),
                maxy.max(c[1]),
                minz.min(c[2]),
                maxz.max(c[2]),
            )
        },
    );
    ((r.0, r.1), (r.2, r.3), (r.4, r.5))
}

fn part2(points: &[Array1<i32>]) -> i32 {
    points
        .iter()
        .cartesian_product(points)
        .map(|(a, b)| dist(a, b))
        .max_by_key(|x| *x)
        .unwrap()
}

fn read_scanners(lines: Vec<String>) -> Vec<Vec<Array1<i32>>> {
    let mut scanners = vec![];

    let mut curr_scanner = vec![];

    for l in lines.iter().skip(1) {
        if l.starts_with("---") {
            scanners.push(curr_scanner);
            curr_scanner = vec![];
            continue;
        }

        let c = aoc2021::read_ints_from_string(l, true)
            .into_iter()
            .collect::<Array1<_>>();
        curr_scanner.push(c);
    }

    scanners.push(curr_scanner);
    scanners
}

fn main() -> io::Result<()> {
    let input = aoc2021::read_input_lines()?;
    let input = read_scanners(input);

    let (points, translations) = part1(&input);
    println!("---\nPart 1: {}", points);

    let p2 = part2(&translations);
    println!("Part 2: {}", p2);

    Ok(())
}
