/// This is a stupidly overcomplicated solution.
/// Just wanted to try implementing trees and stuff using Rc<RefCell<T>>.
/// Will not try again...
use itertools::Itertools;
use std::{cell::RefCell, fmt::Display, io, rc::Rc};

struct PairNode {
    left: SnailfishNumber,
    right: SnailfishNumber,
    parent: Option<Rc<RefCell<PairNode>>>,
}

impl PairNode {
    fn new(parent: Option<Rc<RefCell<PairNode>>>) -> Self {
        PairNode {
            left: SnailfishNumber::Regular(0),
            right: SnailfishNumber::Regular(0),
            parent,
        }
    }

    fn zero_child(&mut self, child: Rc<RefCell<PairNode>>) {
        if let SnailfishNumber::Pair(p) = &self.left {
            if Rc::ptr_eq(&child, p) {
                self.left = SnailfishNumber::Regular(0);
                return;
            }
        }

        if let SnailfishNumber::Pair(p) = &self.right {
            if Rc::ptr_eq(&child, p) {
                self.right = SnailfishNumber::Regular(0);
                return;
            }
        }

        panic!("zero_child called with a non-child argument")
    }
}

impl Display for PairNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{},{}]", self.left, self.right)
    }
}

fn add_to_left(target: Rc<RefCell<PairNode>>, val: u32, coming_from: Rc<RefCell<PairNode>>) {
    let coming_from_right = match &target.borrow().right {
        SnailfishNumber::Pair(rp) if Rc::ptr_eq(rp, &coming_from) => true,
        _ => false,
    };
    let coming_from_parent = match &target.borrow().parent {
        Some(pp) if Rc::ptr_eq(pp, &coming_from) => true,
        _ => false,
    };
    if coming_from_right {
        let mut node = target.borrow_mut();
        match &node.left {
            SnailfishNumber::Regular(prev) => {
                node.left = SnailfishNumber::Regular(prev + val);
            }
            SnailfishNumber::Pair(p) => {
                add_to_right(p.clone(), val, target.clone());
            }
        }
    } else if coming_from_parent {
        let mut node = target.borrow_mut();
        match &node.left {
            SnailfishNumber::Regular(prev) => {
                node.left = SnailfishNumber::Regular(prev + val);
            }
            SnailfishNumber::Pair(p) => {
                add_to_left(p.clone(), val, target.clone());
            }
        }
    } else if let Some(parent) = &target.borrow().parent {
        add_to_left(parent.clone(), val, target.clone());
    }
}

fn add_to_right(target: Rc<RefCell<PairNode>>, val: u32, coming_from: Rc<RefCell<PairNode>>) {
    let coming_from_left = match &target.borrow().left {
        SnailfishNumber::Pair(lp) if Rc::ptr_eq(lp, &coming_from) => true,
        _ => false,
    };
    let coming_from_parent = match &target.borrow().parent {
        Some(pp) if Rc::ptr_eq(pp, &coming_from) => true,
        _ => false,
    };
    if coming_from_left {
        let mut node = target.borrow_mut();
        match &node.right {
            SnailfishNumber::Regular(prev) => {
                node.right = SnailfishNumber::Regular(prev + val);
            }
            SnailfishNumber::Pair(p) => {
                add_to_left(p.clone(), val, target.clone());
            }
        }
    } else if coming_from_parent {
        let mut node = target.borrow_mut();
        match &node.right {
            SnailfishNumber::Regular(prev) => {
                node.right = SnailfishNumber::Regular(prev + val);
            }
            SnailfishNumber::Pair(p) => {
                add_to_right(p.clone(), val, target.clone());
            }
        }
    } else if let Some(parent) = &target.borrow().parent {
        add_to_right(parent.clone(), val, target.clone());
    }
}

fn explode(node_ptr: Rc<RefCell<PairNode>>, level: usize) -> bool {
    if level >= 4 {
        let l = node_ptr.borrow().left.clone();
        let r = node_ptr.borrow().right.clone();
        if let SnailfishNumber::Regular(l) = l {
            if let SnailfishNumber::Regular(r) = r {
                let parent_ptr = node_ptr.borrow().parent.clone().unwrap();
                add_to_left(parent_ptr.clone(), l, node_ptr.clone());
                add_to_right(parent_ptr.clone(), r, node_ptr.clone());
                parent_ptr.borrow_mut().zero_child(node_ptr);
                return true;
            }
        }
    }

    let lp = node_ptr.borrow().left.clone();
    if let SnailfishNumber::Pair(lp) = lp {
        if explode(lp, level + 1) {
            return true;
        }
    }

    let rp = node_ptr.borrow().right.clone();
    if let SnailfishNumber::Pair(rp) = rp {
        if explode(rp, level + 1) {
            return true;
        }
    }

    false
}

#[derive(Clone)]
enum SnailfishNumber {
    Regular(u32),
    Pair(Rc<RefCell<PairNode>>),
}

impl Display for SnailfishNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Regular(x) => write!(f, "{}", x),
            Self::Pair(p) => p.borrow().fmt(f),
        }
    }
}

impl SnailfishNumber {
    fn clone_tree(&self, parent: Option<Rc<RefCell<PairNode>>>) -> SnailfishNumber {
        match self {
            Self::Regular(x) => Self::Regular(*x),
            Self::Pair(pp) => {
                let new = Rc::new(RefCell::new(PairNode::new(parent)));
                let pl = pp.borrow().left.clone_tree(Some(new.clone()));
                let pr = pp.borrow().right.clone_tree(Some(new.clone()));
                new.borrow_mut().left = pl;
                new.borrow_mut().right = pr;
                Self::Pair(new)
            }
        }
    }

    fn reduce(&mut self) {
        while self.explode() || self.split() {}
    }

    fn explode(&mut self) -> bool {
        if let SnailfishNumber::Pair(pair) = self {
            return explode(pair.clone(), 0);
        }
        panic!("Top-level regular!")
    }

    fn split(&mut self) -> bool {
        if let SnailfishNumber::Pair(pn) = self {
            let mut pn_mut = pn.borrow_mut();
            match pn_mut.left {
                SnailfishNumber::Regular(x) if x >= 10 => {
                    let mut new_pair = PairNode::new(Some(pn.clone()));
                    new_pair.left = SnailfishNumber::Regular(x / 2);
                    new_pair.right = SnailfishNumber::Regular((x + 1) / 2);
                    pn_mut.left = SnailfishNumber::Pair(Rc::new(RefCell::new(new_pair)));
                    return true;
                }
                SnailfishNumber::Pair(_) => {
                    if pn_mut.left.split() {
                        return true;
                    }
                }
                _ => {}
            }
            match pn_mut.right {
                SnailfishNumber::Regular(x) if x >= 10 => {
                    let mut new_pair = PairNode::new(Some(pn.clone()));
                    new_pair.left = SnailfishNumber::Regular(x / 2);
                    new_pair.right = SnailfishNumber::Regular((x + 1) / 2);
                    pn_mut.right = SnailfishNumber::Pair(Rc::new(RefCell::new(new_pair)));
                    return true;
                }
                SnailfishNumber::Pair(_) => {
                    if pn_mut.right.split() {
                        return true;
                    }
                }
                _ => {}
            }
            return false;
        }

        panic!("Top-level regular!")
    }

    fn add(mut self, mut other: Self) -> Self {
        let new_pair = PairNode::new(None);
        let new_pair_rc = Rc::new(RefCell::new(new_pair));

        if let SnailfishNumber::Pair(p) = &mut self {
            p.borrow_mut().parent = Some(new_pair_rc.clone());
        }
        if let SnailfishNumber::Pair(p) = &mut other {
            p.borrow_mut().parent = Some(new_pair_rc.clone());
        }

        {
            let mut new_pair_mut = new_pair_rc.borrow_mut();
            new_pair_mut.left = self;
            new_pair_mut.right = other;
        }

        let mut num = SnailfishNumber::Pair(new_pair_rc);
        num.reduce();
        num
    }

    fn magnitude(&self) -> u32 {
        match self {
            SnailfishNumber::Regular(x) => *x,
            SnailfishNumber::Pair(pp) => {
                3 * pp.borrow().left.magnitude() + 2 * pp.borrow().right.magnitude()
            }
        }
    }
}

fn part1(input: &Vec<SnailfishNumber>) -> u32 {
    input
        .iter()
        .map(|t| t.clone_tree(None))
        .reduce(|acc, n| acc.add(n))
        .unwrap()
        .magnitude()
}

fn part2(input: &Vec<SnailfishNumber>) -> u32 {
    input
        .iter()
        .cartesian_product(input.iter())
        .map(|(a, b)| (a.clone_tree(None), b.clone_tree(None)))
        .map(|(a, b)| a.add(b).magnitude())
        .max()
        .unwrap()
}

fn parse_tree(l: &str, parent: Option<Rc<RefCell<PairNode>>>) -> (SnailfishNumber, usize) {
    if l.starts_with('[') {
        // parse pair
        let tree = Rc::new(RefCell::new(PairNode::new(parent)));
        let (left, llen) = parse_tree(&l[1..], Some(tree.clone()));
        let (right, rlen) = parse_tree(&l[1 + llen + 1..], Some(tree.clone()));
        {
            let mut tree_mut = tree.borrow_mut();
            tree_mut.left = left;
            tree_mut.right = right;
        }
        (SnailfishNumber::Pair(tree), 3 + llen + rlen)
    } else {
        // parse literal
        (
            SnailfishNumber::Regular(l.chars().next().unwrap().to_digit(10).unwrap()),
            1,
        )
    }
}

fn main() -> io::Result<()> {
    let input_str = aoc2021::read_input_lines()?;
    let input = input_str
        .into_iter()
        .map(|l| parse_tree(&l, None).0)
        .collect_vec();

    let p1 = part1(&input);
    println!("Part 1: {}", p1);

    let p2 = part2(&input);
    println!("Part 2: {}", p2);

    Ok(())
}
