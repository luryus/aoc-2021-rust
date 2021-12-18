use std::collections::VecDeque;
use std::io;

fn run(input: &[u16], rounds: usize) -> usize {
    let mut fish = VecDeque::from([0; 9]);
    for x in input {
        fish[*x as usize] += 1;
    }

    for _ in 0..rounds {
        let zeros = fish.pop_front().unwrap();
        // New eights
        fish.push_back(zeros);
        // The zeros reset to 6
        fish[6] += zeros;
    }

    fish.into_iter().sum()
}

fn main() -> io::Result<()> {
    let input = aoc2021::read_input_ints(false)?;

    let p1 = run(&input, 80);
    println!("Part 1: {}", p1);

    let p2 = run(&input, 256);
    println!("Part 2: {}", p2);

    Ok(())
}
