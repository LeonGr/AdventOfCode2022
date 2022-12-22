use nom::{branch::alt, character::complete::one_of, character::complete::u8, combinator::map, multi::many0, IResult};

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
            let cell =
                match c {
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
    println!("{}", instruction_line);

    let instructions = match instructions(instruction_line) {
        Ok((_, instructions)) => instructions,
        Err(_) => unreachable!(),
    };

    let grid = Grid {
        cells,
    };

    (grid, instructions)
}

fn turn(i: &str) -> IResult<&str, Rotation> {
    let (matched, direction) = one_of("RL")(i)?;

    let rotation =
        match direction {
            'R' => Rotation::Clockwise,
            'L' => Rotation::Counterclockwise,
            _ => unreachable!(),
        };

    Ok((matched, rotation))
}

fn instructions(input: &str) -> IResult<&str, Vec<Instruction>> {
    use Instruction::{Turn, Move};

    many0(alt((map(turn, Turn), map(u8, Move))))(input)
}

#[derive(Debug, Clone, Copy)]
enum Cell {
    Wall,
    Tile,
}

struct Grid {
    cells: Vec<Vec<Option<Cell>>>,
}

type Coord = usize;
type Pos = (Coord, Coord);

#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss, clippy::cast_possible_wrap)]
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
                *cell
            } else {
                None
            }
        } else {
            None
        }
    }

    fn do_move(&self, position: Pos, direction: &Direction, distance: Distance) -> Pos {

        let (dx, dy) = match direction {
            Direction::Right => (1, 0),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Up => (0, -1),
        };

        let mut current_position = position;

        for _ in 0..distance {
            let (x, y) = current_position;
            let (nx, ny) = (x as i32 + dx, y as i32 + dy);

            match self.get_cell((nx, ny)) {
                Some(cell) => match cell {
                    Cell::Wall => return current_position,
                    Cell::Tile => {
                        current_position = (nx as usize, ny as usize);
                    },
                },
                None => {
                    match direction {
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
                        },
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
                        },
                        Direction::Left => {
                            let row = self.cells.get(y as usize).unwrap();
                            for (x, cell) in row.iter().enumerate().rev() {
                                match cell {
                                    Some(Cell::Tile) => {
                                        current_position = (x, y);
                                        break;
                                    },
                                    Some(Cell::Wall) => return current_position,
                                    _ => continue,
                                }
                            }
                        },
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
                        },
                    }
                },
            }
        }

        current_position
    }
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
}

type Distance = u8;

#[derive(Debug)]
enum Instruction {
    Turn(Rotation),
    Move(Distance),
}

fn part1((grid, instructions): &(Grid, Vec<Instruction>)) -> usize {
    println!("{:?}", instructions);

    let mut position = grid.get_start();
    println!("start: {:?}", position);

    let mut direction = Direction::Right;

    for instruction in instructions {
        println!("current_position: {:?}", position);
        println!("current_direction: {:?}", direction);
        println!("processing instruction: {:?}", instruction);

        match instruction {
            Instruction::Turn(rotation) => direction = direction.turn(rotation),
            Instruction::Move(distance) => position = grid.do_move(position, &direction, *distance),
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
    // println!("part2: {}", part2(&parsed));
}
