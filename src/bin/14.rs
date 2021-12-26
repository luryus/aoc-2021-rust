use itertools::Itertools;
use std::{collections::BTreeMap, io};

fn run(init: &str, nodes: &BTreeMap<(char, char), Node>, rounds: usize) -> usize {
    let mut counts: BTreeMap<char, usize> = BTreeMap::new();
    let mut cache = BTreeMap::new();
    for (a, b) in init.chars().tuple_windows() {
        let n = nodes[&(a, b)];
        for (c, v) in n.get_node_char_counts(nodes, rounds, &mut cache) {
            *counts.entry(c).or_default() += v;
        }
    }

    for c in init.chars().skip(1).take(init.len() - 2) {
        counts.entry(c).and_modify(|v| *v -= 1);
    }

    let (min, max) = counts.values().minmax().into_option().unwrap();
    max - min
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Node {
    left: char,
    right: char,
    mid: char,
}

impl Node {
    fn get_next_nodes(&self, nodes: &BTreeMap<(char, char), Node>) -> (Node, Node) {
        (
            nodes[&(self.left, self.mid)],
            nodes[&(self.mid, self.right)],
        )
    }

    fn get_node_char_counts(
        &self,
        nodes: &BTreeMap<(char, char), Node>,
        level: usize,
        cache: &mut BTreeMap<(usize, Node), BTreeMap<char, usize>>,
    ) -> BTreeMap<char, usize> {
        if level == 1 {
            let mut res = BTreeMap::new();
            res.insert(self.left, 1);
            *res.entry(self.right).or_default() += 1;
            *res.entry(self.mid).or_default() += 1;
            return res;
        }

        if let Some(res) = cache.get(&(level, *self)) {
            return res.clone();
        }

        let (l, r) = self.get_next_nodes(nodes);
        let mut l_counts = l.get_node_char_counts(nodes, level - 1, cache);
        let r_counts = r.get_node_char_counts(nodes, level - 1, cache);
        l_counts.entry(self.mid).and_modify(|v| *v -= 1);
        for (k, v) in r_counts {
            *l_counts.entry(k).or_default() += v;
        }

        cache.insert((level, *self), l_counts.clone());

        l_counts
    }
}

fn parse_input(mut input: Vec<String>) -> (String, BTreeMap<(char, char), Node>) {
    let rules = input
        .iter()
        .skip(1)
        .map(|l| {
            let (a, b) = l.split_once(" -> ").unwrap();
            Node {
                left: a.chars().next().unwrap(),
                right: a.chars().nth(1).unwrap(),
                mid: b.chars().next().unwrap(),
            }
        })
        .collect_vec();

    let rule_dict = rules.into_iter().map(|n| ((n.left, n.right), n)).collect();

    (input.swap_remove(0), rule_dict)
}

fn main() -> io::Result<()> {
    let input = aoc2021::read_input_lines()?;
    let (init, nodes) = parse_input(input);

    let p1 = run(&init, &nodes, 10);
    println!("Part 1: {}", p1);

    let p2 = run(&init, &nodes, 40);
    println!("Part 2: {}", p2);

    Ok(())
}
