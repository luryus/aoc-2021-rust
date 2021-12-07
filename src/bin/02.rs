use std::io;

fn part1(input: &[String]) -> i32 {
    let (hor, dep) = input
        .iter()
        .map(|x| aoc2021::split_to_tuple2(x, " ").unwrap())
        .map(|(a, b)| {
            let i = b.parse().unwrap();
            match a {
                "forward" => (i, 0),
                "down" => (0, i),
                "up" => (0, -i),
                _ => panic!("Invalid instruction"),
            }
        })
        .reduce(|(a, b), (acc_a, acc_b)| (a + acc_a, b + acc_b))
        .unwrap();

    hor * dep
}

fn part2(input: &[String]) -> i32 {
    let (hor, _, dep) = input
        .iter()
        .map(|x| aoc2021::split_to_tuple2(x, " ").unwrap())
        .map(|(a, b)| {
            let i = b.parse().unwrap();
            match a {
                "forward" => (i, 0),
                "down" => (0, i),
                "up" => (0, -i),
                _ => panic!("Invalid instruction"),
            }
        })
        .fold((0, 0, 0), |(acc_hor, acc_aim, acc_dep), (hor, aim)| {
            if hor > 0 {
                (acc_hor + hor, acc_aim, acc_aim * hor + acc_dep)
            } else {
                (acc_hor, aim + acc_aim, acc_dep)
            }
        });

    hor * dep
}

fn main() -> io::Result<()> {
    let input = aoc2021::read_input_lines()?;

    let p1 = part1(&input);
    println!("Part 1: {}", p1);

    let p2 = part2(&input);
    println!("Part 2: {}", p2);

    Ok(())
}
