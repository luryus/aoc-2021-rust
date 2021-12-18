use std::io;
use itertools::Itertools;

fn part1(input: &[String]) -> usize {
    input.iter().map(|l| l.split_once(" | ").unwrap().1)
        .map(|l| l.split(' '))
        .flatten()
        .filter(|x| [7,2,3,4].contains(&x.len()))
        .count()
}

struct SevenSegment {
    top_right: char,
    bottom_left: char,
    bottom_right: char
}

impl SevenSegment {
    fn get_num(&self, cs: &str) -> u32 {
        if cs.len() == 2 {
            1
        } else if cs.len() == 4 {
            4
        } else if cs.len() == 3 {
            7
        } else if cs.len() == 7 {
            8
        } else if cs.len() == 6 {
            if !cs.contains(self.bottom_left) {
                9
            } else if !cs.contains(self.top_right) {
                6
            } else {
                0
            }
        } else if !cs.contains(self.top_right) {
            5
        } else if cs.contains(self.bottom_right) {
            3
        } else {
            2
        }
    }
}

fn deduce(line: Vec<&str>) -> Option<SevenSegment> {
    let one_segs = line.iter().find(|x| x.len() == 2)?.chars();
    let (top_right, bottom_right) = one_segs.sorted_by_key(|c| line.iter().filter(|x| x.chars().contains(c)).count())
        .collect_tuple()?;
    let top = line.iter().find(|x| x.len() == 3)?
        .chars().find(|&c| c != top_right && c != bottom_right)?;
    let four_left_segs = line.iter().find(|x| x.len() == 4)?
        .chars().filter(|&c| c != top_right && c != bottom_right);
    let (top_left, middle) = four_left_segs.sorted_by_key(|c| line.iter().filter(|x| x.chars().contains(c)).count())
        .collect_tuple()?;
    let bot_left_bot = line.iter().filter(|x| x.len() == 5).map(|x| x.chars()).flatten()
        .filter(|c| ![top_right, bottom_right, top, top_left, middle].contains(c))
        .sorted().collect_vec();
    let bot = bot_left_bot[1];
    let bot_left = bot_left_bot.into_iter().find(|&x| x != bot)?;

    Some(SevenSegment {
        top_right,
        bottom_left: bot_left,
        bottom_right,
    })
}



fn part2(input: &[String]) -> u32 {
    input.iter().map(|l| l.split_once(" | ").unwrap())
        .map(|(a, b)| (deduce(a.split(' ').collect()).unwrap(), b.split(' ').collect_vec()))
        .map(|(disp, nums)| {
            disp.get_num(nums[0]) * 1000 +
            disp.get_num(nums[1]) * 100 +
            disp.get_num(nums[2]) * 10 +
            disp.get_num(nums[3])
        })
        .sum()
}

fn main() -> io::Result<()> {
    let input = aoc2021::read_input_lines()?;

    let p1 = part1(&input);
    println!("Part 1: {}", p1);

    let p2 = part2(&input);
    println!("Part 2: {}", p2);

    Ok(())
}
