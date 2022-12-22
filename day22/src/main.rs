#![allow(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::cast_possible_wrap
)]

use nom::{
    branch::alt, character::complete::one_of, character::complete::u8, combinator::map,
    multi::many0, IResult,
};

fn read_input() -> Vec<String> {
    let input = include_str!("../input");
    input
        .lines()
        .map(std::string::ToString::to_string)
        .collect()
}

fn parse(lines: &[String]) -> (Grid, Vec<Instruction>) {
    let mut i = 0;
    let mut cells = vec![];

    loop {
        let line = &lines[i];
        i += 1;

        if line.is_empty() {
            break;
        }

        let mut row = vec![];

        for c in line.chars() {
            let cell = match c {
                '.' => Some(Cell::Tile),
                '#' => Some(Cell::Wall),
                ' ' => None,
                _ => unreachable!(),
            };

            row.push(cell);
        }

        cells.push(row);
    }

    // cells.reverse();

    let instruction_line = &lines[i];
    // println!("{}", instruction_line);

    let instructions = match instructions(instruction_line) {
        Ok((_, instructions)) => instructions,
        Err(_) => unreachable!(),
    };

    let grid = Grid { cells };

    (grid, instructions)
}

fn turn(i: &str) -> IResult<&str, Rotation> {
    let (matched, direction) = one_of("RL")(i)?;

    let rotation = match direction {
        'R' => Rotation::Clockwise,
        'L' => Rotation::Counterclockwise,
        _ => unreachable!(),
    };

    Ok((matched, rotation))
}

fn instructions(input: &str) -> IResult<&str, Vec<Instruction>> {
    use Instruction::{Move, Turn};

    many0(alt((map(turn, Turn), map(u8, Move))))(input)
}

#[derive(Debug, Clone, Copy)]
enum Cell {
    Wall,
    Tile,
}

#[derive(Debug)]
enum Rotation {
    Clockwise,
    Counterclockwise,
}

#[derive(Debug)]
enum Direction {
    Right,
    Down,
    Left,
    Up,
}

impl Direction {
    fn turn(&self, rotation: &Rotation) -> Direction {
        use Direction::{Down, Left, Right, Up};

        match rotation {
            Rotation::Clockwise => match self {
                Right => Down,
                Down => Left,
                Left => Up,
                Up => Right,
            },
            Rotation::Counterclockwise => match self {
                Right => Up,
                Down => Right,
                Left => Down,
                Up => Left,
            },
        }
    }

    fn to_digit(&self) -> usize {
        match self {
            Direction::Right => 0,
            Direction::Down => 1,
            Direction::Left => 2,
            Direction::Up => 3,
        }
    }

    fn get_displacement(&self) -> (i32, i32) {
        match self {
            Direction::Right => (1, 0),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Up => (0, -1),
        }
    }
}

type Distance = u8;

#[derive(Debug)]
enum Instruction {
    Turn(Rotation),
    Move(Distance),
}

type Coord = usize;
type Pos = (Coord, Coord);

struct Grid {
    cells: Vec<Vec<Option<Cell>>>,
}

impl Grid {
    fn get_start(&self) -> Pos {
        for (y, row) in self.cells.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                if cell.is_some() {
                    return (x, y);
                }
            }
        }

        unreachable!()
    }

    fn get_cell(&self, position: (i32, i32)) -> Option<Cell> {
        let (x, y) = position;

        if x < 0 || y < 0 {
            return None;
        }

        if let Some(row) = self.cells.get(y as usize) {
            if let Some(cell) = row.get(x as usize) {
                return *cell;
            }
        }

        None
    }

    fn do_move1(&self, position: Pos, direction: &Direction, distance: Distance) -> Pos {
        let (dx, dy) = direction.get_displacement();

        let mut current_position = position;

        for _ in 0..distance {
            let (x, y) = current_position;
            let (nx, ny) = (x as i32 + dx, y as i32 + dy);

            match self.get_cell((nx, ny)) {
                Some(cell) => match cell {
                    Cell::Wall => return current_position,
                    Cell::Tile => {
                        current_position = (nx as usize, ny as usize);
                    }
                },
                None => match direction {
                    Direction::Right => {
                        let row = self.cells.get(y as usize).unwrap();
                        for (x, cell) in row.iter().enumerate() {
                            match cell {
                                Some(Cell::Tile) => {
                                    current_position = (x, y);
                                    break;
                                }
                                Some(Cell::Wall) => return current_position,
                                _ => continue,
                            }
                        }
                    }
                    Direction::Down => {
                        for y in 0..self.cells.len() {
                            let row = self.cells.get(y).unwrap();

                            let cell = row.get(x).unwrap();
                            match cell {
                                Some(Cell::Tile) => {
                                    current_position = (x, y);
                                    break;
                                }
                                Some(Cell::Wall) => return current_position,
                                _ => continue,
                            }
                        }
                    }
                    Direction::Left => {
                        let row = self.cells.get(y as usize).unwrap();
                        for (x, cell) in row.iter().enumerate().rev() {
                            match cell {
                                Some(Cell::Tile) => {
                                    current_position = (x, y);
                                    break;
                                }
                                Some(Cell::Wall) => return current_position,
                                _ => continue,
                            }
                        }
                    }
                    Direction::Up => {
                        for y in (0..self.cells.len()).rev() {
                            let row = self.cells.get(y).unwrap();

                            match row.get(x) {
                                Some(Some(Cell::Tile)) => {
                                    current_position = (x, y);
                                    break;
                                }
                                Some(Some(Cell::Wall)) => return current_position,
                                _ => continue,
                            }
                        }
                    }
                },
            }
        }

        current_position
    }

    fn do_move2(&self, position: Pos, direction: &mut Direction, distance: Distance) -> Pos {
        let (mut dx, mut dy) = direction.get_displacement();

        let mut current_position = position;

        for _ in 0..distance {
            let (x, y) = current_position;
            let (nx, ny) = (x as i32 + dx, y as i32 + dy);

            if let Some(cell) = self.get_cell((nx, ny)) {
                match cell {
                    Cell::Wall => return current_position,
                    Cell::Tile => {
                        current_position = (nx as usize, ny as usize);
                    }
                }
            } else {
                println!("changing face");
                let face = get_face(current_position).unwrap();
                println!("current_face: {:?}", face);
                println!("going in direction: {:?}", direction);

                let (new_face, new_direction) = face.get_new_face_and_direction(direction);
                println!("new_face should be: {:?}", new_face);

                let new_position = face.get_new_position((nx, ny));
                println!("new_face calculated: {:?}", get_face(new_position).unwrap());

                assert_eq!(get_face(new_position).unwrap(), new_face);

                println!("new_position: {:?}", new_position);
                println!("new_direction {:?}", new_direction);

                match self.get_cell((new_position.0 as i32, new_position.1 as i32)) {
                    Some(cell) => {
                        match cell {
                            Cell::Wall => return current_position,
                            Cell::Tile => {
                                *direction = new_direction;
                                (dx, dy) = direction.get_displacement();
                                current_position = new_position;
                            }
,
                        }
                    },
                    None => unreachable!(),
                }
            }
        }

        current_position
    }
}

fn part1((grid, instructions): &(Grid, Vec<Instruction>)) -> usize {
    let mut position = grid.get_start();
    let mut direction = Direction::Right;

    for instruction in instructions {
        match instruction {
            Instruction::Turn(rotation) => direction = direction.turn(rotation),
            Instruction::Move(distance) => {
                position = grid.do_move1(position, &direction, *distance);
            }
        }
    }
    1000 * (position.1 + 1) + 4 * (position.0 + 1) + direction.to_digit()
}

#[derive(Debug, PartialEq)]
enum Face {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
}

impl Face {
    // checked
    fn get_new_face_and_direction(&self, direction: &Direction) -> (Face, Direction) {
        match self {
            Face::One => match direction {
                Direction::Right => (Face::Two, Direction::Right),
                Direction::Down => (Face::Three, Direction::Down),
                Direction::Left => (Face::Four, Direction::Right),
                Direction::Up => (Face::Six, Direction::Right),
            },
            Face::Two => match direction {
                Direction::Right => (Face::Five, Direction::Left),
                Direction::Down => (Face::Three, Direction::Left),
                Direction::Left => (Face::One, Direction::Left),
                Direction::Up => (Face::Six, Direction::Up),
            },
            Face::Three => match direction {
                Direction::Right => (Face::Two, Direction::Up),
                Direction::Down => (Face::Five, Direction::Down),
                Direction::Left => (Face::Four, Direction::Down),
                Direction::Up => (Face::One, Direction::Up),
            },
            Face::Four => match direction {
                Direction::Right => (Face::Five, Direction::Right),
                Direction::Down => (Face::Six, Direction::Down),
                Direction::Left => (Face::One, Direction::Right),
                Direction::Up => (Face::Three, Direction::Right),
            },
            Face::Five => match direction {
                Direction::Right => (Face::Two, Direction::Left),
                Direction::Down => (Face::Six, Direction::Left),
                Direction::Left => (Face::Four, Direction::Left),
                Direction::Up => (Face::Three, Direction::Up),
            },
            Face::Six => match direction {
                Direction::Right => (Face::Five, Direction::Up),
                Direction::Down => (Face::Two, Direction::Down),
                Direction::Left => (Face::One, Direction::Down),
                Direction::Up => (Face::Four, Direction::Up),
            },
        }
    }

    // fn get_new_face_and_direction(&self, direction: &Direction) -> (Face, Direction) {
        // match self {
            // Face::One => match direction {
                // Direction::Right => (Face::Six, Direction::Left),
                // Direction::Down => (Face::Four, Direction::Down),
                // Direction::Left => (Face::Three, Direction::Down),
                // Direction::Up => (Face::Two, Direction::Down),
            // },
            // Face::Two => match direction {
                // Direction::Right => (Face::Three, Direction::Right),
                // Direction::Down => (Face::Five, Direction::Up),
                // Direction::Left => (Face::Six, Direction::Up),
                // Direction::Up => (Face::One, Direction::Down),
            // },
            // Face::Three => match direction {
                // Direction::Right => (Face::Four, Direction::Right),
                // Direction::Down => (Face::Five, Direction::Right),
                // Direction::Left => (Face::Two, Direction::Left),
                // Direction::Up => (Face::One, Direction::Right),
            // },
            // Face::Four => match direction {
                // Direction::Right => (Face::Six, Direction::Down),
                // Direction::Down => (Face::Five, Direction::Down),
                // Direction::Left => (Face::Three, Direction::Left),
                // Direction::Up => (Face::One, Direction::Up),
            // },
            // Face::Five => match direction {
                // Direction::Right => (Face::Six, Direction::Right),
                // Direction::Down => (Face::Two, Direction::Up),
                // Direction::Left => (Face::Three, Direction::Up),
                // Direction::Up => (Face::Four, Direction::Up),
            // },
            // Face::Six => match direction {
                // Direction::Right => (Face::One, Direction::Left),
                // Direction::Down => (Face::Two, Direction::Right),
                // Direction::Left => (Face::Five, Direction::Left),
                // Direction::Up => (Face::Four, Direction::Left),
            // },
        // }
    // }

    fn get_new_position(&self, (x, y): (i32, i32)) -> Pos {
        let new_pos =
            match self {
                Face::One => {
                    if y < ONE_START_Y {
                        (SIX_START_X, (x - ONE_START_X) + SIX_START_Y)
                    } else if x < ONE_START_X {
                        (FOUR_START_X, FOUR_END_Y - (y - ONE_START_Y))
                    } else {
                        unreachable!()
                    }
                },
                Face::Two => {
                    if y < TWO_START_Y {
                        ((x - TWO_START_X) + SIX_START_X, SIX_END_Y)
                    } else if x > TWO_END_X {
                        (FIVE_END_X, FIVE_END_Y - (y - TWO_START_Y))
                    } else if y > TWO_END_Y {
                        (THREE_END_X, (x - TWO_START_X) + THREE_START_Y)
                    } else {
                        unreachable!()
                    }
                },
                Face::Three => {
                    if x < THREE_START_X {
                        ((y - THREE_START_Y) + FOUR_START_X, FOUR_START_Y)
                    } else if x > THREE_END_X {
                        ((y - THREE_START_Y) + TWO_START_X, TWO_END_Y)
                    } else {
                        unreachable!()
                    }
                },
                Face::Four => {
                    if x < FOUR_START_X {
                        (ONE_START_X, ONE_END_Y - (y - FOUR_START_Y))
                    } else if y < FOUR_START_Y {
                        (THREE_START_X, (x - FOUR_START_X) + THREE_START_Y)
                    } else {
                        unreachable!()
                    }
                },
                Face::Five => {
                    if x > FIVE_END_X {
                        (TWO_END_X, TWO_END_Y - (y - FIVE_START_Y))
                    } else if y > FIVE_END_Y {
                        (SIX_END_X, (x - FIVE_START_X) + SIX_START_Y)
                    } else {
                        unreachable!()
                    }
                },
                Face::Six => {
                    if x < SIX_START_X {
                        ((y - SIX_START_Y) + ONE_START_X, ONE_START_Y)
                    } else if x > SIX_END_X {
                        ((y - SIX_START_Y) + FIVE_START_X, FIVE_END_Y)
                    } else if y > SIX_END_Y {
                        ((x - SIX_START_X) + TWO_START_X, TWO_START_Y)
                    } else {
                        unreachable!()
                    }
                },
            };
        (new_pos.0 as usize, new_pos.1 as usize)
    }

    // fn get_new_position(&self, (x, y): (i32, i32)) -> Pos {
        // let new_pos =
            // match self {
                // Face::One => {
                    // if x < ONE_START_X {
                        // ((y - ONE_START_Y) + THREE_START_X, THREE_START_Y)
                    // } else if x > ONE_END_X {
                        // (SIX_END_X, SIX_END_Y - (y - ONE_START_Y))
                    // } else if y < ONE_START_Y {
                        // (TWO_END_X - (x - ONE_START_X), TWO_START_Y)
                    // } else {
                        // unreachable!()
                    // }
                // },
                // Face::Two => {
                    // if x < TWO_START_X {
                        // (SIX_END_X - (y - TWO_START_Y), SIX_END_Y)
                    // } else if y < TWO_START_Y {
                        // (ONE_END_X - (x - TWO_START_X), ONE_START_Y)
                    // } else if y > TWO_END_Y {
                        // (FIVE_END_X - (x - TWO_START_X), FIVE_END_Y)
                    // } else {
                        // unreachable!()
                    // }
                // },
                // Face::Three => {
                    // if y < THREE_START_Y {
                        // (ONE_START_X, (x - THREE_START_X) + ONE_START_Y)
                    // } else if y > THREE_END_Y {
                        // (FIVE_START_X, FIVE_END_Y - (x - THREE_START_X))
                    // } else {
                        // unreachable!()
                    // }
                // },
                // Face::Four => {
                    // if x > FOUR_END_X {
                        // (SIX_END_X - (y - FOUR_START_Y), SIX_START_Y)
                    // } else {
                        // unreachable!()
                    // }
                // },
                // Face::Five => {
                    // if x < FIVE_START_X {
                        // (TWO_END_X - (y - FIVE_START_Y), THREE_END_Y)
                    // } else if y > FIVE_END_Y {
                        // (TWO_END_X - (x - FIVE_START_X), TWO_END_Y)
                    // } else {
                        // unreachable!()
                    // }
                // },
                // Face::Six => {
                    // if x > SIX_END_X {
                        // (ONE_END_X, ONE_END_Y - (y - SIX_START_Y))
                    // } else if y < SIX_START_Y {
                        // (FOUR_END_X, FOUR_END_Y - (x - SIX_START_X))
                    // } else if y > SIX_END_Y {
                        // (TWO_START_X, TWO_END_Y - (x - SIX_START_X))
                    // } else {
                        // unreachable!()
                    // }
                // },
            // };
        // (new_pos.0 as usize, new_pos.1 as usize)
    // }
}

// checked
const ONE_START_X: i32 = 50;
const ONE_END_X: i32 = 99;
const ONE_START_Y: i32 = 0;
const ONE_END_Y: i32 = 49;

const TWO_START_X: i32 = 100;
const TWO_END_X: i32 = 149;
const TWO_START_Y: i32 = 0;
const TWO_END_Y: i32 = 49;

const THREE_START_X: i32 = 50;
const THREE_END_X: i32 = 99;
const THREE_START_Y: i32 = 50;
const THREE_END_Y: i32 = 99;

const FOUR_START_X: i32 = 0;
const FOUR_END_X: i32 = 49;
const FOUR_START_Y: i32 = 100;
const FOUR_END_Y: i32 = 149;

const FIVE_START_X: i32 = 50;
const FIVE_END_X: i32 = 99;
const FIVE_START_Y: i32 = 100;
const FIVE_END_Y: i32 = 149;

const SIX_START_X: i32 = 0;
const SIX_END_X: i32 = 49;
const SIX_START_Y: i32 = 150;
const SIX_END_Y: i32 = 199;

// const ONE_START_X: i32 = 8;
// const ONE_END_X: i32 = 11;
// const ONE_START_Y: i32 = 0;
// const ONE_END_Y: i32 = 3;

// const TWO_START_X: i32 = 0;
// const TWO_END_X: i32 = 3;
// const TWO_START_Y: i32 = 4;
// const TWO_END_Y: i32 = 7;

// const THREE_START_X: i32 = 4;
// const THREE_END_X: i32 = 7;
// const THREE_START_Y: i32 = 4;
// const THREE_END_Y: i32 = 7;

// const FOUR_START_X: i32 = 8;
// const FOUR_END_X: i32 = 11;
// const FOUR_START_Y: i32 = 4;
// const FOUR_END_Y: i32 = 7;

// const FIVE_START_X: i32 = 8;
// const FIVE_END_X: i32 = 11;
// const FIVE_START_Y: i32 = 8;
// const FIVE_END_Y: i32 = 11;

// const SIX_START_X: i32 = 12;
// const SIX_END_X: i32 = 15;
// const SIX_START_Y: i32 = 8;
// const SIX_END_Y: i32 = 11;

fn get_face((x, y): Pos) -> Option<Face> {
    let (x, y) = (x as i32, y as i32);

    if (ONE_START_X..=ONE_END_X).contains(&x) && (ONE_START_Y..=ONE_END_Y).contains(&y) {
        Some(Face::One)
    } else if (TWO_START_X..=TWO_END_X).contains(&x) && (TWO_START_Y..=TWO_END_Y).contains(&y) {
        Some(Face::Two)
    } else if (THREE_START_X..=THREE_END_X).contains(&x) && (THREE_START_Y..=THREE_END_Y).contains(&y) {
        Some(Face::Three)
    } else if (FOUR_START_X..=FOUR_END_X).contains(&x) && (FOUR_START_Y..=FOUR_END_Y).contains(&y) {
        Some(Face::Four)
    } else if (FIVE_START_X..=FIVE_END_X).contains(&x) && (FIVE_START_Y..=FIVE_END_Y).contains(&y) {
        Some(Face::Five)
    } else if (SIX_START_X..=SIX_END_X).contains(&x) && (SIX_START_Y..=SIX_END_Y).contains(&y) {
        Some(Face::Six)
    } else {
        None
    }
}

fn part2((grid, instructions): &(Grid, Vec<Instruction>)) -> usize {
    let mut position = grid.get_start();
    let mut direction = Direction::Right;

    for instruction in instructions {
        println!("current_position: {:?}", position);
        println!("current_direction: {:?}", direction);
        println!("processing instruction: {:?}", instruction);

        match instruction {
            Instruction::Turn(rotation) => direction = direction.turn(rotation),
            Instruction::Move(distance) => {
                position = grid.do_move2(position, &mut direction, *distance);
            }
        }
        println!();
    }

    println!("final position: {:?}", position);

    let (final_column, final_row) = (position.0 + 1, position.1 + 1);

    1000 * final_row + 4 * final_column + direction.to_digit()
}

fn main() {
    let lines = read_input();
    let parsed = parse(&lines);

    println!("part1: {}", part1(&parsed));
    println!("part2: {}", part2(&parsed));
}
