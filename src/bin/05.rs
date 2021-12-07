use std::{collections::HashMap, io};

fn run(input: &[Line], part2: bool) -> usize {
    let input = input
        .iter()
        .filter(|x| part2 || x.start_x == x.end_x || x.start_y == x.end_y);

    let mut cs = HashMap::new();

    for l in input {
        let (dx, dy) = (l.dx().signum(), l.dy().signum());

        let mut x = l.start_x;
        let mut y = l.start_y;
        while x != l.end_x || y != l.end_y {
            let e = cs.entry((x, y)).or_default();
            *e += 1;
            x += dx;
            y += dy;
        }
        let e = cs.entry((x, y)).or_default();
        *e += 1;
    }

    cs.values().filter(|x: &&usize| **x > 1).count()
}

#[derive(Debug)]
struct Line {
    start_x: i16,
    start_y: i16,
    end_x: i16,
    end_y: i16,
}

impl Line {
    fn dy(&self) -> i16 {
        self.end_y - self.start_y
    }

    fn dx(&self) -> i16 {
        self.end_x - self.start_x
    }
}

fn parse_lines(lines: &[String]) -> Vec<Line> {
    lines
        .iter()
        .map(|l| aoc2021::read_ints_from_string(l))
        .map(|l: Vec<i16>| Line {
            start_x: l[0],
            start_y: l[1],
            end_x: l[2],
            end_y: l[3],
        })
        .collect()
}

fn main() -> io::Result<()> {
    let input = aoc2021::read_input_lines()?;
    let input = parse_lines(&input);

    let p1 = run(&input, false);
    println!("Part 1: {}", p1);

    let p2 = run(&input, true);
    println!("Part 2: {}", p2);

    Ok(())
}
