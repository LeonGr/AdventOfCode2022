use std::{fmt::Debug, collections::HashSet};

fn read_input() -> Vec<String> {
    let input = include_str!("../input");
    input.to_string().lines().map(|s| s.to_string()).collect()
}

#[derive(Clone, Copy, Debug)]
enum Move {
    Right(usize),
    Left(usize),
    Up(usize),
    Down(usize),
}

type Pos = (i32, i32);

fn distance(pos: Pos, other: Pos) -> usize {
    let (x, y) = pos;
    let (p, q) = other;
    let (x, y) = (x as f64, y as f64);
    let (p, q) = (p as f64, q as f64);

    f64::floor(f64::sqrt((x - p).powi(2) + (y - q).powi(2))) as usize
}


#[derive(Debug, Clone, Copy)]
struct Grid {
    head: Pos,
    tail: Pos,
}

impl Grid {
    fn follow_tail(&mut self) {
        match distance(self.head, self.tail) {
            0 | 1 => (),
            2 => {
                let (x, y) = self.head;
                let (p, q) = self.tail;

                if x == p {
                    if y > q {
                        self.tail = (p, q + 1);
                    } else {
                        self.tail = (p, q - 1);
                    }
                } else if y == q {
                    if x > p {
                        self.tail = (p + 1, q);
                    } else {
                        self.tail = (p - 1, q);
                    }
                } else {
                    if x > p {
                        self.tail = (p + 1, q);
                    } else {
                        self.tail = (p - 1, q);
                    }

                    let (p, q) = self.tail;

                    if y > q {
                        self.tail = (p, q + 1);
                    } else {
                        self.tail = (p, q - 1);
                    }
                }
            }
            _ => unreachable!(),
        }
    }

    fn move_head(&mut self, m: Move, tail_positions: &mut HashSet<Pos>) {
        match m {
            Move::Right(d) => {
                for _ in 0..d {
                    let (x, y) = self.head;
                    self.head = (x + 1, y);
                    self.follow_tail();
                    tail_positions.insert(self.tail);
                }
            },
            Move::Left(d) => {
                for _ in 0..d {
                    let (x, y) = self.head;
                    self.head = (x - 1, y);
                    self.follow_tail();
                    tail_positions.insert(self.tail);
                }
            },
            Move::Up(d) => {
                for _ in 0..d {
                    let (x, y) = self.head;
                    self.head = (x, y + 1);
                    self.follow_tail();
                    tail_positions.insert(self.tail);
                }
            },
            Move::Down(d) => {
                for _ in 0..d {
                    let (x, y) = self.head;
                    self.head = (x, y - 1);
                    self.follow_tail();
                    tail_positions.insert(self.tail);
                }
            },
        }
    }
}

fn print(grid: &Grid) {
    // let mut grid_2d = vec![vec!['.'; 6]; 6];

    // let (x, y) = grid.head;
    // let (p, q) = grid.tail;

    // grid_2d[0][0] = 's';
    // grid_2d[q][p] = 'T';
    // grid_2d[y][x] = 'H';

    // for line in grid_2d.iter().rev() {
        // let as_string: String = line.iter().collect();
        // println!("{}", as_string);
    // }

    // println!();
}

fn parse(input: &[String]) -> Vec<Move> {
    input.iter()
        .map(|line| {
            let (dir, dist) = line.split_once(' ').unwrap();
            let dist = dist.parse::<usize>().unwrap();

            match dir {
                "R" => Move::Right(dist),
                "L" => Move::Left(dist),
                "U" => Move::Up(dist),
                "D" => Move::Down(dist),
                _ => unreachable!(),
            }
        })
        .collect()

}

fn part1(moves: &Vec<Move>) -> usize {
    let mut grid = Grid {
        head: (0, 0),
        tail: (0, 0),
    };

    let mut tail_positions: HashSet<Pos> = HashSet::new();

    for m in moves {
        grid.move_head(*m, &mut tail_positions);
    }

    tail_positions.len()
}

fn part2(moves: &Vec<Move>) -> usize {
    let init = Grid {
        head: (0, 0),
        tail: (0, 0),
    };

    let mut grids = vec![init; 9];

    let mut void: HashSet<Pos> = HashSet::new();
    let mut tail_positions: HashSet<Pos> = HashSet::new();
    tail_positions.insert((0, 0));

    for m in moves {
        let d = match m {
            Move::Right(d) => d,
            Move::Left(d) => d,
            Move::Up(d) => d,
            Move::Down(d) => d,
        };

        for _ in 0..*d {
            for i in 0..grids.len() {
                if i == 0 {
                    let grid = &mut grids[i];
                    let new_tail = {
                        match m {
                            Move::Right(_) => {
                                grid.move_head(Move::Right(1), &mut void);
                                grid.tail
                            },
                            Move::Left(_) => {
                                grid.move_head(Move::Left(1), &mut void);
                                grid.tail
                            },
                            Move::Up(_) => {
                                grid.move_head(Move::Up(1), &mut void);
                                grid.tail
                            },
                            Move::Down(_) => {
                                grid.move_head(Move::Down(1), &mut void);
                                grid.tail
                            },
                        }
                    };

                    let next_grid = &mut grids[i + 1];
                    next_grid.head = new_tail;
                    next_grid.follow_tail();
                } else if i < 8 {
                    let prev_grid = grids[i - 1];
                    let grid = &mut grids[i];
                    grid.head = prev_grid.tail;
                    grid.follow_tail();
                } else {
                    let prev_grid = grids[i - 1];
                    let grid = &mut grids[i];
                    grid.head = prev_grid.tail;
                    grid.follow_tail();
                    tail_positions.insert(grid.tail);
                }

            }

        }

        // print_grids(&grids);
    }

    tail_positions.len()
}

fn print_grids(grids: &[Grid]) {
    let mut min_x = 0;
    let mut max_x = 6;
    let mut min_y = 0;
    let mut max_y = 6;

    for grid in grids {
        println!("{:?}", grid);
        let (x, y) = grid.head;

        min_x = i32::min(min_x, x);
        max_x = i32::max(max_x, x);
        min_y = i32::min(min_y, y);
        max_y = i32::max(max_y, y);

        let (x, y) = grid.tail;
        min_x = i32::min(min_x, x);
        max_x = i32::max(max_x, x);
        min_y = i32::min(min_y, y);
        max_y = i32::max(max_y, y);
    }

    min_x -= 1;
    max_x += 1;
    min_y -= 1;
    max_y += 1;

    let mut grid_2d = vec![vec!['.'; (max_x - min_x) as usize]; (max_y - min_y) as usize];
    let x_offset = 0 - min_x;
    let y_offset = 0 - min_y;

    grid_2d[y_offset as usize][x_offset as usize] = 's';

    for i in (0..grids.len()).rev() {
        let grid = grids[i];

        let (x, y) = grid.head;
        let (x, y) = ((x + x_offset) as usize, (y + y_offset) as usize);
        let (p, q) = grid.tail;
        let (p, q) = ((p + x_offset) as usize, (q + y_offset) as usize);

        if i == 0 {
            grid_2d[y][x] = 'H';
            grid_2d[q][p] = *(i + 1).to_string().chars().collect::<Vec<char>>().first().unwrap();
        } else {
            grid_2d[q][p] = *(i + 1).to_string().chars().collect::<Vec<char>>().first().unwrap();
        }
    }

    for line in grid_2d.iter().rev() {
        let as_string: String = line.iter().collect();
        println!("{}", as_string);
    }

    println!();
}

fn main() {
    let lines = read_input();
    let parsed = parse(&lines);
    println!("part1: {} should be 6470", part1(&parsed));
    println!("part2: {} should be 2658", part2(&parsed));
}
