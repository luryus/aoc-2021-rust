use itertools::Itertools;

use std::io;

struct PacketHeader {
    version: u8,
    type_id: u8,
}

enum Packet {
    Literal(PacketHeader, u64),
    Operator(PacketHeader, Vec<Packet>),
}

impl Packet {
    fn version_sum(&self) -> usize {
        match self {
            Packet::Literal(header, _) => header.version as usize,
            Packet::Operator(header, sub) => {
                header.version as usize + sub.iter().map(|p| p.version_sum()).sum::<usize>()
            }
        }
    }

    fn eval(&self) -> usize {
        match self {
            Packet::Literal(_, v) => *v as usize,
            Packet::Operator(PacketHeader { type_id, .. }, sub) => match *type_id {
                0 => sub.iter().map(|s| s.eval()).sum(),
                1 => sub.iter().map(|s| s.eval()).product(),
                2 => sub.iter().map(|s| s.eval()).min().unwrap(),
                3 => sub.iter().map(|s| s.eval()).max().unwrap(),
                5 => (sub[0].eval() > sub[1].eval()) as usize,
                6 => (sub[0].eval() < sub[1].eval()) as usize,
                7 => (sub[0].eval() == sub[1].eval()) as usize,
                _ => panic!("Unknown type id"),
            },
        }
    }
}

fn read_u16(data: &[bool]) -> u16 {
    data.iter().fold(0, |acc, bit| acc << 1 | *bit as u16)
}

fn read_literal(data: &[bool]) -> (u64, usize) {
    // Varint-ish
    let chunks = data.iter().copied().chunks(5);
    let parts = chunks
        .into_iter()
        .map(|mut c| (c.next().unwrap(), read_u16(&c.collect_vec())));

    let mut res = 0u64;
    let mut len = 0;
    for (more, part) in parts {
        len += 1;
        res = res << 4 | part as u64;
        if !more {
            break;
        }
    }
    (res, len * 5)
}

fn read_operator(data: &[bool]) -> (Vec<Packet>, usize) {
    if data[0] {
        // number of sub packets
        let num_subpackets = read_u16(&data[1..12]);
        let (len, res) = (0..num_subpackets).fold((12, vec![]), |(start, mut ps), _| {
            let (p, len) = read_packet(&data[start..]);
            ps.push(p);
            (start + len, ps)
        });
        (res, len)
    } else {
        // total length of subpackets
        let len = read_u16(&data[1..16]) as usize;
        let mut next_start = 16usize;
        let mut res = vec![];
        while next_start < (16 + len) {
            let (p, len) = read_packet(&data[next_start..]);
            res.push(p);
            next_start += len;
        }

        (res, next_start)
    }
}

fn read_packet(data: &[bool]) -> (Packet, usize) {
    let header = PacketHeader {
        version: read_u16(&data[0..3]) as u8,
        type_id: read_u16(&data[3..6]) as u8,
    };

    match header.type_id {
        4 => {
            let (val, len) = read_literal(&data[6..]);
            (Packet::Literal(header, val), len + 6)
        }
        _ => {
            let (val, len) = read_operator(&data[6..]);
            (Packet::Operator(header, val), len + 6)
        }
    }
}

fn read_packets(input: &str) -> Packet {
    let bits = input
        .chars()
        .flat_map(|c| {
            let b = c.to_digit(16).unwrap();
            [
                (b >> 3 & 0x1) != 0,
                (b >> 2 & 0x1) != 0,
                (b >> 1 & 0x1) != 0,
                (b & 0x1) != 0,
            ]
        })
        .collect_vec();

    let (p, _) = read_packet(&bits);
    p
}

fn main() -> io::Result<()> {
    let input = aoc2021::read_input_string()?;
    let input = input.trim();
    let root_packet = read_packets(input);

    println!("Part 1: {}", root_packet.version_sum());
    println!("Part 2: {}", root_packet.eval());

    Ok(())
}
