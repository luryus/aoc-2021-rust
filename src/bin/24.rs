// Todays puzzle was the traditional assembly decompilation.
// I did not solve it by making an interpreter for the code,
// I just decompiled the code by hand while being away from
// home for Christmas. And then deduced how the thing works
// and finally used Excel to more easily find the solution.

// I did not bother writing the explanation here, there's a
// bunch of very good ones on the subreddit.

// Just for fun I implemented the interpreter here later, and
// used it to validate the correct answers.

use std::io;

#[derive(Clone, Copy)]
enum Operand {
    Var(char),
    Literal(i32),
}

#[derive(Clone, Copy)]
enum Instr {
    Inp(char),
    Add(char, Operand),
    Sub(char, Operand),
    Mul(char, Operand),
    Div(char, Operand),
    Mod(char, Operand),
    Eql(char, Operand),
}

fn part1(prg: &Vec<Instr>) -> usize {
    let n = "59996912981939"
        .chars()
        .map(|c| c.to_digit(10).unwrap() as i32)
        .collect();

    if execute(prg, &n) != 0 {
        panic!("Answer not correct, interpreter not working?");
    }
    59996912981939
}

fn part2(prg: &Vec<Instr>) -> usize {
    let n = "17241911811915"
        .chars()
        .map(|c| c.to_digit(10).unwrap() as i32)
        .collect();
    if execute(prg, &n) != 0 {
        panic!("Answer not correct, interpreter not working?");
    }
    17241911811915
}

struct Registers {
    x: i32,
    y: i32,
    z: i32,
    w: i32,
}

impl Registers {
    fn new() -> Self {
        Self {
            x: 0,
            y: 0,
            z: 0,
            w: 0,
        }
    }

    fn get_mut<'a>(&mut self, c: char) -> &mut i32 {
        match c {
            'x' => &mut self.x,
            'z' => &mut self.z,
            'y' => &mut self.y,
            'w' => &mut self.w,
            _ => panic!(),
        }
    }

    fn op_val(&self, op: Operand) -> i32 {
        match op {
            Operand::Literal(v) => v,
            Operand::Var(v) => match v {
                'x' => self.x,
                'z' => self.z,
                'y' => self.y,
                'w' => self.w,
                _ => panic!(),
            },
        }
    }
}

fn execute(prg: &Vec<Instr>, inputs: &Vec<i32>) -> i32 {
    let mut regs = Registers::new();
    let mut inp_iter = inputs.iter();

    for i in prg {
        match i {
            &Instr::Inp(v) => {
                *regs.get_mut(v) = *inp_iter.next().expect("No more inputs remaining")
            }
            &Instr::Add(v, op) => *regs.get_mut(v) += regs.op_val(op),
            &Instr::Sub(v, op) => *regs.get_mut(v) -= regs.op_val(op),
            &Instr::Mul(v, op) => *regs.get_mut(v) *= regs.op_val(op),
            &Instr::Div(v, op) => *regs.get_mut(v) /= regs.op_val(op),
            &Instr::Mod(v, op) => *regs.get_mut(v) %= regs.op_val(op),
            &Instr::Eql(v, op) => *regs.get_mut(v) = (*regs.get_mut(v) == regs.op_val(op)) as i32,
        }
    }

    regs.z
}

fn read_program(prg: &Vec<String>) -> Vec<Instr> {
    prg.iter()
        .map(|l| {
            if l.starts_with("inp") {
                return Instr::Inp(l.chars().last().unwrap());
            }
            let (ins, target, operand) = aoc2021::split_to_tuple3(l, " ").unwrap();
            let operand = if operand.starts_with(|c: char| c.is_alphabetic()) {
                Operand::Var(operand.chars().next().unwrap())
            } else {
                Operand::Literal(operand.parse().unwrap())
            };
            let target = target.chars().next().unwrap();
            match ins {
                "add" => Instr::Add(target, operand),
                "sub" => Instr::Sub(target, operand),
                "mul" => Instr::Mul(target, operand),
                "div" => Instr::Div(target, operand),
                "mod" => Instr::Mod(target, operand),
                "eql" => Instr::Eql(target, operand),
                _ => panic!("Unknown instruction"),
            }
        })
        .collect()
}

fn main() -> io::Result<()> {
    let input = aoc2021::read_input_lines()?;
    let prg = read_program(&input);

    let p1 = part1(&prg);
    println!("Part 1: {}", p1);

    let p2 = part2(&prg);
    println!("Part 2: {}", p2);

    Ok(())
}
