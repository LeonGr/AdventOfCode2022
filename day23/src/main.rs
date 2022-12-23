use std::collections::{HashMap, HashSet, VecDeque};

fn read_input() -> Vec<String> {
    include_str!("../input").lines().map(std::string::ToString::to_string).collect()
}

type Coord = i32;
type Pos = (Coord, Coord);

fn parse(lines: &[String]) -> HashSet<Pos> {
    lines.iter().enumerate().flat_map(|(y, line)| {
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
    displacements.iter().filter(|(dx, dy)| !elves.contains(&(x + dx, y + dy))).count()
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
    let adjactent = [(-1, -1), (-1, 0), (-1, 1), (0, 1), (0, -1), (1, -1), (1, 0), (1, 1)];
    count_free(elves, pos, &adjactent)
}

type Rounds = usize;

fn elf_dance(elves: &HashSet<Pos>, rounds: Option<Rounds>) -> (HashSet<Pos>, Rounds) {
    let mut elves_copy = elves.clone();

    let mut directions_considered = VecDeque::from([
        Direction::North,
        Direction::South,
        Direction::West,
        Direction::East,
    ]);

    let mut proposed: HashMap<Pos, Vec<Pos>> = HashMap::new();
    let mut rounds_done = 0;
    loop {
        rounds_done += 1;

        for elf in &elves_copy {
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


        let mut any_moves = false;
        for (pos, elves_proposed) in &proposed {
            if elves_proposed.len() == 1 {
                elves_copy.remove(&elves_proposed[0]);
                elves_copy.insert(*pos);
                any_moves = true;
            }
        }

        if let Some(rounds) = rounds {
            if rounds_done == rounds {
                break;
            }
        } else if !any_moves {
            break;
        }

        directions_considered.rotate_left(1);
        proposed.clear();
    }

    (elves_copy, rounds_done)
}

fn get_boundaries(elves: &HashSet<Pos>) -> (Coord, Coord, Coord, Coord) {
    elves.iter().fold(
        (Coord::MAX, Coord::MIN, Coord::MAX, Coord::MIN),
        |(min_x, max_x, min_y, max_y), (x, y)| {
            (min_x.min(*x), max_x.max(*x), min_y.min(*y), max_y.max(*y))
        },
    )
}

fn part1(elves: &HashSet<Pos>) -> usize {
    let (elves_copy, _) = elf_dance(elves, Some(10));

    let (min_x, max_x, min_y, max_y) = get_boundaries(&elves_copy);

    (min_x..=max_x).fold(0, |acc, x| {
        (min_y..=max_y).fold(0, |acc, y| {
            if elves_copy.contains(&(x, y)) {
                acc
            } else {
                acc + 1
            }
        }) + acc
    })
}

fn part2(elves: &HashSet<Pos>) -> usize {
    elf_dance(elves, None).1
}

fn main() {
    let lines = read_input();
    let parsed = parse(&lines);

    println!("part1: {}", part1(&parsed));
    println!("part2: {}", part2(&parsed));
}
