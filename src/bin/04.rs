use std::io;
use itertools::Itertools;
use ndarray::Array2;

fn part1(nums: &Vec<u32>, boards: &Vec<BingoBoard>) -> usize {
    let mut boards = boards.clone();

    for n in nums {
        for b in boards.iter_mut() {
            b.mark_num(*n);
            if b.check() {
                return (b.unmarked_sum() * n) as usize
            }
        }
    }

    unreachable!("No board win")
}


fn part2(nums: &Vec<u32>, boards: &Vec<BingoBoard>) -> usize {
    let mut boards = boards.clone();
    let mut last_score = 0u32;

    for n in nums {
        let mut bingo_indices = vec![];
        for (i, b) in boards.iter_mut().enumerate() {
            b.mark_num(*n);
            
            if b.check() {
                bingo_indices.push(i);
            }
        }

        if !bingo_indices.is_empty() {
            last_score = boards[*bingo_indices.last().unwrap()].unmarked_sum() * n;
            for i in bingo_indices.into_iter().rev() {
                boards.remove(i);
            }
        }
    }

    last_score as usize
}

#[derive(Clone)]
struct BingoBoard {
    rows: Array2<u32>,
    marks: Array2<bool>,
}

impl BingoBoard {
    fn mark_num(&mut self, n: u32) {
        let pos = self.rows.indexed_iter().filter(|(_, v)| n == **v).next();
        if let Some((p, _)) = pos {
            self.marks[p] = true;
        }
    }

    fn check(&self) -> bool {
        let row_res = self.marks.rows().into_iter().any(|r| r.iter().all(|x| *x));
        if row_res {
            return true;
        }
        let col_res = self.marks.columns().into_iter().any(|c| c.iter().all(|x| *x));
        return col_res;
    }

    fn unmarked_sum(&self) -> u32 {
        self.rows.indexed_iter()
            .filter(|(i, _)| !self.marks[*i])
            .map(|(_, v)| *v)
            .sum()
    }
}

fn main() -> io::Result<()> {
    let input = aoc2021::read_input_string()?;
    let mut input = input.split("\n\n");
    let bingo_nums = aoc2021::read_ints_from_string(input.next().unwrap());
    let boards: Vec<BingoBoard> = parse_bingo_boards(input);

    let p1 = part1(&bingo_nums, &boards);
    println!("Part 1: {}", p1);

    let p2 = part2(&bingo_nums, &boards);
    println!("Part 2: {}", p2);

    Ok(())
}

fn parse_bingo_boards(input: std::str::Split<&str>) -> Vec<BingoBoard> {
    input
        .map(|b| {
            b.lines().map(|l| aoc2021::read_ints_from_string::<u32>(l)).collect_vec()
        })
        .map(|b_rows| {
            let rc = b_rows.len();
            let b = Array2::from_shape_vec((rc, b_rows[0].len()), b_rows.into_iter().flatten().collect()).unwrap();
            BingoBoard {
                marks: Array2::default(b.raw_dim()),
                rows: b,
            }
        })
        .collect()
}
