use itertools::Itertools;
use ndarray::Array2;
use std::{
    collections::{BTreeMap, BinaryHeap, HashMap, HashSet},
    io,
};

#[derive(Clone, Copy, Debug)]
enum Square {
    Wall,
    Corridor,
    Room(char),
}

#[derive(Clone, Eq, Hash, PartialEq)]
struct State(BTreeMap<(usize, usize), char>);

fn amphipod_move_cost(ap: char) -> usize {
    match ap {
        'A' => 1,
        'B' => 10,
        'C' => 100,
        'D' => 1000,
        _ => panic!(),
    }
}

fn abs_diff(a: usize, b: usize) -> usize {
    if a > b {
        a - b
    } else {
        b - a
    }
}

fn print_state(grid: &Array2<Square>, state: &State) {
    for (y, r) in grid.rows().into_iter().enumerate() {
        for (x, s) in r.indexed_iter() {
            if let Some(ap) = state.0.get(&(y, x)) {
                print!("{}", ap);
            } else {
                print!(
                    "{}",
                    match s {
                        Square::Wall => '#',
                        _ => '.',
                    }
                );
            }
        }
        println!();
    }
    println!();
}

fn adjacents(x: usize, y: usize, w: usize, h: usize) -> impl Iterator<Item = (usize, usize)> {
    [
        if x > 0 { Some((y, x - 1)) } else { None },
        if x < w - 1 { Some((y, x + 1)) } else { None },
        if y > 0 { Some((y - 1, x)) } else { None },
        if y < h - 1 { Some((y + 1, x)) } else { None },
    ]
    .into_iter()
    .flatten()
}

fn cost(ap: char, from: (usize, usize), to: (usize, usize)) -> usize {
    let unit = amphipod_move_cost(ap);

    let x = if from.1 > to.1 {
        from.1 - to.1
    } else {
        to.1 - from.1
    };
    
    let y = if x == 0 {
        if from.0 > to.0 {
            from.0 - to.0
        } else {
            to.0 - from.0
        }
    } else {
        from.0 - 1 + to.0 - 1
    };

    unit * (x + y)
}

fn move_amphipod<'a>(
    ap: char,
    pos: (usize, usize),
    grid: &'a Array2<Square>,
    state: &'a State,
) -> impl Iterator<Item = ((usize, usize), usize)> + 'a {
    reachable_positions(pos, grid, state)
        .into_iter()
        .filter_map(move |(dy, dx)| match (grid[pos], grid[(dy, dx)]) {
            (Square::Room(aap), Square::Room(bap)) if aap == bap && dy < pos.0 => None, 
            (_, Square::Room(rap)) if rap == ap && room_clean(ap, state, grid) => {
                Some(((dy, dx), cost(ap, pos, (dy, dx))))
            }
            (Square::Room(_), Square::Corridor) if ![3, 5, 7, 9].contains(&dx) => {
                Some(((dy, dx), cost(ap, pos, (dy, dx))))
            }
            (Square::Corridor, Square::Corridor) => None,
            _ => None,
        })
}

fn reachable_positions(
    pos: (usize, usize),
    grid: &Array2<Square>,
    state: &State,
) -> Vec<(usize, usize)> {
    let mut res = vec![pos];

    loop {
        let new_res = res
            .iter()
            .flat_map(|&(y, x)| adjacents(x, y, grid.ncols(), grid.nrows()))
            .chain(res.iter().copied())
            .unique()
            .filter(|c| *c != pos && !matches!(grid[*c], Square::Wall) && !state.0.contains_key(c))
            .sorted()
            .collect();

        if new_res == res {
            return res;
        } else {
            res = new_res;
        }
    }
}

fn room_clean(room_ap: char, state: &State, grid: &Array2<Square>) -> bool {
    let x = amphipod_room_x_coord(room_ap);
    (2..grid.nrows()-1).all(|y| state.0.get(&(y, x)).map(|&c| c == room_ap).unwrap_or(true))
}

fn amphipod_room_x_coord(ap: char) -> usize {
    match ap {
        'A' => 3,
        'B' => 5,
        'C' => 7,
        'D' => 9,
        _ => panic!("Unknown room name"),
    }
}

#[derive(Eq, PartialEq)]
struct CostState(usize, usize, State);
impl PartialOrd for CostState {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        (other.0 + other.1).partial_cmp(&(self.0 + self.1))
    }
}
impl Ord for CostState {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (other.0 + other.1).cmp(&(self.0 + self.1))
    }
}

fn heuristic(grid: &Array2<Square>, state: &State) -> usize {
    state
        .0
        .iter()
        .map(|(&coord, &ap)| {
            if let Square::Room(rap) = grid[coord] {
                if rap == ap {
                    return 0;
                }
            }
            amphipod_move_cost(ap) * (coord.0 + abs_diff(coord.1, amphipod_room_x_coord(ap)))
        })
        .sum()
}

fn run(grid: &Array2<Square>, init: &State) -> usize {
    let mut q = BinaryHeap::new();
    let mut visited = HashSet::new();
    q.push(CostState(0, heuristic(grid, init), init.clone()));

    while let Some(CostState(cost, _, state)) = q.pop() {
        if visited.contains(&state) {
            continue;
        }
        //print_state(grid, &state);
        if is_final(&state, grid) {
            println!();
            return cost;
        }

        for (&sc, &sap) in &state.0 {
            if sc.0 >= 2 && sc.1 == amphipod_room_x_coord(sap) {
                if (sc.0..(grid.nrows() - 1))
                    .all(|yy| matches!(state.0.get(&(yy, sc.1)), Some(apap) if *apap == sap))
                {
                    continue;
                }
            }
            let moves = move_amphipod(sap, sc, grid, &state);
            for (dest, mcost) in moves {
                let mut new_state = state.clone();
                new_state.0.remove(&sc);
                new_state.0.insert(dest, sap);
                if !visited.contains(&new_state) {
                    q.push(CostState(
                        cost + mcost,
                        heuristic(grid, &new_state),
                        new_state,
                    ));
                }
            }
        }

        visited.insert(state);
    }

    unreachable!()
}

fn is_final(state: &State, grid: &Array2<Square>) -> bool {
    state
        .0
        .iter()
        .all(|(c, &ap)| matches!(grid[*c], Square::Room(rap) if rap == ap))
}

fn read_grid(lines: &Vec<String>) -> (Array2<Square>, State) {
    let w = lines[0].len();
    let h = lines.len();

    let mut arr = Array2::from_elem((h, w), Square::Wall);

    let mut amphipods = BTreeMap::new();

    for (y, l) in lines.into_iter().enumerate() {
        for (x, c) in l.chars().enumerate() {
            let sq = match c {
                '#' | ' ' => Square::Wall,
                '.' if y == 1 => Square::Corridor,
                _ => {
                    amphipods.insert((y, x), c);
                    Square::Room(match x {
                        3 => 'A',
                        5 => 'B',
                        7 => 'C',
                        9 => 'D',
                        _ => panic!("Invalid room x coordinate"),
                    })
                }
            };
            arr[(y, x)] = sq;
        }
    }

    (arr, State(amphipods))
}

fn main() -> io::Result<()> {
    let mut input = aoc2021::read_input_lines()?;
    let (arr, amphipods) = read_grid(&input);

    let p1 = run(&arr, &amphipods);
    println!("Part 1: {}", p1);

    input.insert(3, "  #D#C#B#A#".to_string());
    input.insert(4, "  #D#B#A#C#".to_string());

    let (arr, amphipods) = read_grid(&input);
    let p2 = run(&arr, &amphipods);
    println!("Part 2: {}", p2);

    Ok(())
}
