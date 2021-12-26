use itertools::Itertools;
use std::io;

struct Cave {
    large: bool,
    edges: Vec<usize>,
}

#[derive(Clone)]
struct Path {
    path: Vec<usize>,
    double_visit_used: bool,
}

fn parse_input<'a>(input: Vec<String>) -> (Vec<Cave>, usize, usize) {
    let all_names: Vec<_> = input
        .iter()
        .map(|l| aoc2021::read_regex_matches_from_string(l, r"\w+"))
        .flatten()
        .unique()
        .collect();
    let mut res: Vec<Cave> = all_names
        .iter()
        .map(|n| Cave {
            edges: vec![],
            large: n.chars().next().unwrap().is_uppercase(),
        })
        .collect();

    for l in &input {
        let (start, end) = aoc2021::read_regex_matches_from_string(l, r"\w+")
            .into_iter()
            .collect_tuple()
            .unwrap();

        let start_idx = all_names.iter().position(|n| n == &start).unwrap();
        let end_idx = all_names.iter().position(|n| n == &end).unwrap();

        res[start_idx].edges.push(end_idx);
        res[end_idx].edges.push(start_idx);
    }

    (
        res,
        all_names.iter().position(|n| *n == "start").unwrap(),
        all_names.iter().position(|n| *n == "end").unwrap(),
    )
}

fn f(caves: &[Cave], path: Vec<&Cave>, end: usize) -> usize {
    let head = *path.last().unwrap();
    if std::ptr::eq(head, &caves[end]) {
        return 1;
    }

    let mut paths = 0;
    for e in &head.edges {
        let ec = &caves[*e];
        if ec.large || !path.iter().any(|pc| std::ptr::eq(ec, *pc)) {
            let mut p = path.clone();
            p.push(ec);
            paths += f(caves, p, end);
        }
    }

    paths
}

fn part1(input: &Vec<Cave>, start: usize, end: usize) -> usize {
    f(input, vec![&input[start]], end)
}

fn can_visit(path: &Path, idx: usize, cave: &Cave, start_pos: usize) -> (bool, bool) {
    if cave.large {
        return (true, path.double_visit_used);
    }

    let prev_exist = path.path.contains(&idx);

    match (prev_exist, path.double_visit_used) {
        (false, _) => (true, path.double_visit_used),
        (true, true) => (false, true),
        (true, false) if idx == start_pos => (false, false),
        (true, false) => (true, true),
    }
}

fn f2(caves: &[Cave], path: Path, start: usize, end: usize) -> usize {
    let head = *path.path.last().unwrap();
    if head == end {
        return 1;
    }
    let head_cave = &caves[head];

    let mut paths = 0;
    for e in &head_cave.edges {
        let ec = &caves[*e];
        let (can_visit, second_visit_used) = can_visit(&path, *e, ec, start);
        if can_visit {
            let mut p = path.clone();
            p.double_visit_used = second_visit_used;
            p.path.push(*e);
            paths += f2(caves, p, start, end);
        }
    }

    paths
}

fn part2(input: &Vec<Cave>, start: usize, end: usize) -> usize {
    f2(
        input,
        Path {
            path: vec![start],
            double_visit_used: false,
        },
        start,
        end,
    )
}

fn main() -> io::Result<()> {
    let input = aoc2021::read_input_lines()?;
    let (input, start_cave, end_cave) = parse_input(input);

    let p1 = part1(&input, start_cave, end_cave);
    println!("Part 1: {}", p1);

    let p2 = part2(&input, start_cave, end_cave);
    println!("Part 2: {}", p2);

    Ok(())
}
