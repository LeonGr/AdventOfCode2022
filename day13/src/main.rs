use std::{cmp::Ordering, str::FromStr};

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
        get_order(self, other)
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
        let mut parts: Vec<String> = vec![];

        let mut i = 0;
        let chars: Vec<char> = string.chars().collect();
        while let Some(c) = chars.get(i) {
            match c {
                ',' => (),
                '[' | ']' => {
                    let x = c.to_string();
                    parts.push(x);
                }
                d => {
                    parts.push(d.to_string());

                    let mut j = i + 1;
                    while let Some(y) = chars.get(j) {
                        if y.is_numeric() {
                            let mut last = parts.pop().unwrap();
                            last += &y.to_string();
                            parts.push(last);
                        } else {
                            break;
                        }

                        j += 1;
                    }

                    i = j - 1;
                }
            }

            i += 1;
        }

        let mut stack: Vec<Packet> = vec![];

        parts.iter().for_each(|c| match c.to_string().as_str() {
            "[" => {
                stack.push(Packet::List(vec![]));
            }
            "]" => {
                let packet = stack.pop().unwrap();
                if let Some(last_packet) = stack.last_mut() {
                    match last_packet {
                        Packet::List(packet_items) => packet_items.push(packet),
                        Packet::Item(_) => unreachable!(),
                    }
                } else {
                    stack.push(packet);
                }
            }
            d => {
                let num: u8 = d.parse().unwrap();
                let last = stack.last_mut().unwrap();
                match last {
                    Packet::List(packet_items) => packet_items.push(Packet::Item(num)),
                    Packet::Item(_) => unreachable!(),
                }
            }
        });

        Ok(stack.pop().unwrap())
    }
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

fn get_order(first: &Packet, second: &Packet) -> Ordering {
    match (first, second) {
        (Packet::List(first_items), Packet::List(second_items)) => {
            for i in 0..(usize::max(first_items.len(), second_items.len())) {
                match (first_items.get(i), second_items.get(i)) {
                    (None, None | Some(_)) => return Ordering::Less,
                    (Some(_), None) => return Ordering::Greater,
                    (Some(left), Some(right)) => match get_order(left, right) {
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
            get_order(left, &list_single_item)
        }
        (Packet::Item(d), right @ Packet::List(_)) => {
            let list_single_item = Packet::List(vec![Packet::Item(*d)]);
            get_order(&list_single_item, right)
        }
        (Packet::Item(left), Packet::Item(right)) => match (left, right) {
            (l, r) if l == r => Ordering::Equal,
            (l, r) if l < r => Ordering::Less,
            (l, r) if l > r => Ordering::Greater,
            _ => unreachable!(),
        },
    }
}

fn part1(pairs: &[Pair]) -> usize {
    pairs
        .iter()
        .enumerate()
        .filter(|(_, (first, second))| match get_order(first, second) {
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

    println!("part1: {} should be 5330", part1(&parsed));
    println!("part2: {} should be 27648", part2(&parsed));
}
