use std::{cmp::Ordering, str::FromStr};

use nom::{
    branch::alt, character::complete::char, character::complete::u8, combinator::map,
    multi::separated_list0, sequence::delimited, IResult,
};

fn read_input() -> String {
    let input = include_str!("../input");
    input.to_owned()
}

#[derive(Clone, Debug)]
enum Packet {
    List(Vec<Packet>),
    Item(u8),
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Packet::List(first_items), Packet::List(second_items)) => {
                for i in 0..(usize::max(first_items.len(), second_items.len())) {
                    match (first_items.get(i), second_items.get(i)) {
                        (None, None | Some(_)) => return Ordering::Less,
                        (Some(_), None) => return Ordering::Greater,
                        (Some(left), Some(right)) => match left.cmp(right) {
                            Ordering::Less => return Ordering::Less,
                            Ordering::Equal => continue,
                            Ordering::Greater => return Ordering::Greater,
                        },
                    }
                }

                Ordering::Equal
            }
            (left @ Packet::List(_), Packet::Item(d)) => {
                let list_single_item = Packet::List(vec![Packet::Item(*d)]);
                left.cmp(&list_single_item)
            }
            (Packet::Item(d), right @ Packet::List(_)) => {
                let list_single_item = Packet::List(vec![Packet::Item(*d)]);
                list_single_item.cmp(right)
            }
            (Packet::Item(left), Packet::Item(right)) => match (left, right) {
                (l, r) if l == r => Ordering::Equal,
                (l, r) if l < r => Ordering::Less,
                (l, r) if l > r => Ordering::Greater,
                _ => unreachable!(),
            },
        }

    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Packet {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::List(l0), Self::List(r0)) => l0 == r0,
            (Self::Item(l0), Self::Item(r0)) => l0 == r0,
            _ => false,
        }
    }
}

impl Eq for Packet {}

impl FromStr for Packet {
    type Err = String;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        match packet(string) {
            Ok((_, packet)) => Ok(packet),
            Err(err) => {
                println!("{:?}", err);
                Err(String::from("Error"))
            }
        }
    }
}

fn list(input: &str) -> IResult<&str, Vec<Packet>> {
    delimited(char('['), separated_list0(char(','), packet), char(']'))(input)
}

fn packet(input: &str) -> IResult<&str, Packet> {
    use Packet::{Item, List};

    alt((map(list, List), map(u8, Item)))(input)
}

type Pair = (Packet, Packet);

fn parse(input: &str) -> Vec<Pair> {
    input
        .split("\n\n")
        .map(|pair| {
            let parts: Vec<Packet> = pair
                .trim()
                .split('\n')
                .map(|packet| Packet::from_str(packet).unwrap())
                .collect();

            (parts[0].clone(), parts[1].clone())
        })
        .collect()
}

fn part1(pairs: &[Pair]) -> usize {
    pairs
        .iter()
        .enumerate()
        .filter(|(_, (first, second))| match first.cmp(second) {
            Ordering::Less => true,
            Ordering::Equal => unreachable!(),
            Ordering::Greater => false,
        })
        .map(|(i, _)| i + 1)
        .sum()
}

fn part2(pairs: &[Pair]) -> usize {
    let mut packets: Vec<Packet> = pairs
        .iter()
        .flat_map(|(left, right)| vec![left.clone(), right.clone()])
        .collect();

    let first_divider = Packet::List(vec![Packet::List(vec![Packet::Item(2)])]);
    let second_divider = Packet::List(vec![Packet::List(vec![Packet::Item(6)])]);

    packets.push(first_divider.clone());
    packets.push(second_divider.clone());

    packets.sort();

    packets
        .iter()
        .enumerate()
        .filter_map(|(i, packet)| {
            if *packet == first_divider || *packet == second_divider {
                Some(i + 1)
            } else {
                None
            }
        })
        .product()
}

fn main() {
    let input = read_input();
    let parsed = parse(&input);

    println!("part1: {}", part1(&parsed));
    println!("part2: {}", part2(&parsed));
}
