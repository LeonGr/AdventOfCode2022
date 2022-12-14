use std::collections::HashMap;

fn read_input() -> Vec<String> {
    let input = include_str!("../input");
    input.to_string().lines().map(std::string::ToString::to_string).collect()
}

#[derive(Clone, Copy)]
enum Cell {
    Sand,
    Air,
    Rock
}

type Coord = i32;

#[derive(Clone)]
struct Grid {
    cells: HashMap<(Coord, Coord), Cell>,
    x_min: Coord,
    x_max: Coord,
    y_min: Coord,
    y_max: Coord,
    floor: bool,
}

impl Grid {
    fn get_cell(&self, x: Coord, y: Coord) -> Option<Cell> {
        match self.cells.get(&(x, y)) {
            Some(cell) => Some(*cell),
            None => {
                if !self.floor {
                    if x < self.x_min || x > self.x_max || y < self.y_min || y > self.y_max {
                        None
                    } else {
                        Some(Cell::Air)
                    }
                } else if y == self.y_max + 2 {
                    Some(Cell::Rock)
                } else {
                    Some(Cell::Air)
                }
            },
        }
    }
}

fn parse(lines: &[String]) -> Grid {
    let paths: Vec<Vec<(Coord, Coord)>> =
        lines.iter()
        .map(|line| {
            line.split(" -> ")
                .map(|segment| {
                    let (x, y) = segment.split_once(',').unwrap();
                    (x.parse().unwrap(), y.parse().unwrap())
                })
                .collect()
        })
        .collect();

    let end_points =
        paths.iter()
            .fold((Coord::MAX, 0, 0, 0), |acc, segments| {
                let (mut x_min, mut x_max, mut y_min, mut y_max) = acc;
                for segment in segments.iter() {
                        let (x, y) = segment;

                        x_min = Coord::min(*x, x_min);
                        x_max = Coord::max(*x, x_max);
                        y_min = Coord::min(*y, y_min);
                        y_max = Coord::max(*y, y_max);
                    }

                (x_min, x_max, y_min, y_max)
            });

    let (x_min, x_max, y_min, y_max) = end_points;

    let mut cells = HashMap::new();

    for path in &paths {
            path.windows(2)
                .for_each(|segment| {
                    let first = segment[0];
                    let second = segment[1];
                    let sx = Coord::min(first.0, second.0);
                    let sy = Coord::min(first.1, second.1);
                    let ex = Coord::max(first.0, second.0);
                    let ey = Coord::max(first.1, second.1);

                    if sx == ex {
                        let x = sx;
                        (sy..=ey).for_each(|y| {
                            // cells.insert(((x - x_min), (y - y_min)), Cell::Rock);
                            cells.insert((x, y), Cell::Rock);
                        });
                    } else if sy == ey {
                        let y = sy;
                        (sx..=ex).for_each(|x| {
                            // cells.insert(((x - x_min), (y - y_min)), Cell::Rock);
                            cells.insert((x, y), Cell::Rock);
                        });
                    }
                });
        }

    Grid { cells, x_min, x_max, y_min, y_max, floor: false }
}

enum MoveResult {
    Moved((Coord, Coord)),
    Stopped,
    Full,
    Abyss,
}

fn move_sand(grid: &mut Grid, coords: (Coord, Coord)) -> MoveResult {
    let (sand_x, sand_y) = (coords.0 as i32, coords.1 as i32);

    let displacements = [(sand_x, sand_y + 1), (sand_x - 1, sand_y + 1), (sand_x + 1, sand_y + 1)];

    // for (x, y) in displacements {
        // if !grid.floor && (x < 0 || y < 0) {
            // return MoveResult::Abyss;
        // }

        // if let Some(cell) = grid.get_cell(x, y) {
            // match cell {
                // Cell::Air => {
                    // grid.cells.remove(&(sand_x, sand_y));
                    // grid.cells.insert((x, y), Cell::Sand);
                    // return MoveResult::Moved((x, y));
                // },
                // Cell::Sand | Cell::Rock => continue,
            // }
        // }

        // if grid.floor {
            // if coords == (500, 0) {
                // return MoveResult::Full;
            // }

            // return MoveResult::Stopped;
        // }

        // return MoveResult::Abyss;
    // }

    // MoveResult::Stopped

    if grid.floor {
        for (x, y) in displacements {
            if let Some(cell) = grid.get_cell(x, y) {
                match cell {
                    Cell::Air => {
                        grid.cells.remove(&(sand_x, sand_y));
                        grid.cells.insert((x, y), Cell::Sand);
                        return MoveResult::Moved((x, y));
                    },
                    Cell::Sand | Cell::Rock => continue,
                }
            }
        }

        if coords == (500, 0) {
            return MoveResult::Full;
        }

        return MoveResult::Stopped;
    }

    for (x, y) in displacements {
        if x < 0 || y < 0 {
            return MoveResult::Abyss;
        }

        if let Some(cell) = grid.get_cell(x, y) {
            match cell {
                Cell::Air => {
                    grid.cells.remove(&(sand_x, sand_y));
                    grid.cells.insert((x, y), Cell::Sand);
                    return MoveResult::Moved((x, y));
                },
                Cell::Sand | Cell::Rock => continue,
            }
        }

        return MoveResult::Abyss;
    }

    MoveResult::Stopped
}

fn add_sand(grid: &mut Grid) -> bool {
    let sand_start_pos = (500, 0);
    grid.cells.insert(sand_start_pos, Cell::Sand);

    let mut sand_pos = sand_start_pos;

    loop {
        match move_sand(grid, sand_pos) {
            MoveResult::Moved(pos) => sand_pos = pos,
            MoveResult::Stopped => break,
            MoveResult::Full | MoveResult::Abyss => return false,
        }
    }

    true
}

fn part1(grid: &Grid) -> usize {
    let mut grid = grid.clone();

    let mut i = 0;
    while add_sand(&mut grid) {
        i += 1;
    }

    i
}

fn part2(grid: &Grid) -> usize {
    let mut grid = grid.clone();
    grid.floor = true;

    let mut i = 0;
    while add_sand(&mut grid) {
        i += 1;
    }

    i + 1
}

fn main() {
    let lines = read_input();
    let grid = parse(&lines);

    println!("part1: {}", part1(&grid));
    println!("part2: {}", part2(&grid));
}
