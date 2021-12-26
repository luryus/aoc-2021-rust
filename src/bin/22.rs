use itertools::Itertools;
use std::io;

fn part1(input: &[(bool, Vec<i32>)]) -> usize {
    let input: Vec<_> = input
        .iter()
        .filter(|(_, cs)| cs.iter().all(|c| c.abs() <= 50))
        .cloned()
        .collect();
    run(&input)
}

#[derive(Clone, Eq, PartialEq, Debug)]
struct Cube(i32, i32, i32, i32, i32, i32);
impl Cube {
    fn volume(&self) -> usize {
        (self.1 - self.0 + 1) as usize
            * (self.3 - self.2 + 1) as usize
            * (self.5 - self.4 + 1) as usize
    }

    fn overlap(&self, other: &Cube) -> bool {
        self.1 >= other.0
            && other.1 >= self.0
            && self.3 >= other.2
            && other.3 >= self.2
            && self.5 >= other.4
            && other.5 >= self.4
    }

    fn intersection(&self, other: &Cube) -> Option<Cube> {
        if !self.overlap(other) {
            None
        } else {
            let x0 = self.0.max(other.0);
            let x1 = self.1.min(other.1);
            let y0 = self.2.max(other.2);
            let y1 = self.3.min(other.3);
            let z0 = self.4.max(other.4);
            let z1 = self.5.min(other.5);
            Some(Cube(x0, x1, y0, y1, z0, z1))
        }
    }

    fn with_x(&self, x0: i32, x1: i32) -> Self {
        Cube(x0, x1, self.2, self.3, self.4, self.5)
    }

    fn with_y(&self, y0: i32, y1: i32) -> Self {
        Cube(self.0, self.1, y0, y1, self.4, self.5)
    }

    fn with_z(&self, z0: i32, z1: i32) -> Self {
        Cube(self.0, self.1, self.2, self.3, z0, z1)
    }
}

#[test]
fn test_cube_volume() {
    assert_eq!(27, Cube(0, 2, 0, 2, 0, 2).volume());
    assert_eq!(27, Cube(-3, -1, -3, -1, -3, -1).volume());
}

fn get_split_coord(a0: i32, a1: i32, b0: i32, b1: i32) -> (i32, i32) {
    if !(a1 >= b0 && b1 >= a0) {
        panic!("Coordinates do not overlap")
    } else {
        (a0.max(b0), a1.min(b1))
    }
}

#[test]
fn test_get_split_coords() {
    assert_eq!((0, 2), get_split_coord(0, 2, 0, 2));
    assert_eq!((1, 1), get_split_coord(0, 2, 1, 1));
    assert_eq!((3, 6), get_split_coord(3, 6, 3, 6));
    assert_eq!((0, 2), get_split_coord(-1, 3, 0, 2));
    assert_eq!((0, 2), get_split_coord(-1, 3, 0, 2));
    assert_eq!((1, 1), get_split_coord(0, 1, 1, 2));
}

fn split_to_parts(cube: &Cube, splitter: &Cube) -> Vec<Cube> {
    let splitter = match cube.intersection(splitter) {
        Some(s) => s,
        None => return vec![cube.clone()],
    };

    // X axis
    let (x0, x1) = get_split_coord(cube.0, cube.1, splitter.0, splitter.1);
    let x_cubes = [
        cube.with_x(cube.0, x0 - 1),
        cube.with_x(x0, x1),
        cube.with_x(x1 + 1, cube.1),
    ]
    .into_iter()
    .filter(|c| c.0 <= c.1);

    // Y axis
    let y_cubes = x_cubes.flat_map(|c| {
        if !c.overlap(&splitter) {
            vec![c]
        } else {
            let (y0, y1) = get_split_coord(c.2, c.3, splitter.2, splitter.3);
            [
                c.with_y(c.2, y0 - 1),
                c.with_y(y0, y1),
                c.with_y(y1 + 1, c.3),
            ]
            .into_iter()
            .filter(|c| c.2 <= c.3)
            .collect()
        }
    });

    // Z axis
    let z_cubes = y_cubes.flat_map(|c| {
        if !c.overlap(&splitter) {
            vec![c]
        } else {
            let (z0, z1) = get_split_coord(c.4, c.5, splitter.4, splitter.5);
            [
                c.with_z(c.4, z0 - 1),
                c.with_z(z0, z1),
                c.with_z(z1 + 1, c.5),
            ]
            .into_iter()
            .filter(|c| c.4 <= c.5)
            .collect()
        }
    });

    z_cubes
        .into_iter()
        .filter(|c| !c.overlap(&splitter))
        .collect()
}

#[test]
fn test_split_to_parts() {
    // Just x
    assert_eq!(
        vec![Cube(0, 0, 0, 2, 0, 2), Cube(2, 2, 0, 2, 0, 2)],
        split_to_parts(&Cube(0, 2, 0, 2, 0, 2), &Cube(1, 1, -100, 100, -100, 100))
    );

    // Just y
    assert_eq!(
        vec![Cube(0, 2, 0, 0, 0, 2), Cube(0, 2, 2, 2, 0, 2)],
        split_to_parts(&Cube(0, 2, 0, 2, 0, 2), &Cube(-100, 100, 1, 1, -100, 100))
    );

    // Just z
    assert_eq!(
        vec![Cube(0, 2, 0, 2, 0, 0), Cube(0, 2, 0, 2, 2, 2)],
        split_to_parts(&Cube(0, 2, 0, 2, 0, 2), &Cube(-100, 100, -100, 100, 1, 1))
    );

    // Corner
    let parts = split_to_parts(&Cube(0, 2, 0, 2, 0, 2), &Cube(2, 2, 2, 2, 2, 2));
    assert_eq!(parts.iter().map(|p| p.volume()).sum::<usize>(), 26);
}

fn run(input: &[(bool, Vec<i32>)]) -> usize {
    let mut cubes: Vec<Cube> = Vec::new();

    for (v, cs) in input {
        let cube: (i32, i32, i32, i32, i32, i32) = cs.iter().copied().collect_tuple().unwrap();
        let cube = Cube(cube.0, cube.1, cube.2, cube.3, cube.4, cube.5);

        cubes = cubes
            .into_iter()
            .flat_map(|prev| split_to_parts(&prev, &cube))
            .collect();

        if *v {
            cubes.push(cube);
        }
    }

    cubes.iter().map(|c| c.volume()).sum()
}

fn part2(input: &[(bool, Vec<i32>)]) -> usize {
    run(input)
}

fn read_input() -> io::Result<Vec<(bool, Vec<i32>)>> {
    Ok(aoc2021::read_input_lines()?
        .into_iter()
        .map(|l| {
            (
                l.starts_with("on"),
                aoc2021::read_ints_from_string(&l, true),
            )
        })
        .collect())
}

fn main() -> io::Result<()> {
    let input = read_input()?;

    let p1 = part1(&input);
    println!("Part 1: {}", p1);

    let p2 = part2(&input);
    println!("Part 2: {}", p2);

    Ok(())
}
