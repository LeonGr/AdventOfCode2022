use core::panic;
use std::collections::{HashMap, HashSet, VecDeque};

fn read_input() -> Vec<String> {
    let input = include_str!("../input");
    input
        .lines()
        .map(std::string::ToString::to_string)
        .collect()
}

type Coord = i32;
type Pos = (Coord, Coord);

fn parse(lines: &[String]) -> HashSet<Pos> {
    lines
        .iter()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter_map(|(x, c)| match c {
                    '#' => Some((x as i32, y as i32)),
                    _ => None,
                })
                .collect::<Vec<Pos>>()
        })
        .collect()
}

#[derive(Debug)]
enum Direction {
    North,
    East,
    South,
    West,
}

fn count_free(elves: &HashSet<Pos>, (x, y): Pos, displacements: &[Pos]) -> usize {
    displacements
        .iter()
        .filter(|(dx, dy)| {
            let n = (x + dx, y + dy);

            !elves.contains(&n)
        })
        .count()
}

fn count_free_north(elves: &HashSet<Pos>, pos: Pos) -> usize {
    count_free(elves, pos, &[(-1, -1), (0, -1), (1, -1)])
}

fn count_free_south(elves: &HashSet<Pos>, pos: Pos) -> usize {
    count_free(elves, pos, &[(-1, 1), (0, 1), (1, 1)])
}

fn count_free_west(elves: &HashSet<Pos>, pos: Pos) -> usize {
    count_free(elves, pos, &[(-1, -1), (-1, 0), (-1, 1)])
}

fn count_free_east(elves: &HashSet<Pos>, pos: Pos) -> usize {
    count_free(elves, pos, &[(1, -1), (1, 0), (1, 1)])
}

fn count_free_neightbours(elves: &HashSet<Pos>, pos: Pos) -> usize {
    count_free(
        elves,
        pos,
        &[
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, 1),
            (0, -1),
            (1, -1),
            (1, 0),
            (1, 1),
        ],
    )
}

fn part1(elves: &HashSet<Pos>) -> usize {
    let mut elves_copy = elves.clone();

    let mut directions_considered = VecDeque::from([
        Direction::North,
        Direction::South,
        Direction::West,
        Direction::East,
    ]);

    println!("initial state:");
    print_state(&elves_copy);
    println!("elves: {:?}", elves_copy);

    let rounds = 10;
    for i in 0..rounds {
        let mut proposed: HashMap<Pos, Vec<Pos>> = HashMap::new();
        println!("directions: {:?}", directions_considered);

        // first half
        for elf in &elves_copy {
            println!("elf: {:?}", elf);
            // If no elf in 8 adjactent, do nothing
            // else, propose
            println!("free neightbours: {}", count_free_neightbours(&elves_copy, *elf));

            if count_free_neightbours(&elves_copy, *elf) != 8 {
                for direction in &directions_considered {
                    println!("consider {:?}", direction);
                    let considered_pos = match direction {
                        Direction::North => {
                            if count_free_north(&elves_copy, *elf) == 3 {
                                Some((elf.0, elf.1 - 1))
                            } else {
                                None
                            }
                        }
                        Direction::East => {
                            if count_free_east(&elves_copy, *elf) == 3 {
                                Some((elf.0 + 1, elf.1))
                            } else {
                                None
                            }
                        }
                        Direction::South => {
                            if count_free_south(&elves_copy, *elf) == 3 {
                                Some((elf.0, elf.1 + 1))
                            } else {
                                None
                            }
                        }
                        Direction::West => {
                            if count_free_west(&elves_copy, *elf) == 3 {
                                Some((elf.0 - 1, elf.1))
                            } else {
                                None
                            }
                        }
                    };

                    if let Some(considered_pos) = considered_pos {
                        println!("propose: {:?}", considered_pos);
                        let entry = proposed.entry(considered_pos).or_default();
                        entry.push(*elf);
                        break;
                    }
                }
            }
        }

        // println!("proposed: {:?}", proposed);

        // second half
        for (pos, elves_proposed) in proposed {
            // if they were the only elf to propose that position, move.
            // else, do nothing
            if elves_proposed.len() == 1 {
                let elf = elves_proposed[0];
                if !elves_copy.remove(&elf) {
                    panic!()
                }
                if !elves_copy.insert(pos) {
                    panic!()
                }
            }
        }

        directions_considered.rotate_left(1);

        println!("end of round {}", i + 1);
        print_state(&elves_copy);
    }

    let (min_x, max_x, min_y, max_y) = get_boundaries(&elves_copy);

    let mut count = 0;
    for x in min_x..=max_x {
        for y in min_y..=max_y {
            if !elves_copy.contains(&(x, y)) {
                count += 1;
            }
        }
    }

    count
}

fn print_state(elves: &HashSet<(i32, i32)>) {
    let (min_x, max_x, min_y, max_y) = get_boundaries(elves);

    let mut output = String::new();

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            if elves.contains(&(x, y)) {
                output += "#";
            } else {
                output += ".";
            }
        }

        output += "\n";
    }

    println!("{}", output);
}

fn get_boundaries(elves: &HashSet<Pos>) -> (Coord, Coord, Coord, Coord) {
    elves.iter().fold(
        (Coord::MAX, Coord::MIN, Coord::MAX, Coord::MIN),
        |(min_x, max_x, min_y, max_y), (x, y)| {
            (min_x.min(*x), max_x.max(*x), min_y.min(*y), max_y.max(*y))
        },
    )
}

fn part2(elves: &HashSet<Pos>) -> usize {
    let mut elves_copy = elves.clone();

    let mut directions_considered = VecDeque::from([
        Direction::North,
        Direction::South,
        Direction::West,
        Direction::East,
    ]);

    // print_state(&elves_copy);

    let mut rounds = 0;
    loop {
        rounds += 1;
        let mut proposed: HashMap<Pos, Vec<Pos>> = HashMap::new();
        // first half
        for elf in &elves_copy {
            // If no elf in 8 adjactent, do nothing
            // else, propose

            if count_free_neightbours(&elves_copy, *elf) != 8 {
                for direction in &directions_considered {
                    let considered_pos = match direction {
                        Direction::North => {
                            if count_free_north(&elves_copy, *elf) == 3 {
                                Some((elf.0, elf.1 - 1))
                            } else {
                                None
                            }
                        }
                        Direction::East => {
                            if count_free_east(&elves_copy, *elf) == 3 {
                                Some((elf.0 + 1, elf.1))
                            } else {
                                None
                            }
                        }
                        Direction::South => {
                            if count_free_south(&elves_copy, *elf) == 3 {
                                Some((elf.0, elf.1 + 1))
                            } else {
                                None
                            }
                        }
                        Direction::West => {
                            if count_free_west(&elves_copy, *elf) == 3 {
                                Some((elf.0 - 1, elf.1))
                            } else {
                                None
                            }
                        }
                    };

                    if let Some(considered_pos) = considered_pos {
                        let entry = proposed.entry(considered_pos).or_default();
                        entry.push(*elf);
                        break;
                    }
                }
            }
        }

        // second half
        let mut any_moves = false;
        for (pos, elves_proposed) in proposed {
            // if they were the only elf to propose that position, move.
            // else, do nothing
            if elves_proposed.len() == 1 {
                let elf = elves_proposed[0];
                if !elves_copy.remove(&elf) {
                    panic!()
                }
                if !elves_copy.insert(pos) {
                    panic!()
                }

                any_moves = true;
            }
        }

        if !any_moves {
            break;
        }

        directions_considered.rotate_left(1);
    }

    rounds
}

fn main() {
    let lines = read_input();
    let parsed = parse(&lines);

    println!("part1: {}", part1(&parsed));
    println!("part2: {}", part2(&parsed));
}
