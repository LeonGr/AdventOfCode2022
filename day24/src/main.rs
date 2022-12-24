#![allow(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::cast_possible_wrap
)]

use std::collections::{HashSet, VecDeque};

fn read_input() -> Vec<String> {
    include_str!("../input")
        .lines()
        .map(std::string::ToString::to_string)
        .collect()
}

fn parse(lines: &[String]) -> Valley {
    let fields: Vec<Vec<Cell>> =
        lines
            .iter()
            .skip(1)
            .take(lines.len() - 2)
            .map(|line| {
                line.chars()
                    .skip(1)
                    .take_while(|c| *c != '#')
                    .map(move |c| {
                        match c {
                            '^' => Cell(1, 0, 0, 0),
                            '>' => Cell(0, 1, 0, 0),
                            'v' => Cell(0, 0, 1, 0),
                            '<' => Cell(0, 0, 0, 1),
                            _ => Cell::default(),
                        }
                    })
                    .collect()
            })
            .collect();

    let width = fields.first().unwrap().len();
    let height = fields.len();
    Valley { fields, width, height, position: (0, -1) }
}

type Coord = i32;
type Pos = (Coord, Coord);

#[derive(Debug, Default, Clone)]
#[derive(Eq, Hash, PartialEq)]
struct Cell(usize, usize, usize, usize);

impl Cell {
    fn desctructure(&self) -> (usize, usize, usize, usize) {
        (self.0, self.1, self.2, self.3)
    }

    fn add(&mut self, other: &Cell) {
        self.0 += other.0;
        self.1 += other.1;
        self.2 += other.2;
        self.3 += other.3;
    }

    fn sum(&self) -> usize {
        self.0 + self.1 + self.2 + self.3
    }
}

#[derive(Debug, Clone)]
#[derive(Eq, Hash, PartialEq)]
struct Valley {
    fields: Vec<Vec<Cell>>,
    width: usize,
    height: usize,
    position: Pos,
}

impl ToString for Valley {
    fn to_string(&self) -> String {
        let top_botton: String = vec!["#".to_string(); self.width + 2].into_iter().collect();
        let mut output = String::new();
        output += top_botton.as_str();
        output += "\n";

        for row in &self.fields {
            output += "#";

            for cell in row {
                output +=
                    match cell.desctructure() {
                        (0, 0, 0, 0) => ".".to_string(),
                        (1, 0, 0, 0) => "^".to_string(),
                        (0, 1, 0, 0) => ">".to_string(),
                        (0, 0, 1, 0) => "v".to_string(),
                        (0, 0, 0, 1) => "<".to_string(),
                        _ => {
                            let sum = cell.sum();
                            if sum > 9 {
                                "+".to_string()
                            } else {
                                sum.to_string()
                            }
                        }
                    }.as_str();
            }

            output += "#\n";
        }

        output += top_botton.as_str();

        output
    }
}

impl Valley {
    fn updated_fields(&self) -> Vec<Vec<Cell>>  {
        let mut new = vec![vec![Cell::default(); self.width]; self.height];

        for (y, row) in &mut self.fields.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                let (up, right, down, left) = cell.desctructure();

                let y_up = (y as i32 - 1).rem_euclid(self.height as i32) as usize;
                let x_right = (x + 1).rem_euclid(self.width);
                let y_down = (y + 1).rem_euclid(self.height);
                let x_left = (x as i32 - 1).rem_euclid(self.width as i32) as usize;

                new[y_up][x].add(&Cell(up, 0, 0, 0));
                new[y][x_right].add(&Cell(0, right, 0, 0));
                new[y_down][x].add(&Cell(0, 0, down, 0));
                new[y][x_left].add(&Cell(0, 0, 0, left));
            }
        }

        new
    }

    fn update(&mut self) {
        self.fields = self.updated_fields();
    }

    fn possible_moves(&self) -> Vec<Pos> {
        let (x, y) = self.position;
        let displacements = [(0, 1), (0, -1), (1, 0), (-1, 0), (0, 0)];

        let updated_fields = self.updated_fields();

        displacements.iter().filter_map(|(dx, dy)| {
            let new @ (nx, ny) = (x + dx, y + dy);

            // start
            if new == (0, -1) {
                return Some(new);
            }

            // end
            if new == (self.width as i32 - 1, self.height as i32) {
                return Some(new);
            }

            // outside map (but not start or end)
            if nx < 0 || ny < 0 || nx >= self.width as i32 || ny >= self.height as i32 {
                return None;
            }

            let field = &updated_fields[ny as usize][nx as usize];

            if field.sum() == 0 {
                Some(new)
            } else {
                None
            }
        }).collect()
    }
}

type Time = usize;

fn part1(valley: &Valley) -> usize {
    let valley = valley.clone();
    // println!("{:?}", valley);

    // println!("{}\n", valley.to_string());
    // println!("possible moves: {:?}", valley.possible_moves());

    let target = (valley.width as i32 - 1, valley.height as i32);

    let mut seen: HashSet<(Pos, Time)> = HashSet::new();
    let mut queue = VecDeque::from([(valley, 0)]);
    // let mut depths: HashMap<Valley, usize> = HashMap::from([(valley, 0)]);

    while !queue.is_empty() {
        let (last, time) = queue.pop_front().unwrap();
        if last.position == target {
            return time;
        }

        let possible_moves = last.possible_moves();

        for possible_move in possible_moves {
            let mut clone = last.clone();
            clone.update();
            clone.position = possible_move;

            let new_time = time + 1;

            if seen.get(&(clone.position, new_time)).is_none() {
                seen.insert((clone.position, new_time));
                queue.push_back((clone, new_time));
            }
        }
    }

    unreachable!()
}

fn part2(valley: &Valley) -> usize {
    let valley = valley.clone();
    // println!("{:?}", valley);

    // println!("{}\n", valley.to_string());
    // println!("possible moves: {:?}", valley.possible_moves());

    let start = (0, -1);
    let end = (valley.width as i32 - 1, valley.height as i32);

    let mut seen: HashSet<(Pos, Time)> = HashSet::new();
    let mut queue = VecDeque::from([(valley, 0)]);

    let target = end;

    while !queue.is_empty() {
        let (last, time) = queue.pop_front().unwrap();
        if last.position == target {
            queue.clear();
            seen.clear();
            queue.push_back((last, time));
            break;
        }

        let possible_moves = last.possible_moves();

        for possible_move in possible_moves {
            let mut clone = last.clone();
            clone.update();
            clone.position = possible_move;

            let new_time = time + 1;

            if seen.get(&(clone.position, new_time)).is_none() {
                seen.insert((clone.position, new_time));
                queue.push_back((clone, new_time));
            }
        }
    }

    let target = start;

    while !queue.is_empty() {
        let (last, time) = queue.pop_front().unwrap();
        if last.position == target {
            queue.clear();
            seen.clear();
            queue.push_back((last, time));
            break;
        }

        let possible_moves = last.possible_moves();

        for possible_move in possible_moves {
            let mut clone = last.clone();
            clone.update();
            clone.position = possible_move;

            let new_time = time + 1;

            if seen.get(&(clone.position, new_time)).is_none() {
                seen.insert((clone.position, new_time));
                queue.push_back((clone, new_time));
            }
        }
    }

    let target = end;

    while !queue.is_empty() {
        let (last, time) = queue.pop_front().unwrap();
        if last.position == target {
            return time;
        }

        let possible_moves = last.possible_moves();

        for possible_move in possible_moves {
            let mut clone = last.clone();
            clone.update();
            clone.position = possible_move;

            let new_time = time + 1;

            if seen.get(&(clone.position, new_time)).is_none() {
                seen.insert((clone.position, new_time));
                queue.push_back((clone, new_time));
            }
        }
    }

    unreachable!()
}

fn main() {
    let lines = read_input();
    let parsed = parse(&lines);

    println!("part1: {}", part1(&parsed));
    println!("part2: {}", part2(&parsed));
}
