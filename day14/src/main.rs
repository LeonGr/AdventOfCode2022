use std::fmt::Debug;

fn read_input() -> Vec<String> {
    let input = include_str!("../input");
    input.to_string().lines().map(std::string::ToString::to_string).collect()
}

#[derive(Clone, Debug)]
enum Cell {
    Sand,
    Air,
    Rock
}

type Coord = usize;

#[derive(Clone)]
struct Grid {
    cells: Vec<Vec<Cell>>,
    x_min: Coord,
    x_max: Coord,
    y_min: Coord,
    y_max: Coord,
}

impl ToString for Grid {
    fn to_string(&self) -> String {
        let cells = &self.cells;
        let mut output = String::new();

        for row in cells.iter() {
            for cell in row.iter() {
                output +=
                    match cell {
                        Cell::Sand => "o",
                        Cell::Air => ".",
                        Cell::Rock => "#",
                    }
            }

            output += "\n";
        }

        output
    }
}

impl Grid {
    fn to_relative(&self, x: Coord, y: Coord) -> (Coord, Coord) {
        (x - self.x_min, y - self.y_min)
    }

    fn to_absolute(&self, x: Coord, y: Coord) -> (Coord, Coord) {
        (x + self.x_min, y + self.y_min)
    }

    fn get_cell(&self, x: Coord, y: Coord) -> Option<Cell> {
        let (tx, ty) = self.to_absolute(x, y);

        if tx < self.x_min || tx > self.x_max || ty < self.y_min || ty > self.y_max {
            None
        } else {
            Some(self.cells[y][x].clone())
        }
    }
}

fn parse(lines: &[String]) -> Grid {
    let paths: Vec<Vec<(Coord, Coord)>> =
        lines.iter()
        .map(|line| {
            line.split(" -> ")
                .map(|segment| {
                    // println!("segment: '{}'", segment);
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

    println!("{:?}", end_points);

    let (x_min, x_max, y_min, y_max) = end_points;
    let dx = (x_max - x_min + 1) as usize;
    let dy = (y_max - y_min + 2) as usize;

    let mut cells = vec![vec![Cell::Air; dx]; dy];

    for path in &paths {
            path.windows(2)
                .for_each(|segment| {
                    assert!(segment.len() == 2);

                    println!("{:?}", segment);

                    let first = segment[0];
                    let second = segment[1];
                    let sx = Coord::min(first.0, second.0);
                    let sy = Coord::min(first.1, second.1);
                    let ex = Coord::max(first.0, second.0);
                    let ey = Coord::max(first.1, second.1);

                    if sx == ex {
                        let x = sx;
                        println!("x's equal, x: {}", (x - x_min) as usize);
                        (sy..=ey).for_each(|y| {
                            println!("y: {}", y);
                            cells[(y - y_min) as usize][(x - x_min) as usize] = Cell::Rock;
                        });
                    } else if sy == ey {
                        let y = sy;
                        println!("y's equal, y: {}", (y - y_min) as usize);
                        (sx..=ex).for_each(|x| {
                            println!("x: {}", x);
                            cells[(y - y_min) as usize][(x - x_min) as usize] = Cell::Rock;
                        });
                    } else {
                        unreachable!()
                    }
                });
        }

    Grid { cells, x_min, x_max, y_min, y_max }
}

fn move_sand(grid: &mut Grid, coords: (Coord, Coord)) -> (Coord, Coord) {
    let (sand_x, sand_y) = (coords.0 as i32, coords.1 as i32);

    println!("{:?}", coords);

    let displacements = vec![(sand_x, sand_y + 1), (sand_x - 1, sand_y + 1), (sand_x + 1, sand_y + 1)];

    for (x, y) in displacements {
        if x < 0 || y < 0 {
            return (Coord::MAX, Coord::MAX);
        }

        let x = x as usize;
        let y = y as usize;

        if let Some(cell) = grid.get_cell(x, y) {
            match cell {
                Cell::Air => {
                    grid.cells[sand_y as usize][sand_x as usize] = Cell::Air;
                    grid.cells[y][x] = Cell::Sand;
                    return (x, y);
                },
                Cell::Sand | Cell::Rock => continue,
            }
        } else {
            return (Coord::MAX, Coord::MAX);
        }
    }

    coords
}

fn add_sand(grid: &mut Grid) -> bool {
    let mut sand_pos = grid.to_relative(500, 0);
    grid.cells[sand_pos.1][sand_pos.0] = Cell::Sand;

    loop {
        let new_sand_pos = move_sand(grid, sand_pos);
        if new_sand_pos == (Coord::MAX, Coord::MAX) {
            return false;
        }
        if new_sand_pos == sand_pos {
            break;
        }

        sand_pos = new_sand_pos;
    }

    return true;
}

fn part1(grid: &Grid) -> usize {
    let mut grid = grid.clone();

    let mut i = 0;
    while add_sand(&mut grid) {
        i += 1;
        println!("i: {}\n{}", i, grid.to_string());
    }


    i
}

fn main() {
    let lines = read_input();
    let grid = parse(&lines);

    println!("part1: {}", part1(&grid));
    // println!("part2: {}", part2(&parsed));
}
