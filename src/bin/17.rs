use itertools::Itertools;
use std::io;

const fn arithmetic_sum(n: i32) -> i32 {
    n * (n + 1) / 2
}

fn part1(x0: i32, x1: i32, y0: i32, y1: i32) -> i32 {
    assert!(y0 < 0);
    assert!(y1 < 0);

    let max_y = -y0 - 1;
    let min_y = -y1 - -1;

    for y_vel in (min_y..=max_y).rev() {
        let rounds = max_y * 2 + 1;
        // Try to find a suitable x
        for x_vel in (0..rounds * 2).rev() {
            if x_vel > rounds {
                let s1 = arithmetic_sum(x_vel);
                let s2 = arithmetic_sum(x_vel - rounds);
                if (s1 - s2) <= x1 && (s1 - s2) >= x0 {
                    return arithmetic_sum(y_vel);
                }
            } else {
                let x = arithmetic_sum(x_vel);
                if x <= x1 && x >= x0 {
                    return arithmetic_sum(y_vel);
                }
            }
        }
    }

    unreachable!()
}

fn part2(x0: i32, x1: i32, y0: i32, y1: i32) -> i32 {
    assert!(y0 < 0);
    assert!(y1 < 0);

    // The largest possible y vel is one which just barely
    // does not overshoot the target area when coming down
    let max_y_vel = -y0 - 1;

    // The smallest possible y_vel is one which goes directly
    // down to the lowest row of the target area
    let min_y_vel = y0;

    // The smallest possible x_vel is one which just barely reaches
    // the first column of target area before stalling
    let min_x_vel = (1..).find(|x_vel| arithmetic_sum(*x_vel) >= x0).unwrap();

    // The largest possible x velocity directly goes to the last
    // column of the target area
    let max_x_vel = x1;

    // Brute force all combinations
    let mut valid = 0;
    for (mut x_vel, mut y_vel) in (min_x_vel..=max_x_vel).cartesian_product(min_y_vel..=max_y_vel) {
        let mut x = 0;
        let mut y = 0;
        loop {
            x += x_vel;
            y += y_vel;

            if x > x1 || y < y0 {
                break;
            }
            if x >= x0 && y <= y1 {
                valid += 1;
                break;
            }

            if x_vel > 0 {
                x_vel -= 1;
            }
            y_vel -= 1;
        }
    }

    valid
}

fn main() -> io::Result<()> {
    let input = aoc2021::read_input_ints::<i32>(true)?;
    let (x0, x1, y0, y1) = input.into_iter().collect_tuple().unwrap();

    let p1 = part1(x0, x1, y0, y1);
    println!("Part 1: {}", p1);

    let p2 = part2(x0, x1, y0, y1);
    println!("Part 2: {}", p2);

    Ok(())
}
