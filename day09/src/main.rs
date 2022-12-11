use std::collections::HashSet;

fn read_input() -> Vec<String> {
    let input = include_str!("../input");
    input.to_string().lines().map(std::string::ToString::to_string).collect()
}

#[derive(Clone, Copy, Debug)]
enum Direction {
    Right,
    Left,
    Up,
    Down,
}

struct Move {
    dir: Direction,
    dist: usize,
}

type Pos = (i32, i32);

fn chebyshev_distance((x, y): Pos, (p, q): Pos) -> usize {
    usize::max(i32::abs(x - p) as usize, i32::abs(y - q) as usize)
}

#[derive(Debug, Clone, Copy, Default)]
struct Grid {
    head: Pos,
    tail: Pos,
}

impl Grid {
    fn follow_tail(&mut self) {
        match chebyshev_distance(self.head, self.tail) {
            0 | 1 => (),
            2 => {
                let (x, y) = self.head;
                let (p, q) = self.tail;
                self.tail = (p + f32::round((x - p) as f32 / 2.0) as i32, q + f32::round((y - q) as f32 / 2.0) as i32);
            }
            _ => unreachable!(),
        }
    }

    fn move_head(&mut self, dir: Direction) {
        let (x, y) = self.head;

        self.head = match dir {
            Direction::Right => (x + 1, y),
            Direction::Left => (x - 1, y),
            Direction::Up => (x, y + 1),
            Direction::Down => (x, y - 1),
        };

        self.follow_tail();
    }
}

fn parse(input: &[String]) -> Vec<Move> {
    input
        .iter()
        .map(|line| {
            let (dir, dist) = line.split_once(' ').unwrap();

            let dir = match dir {
                "R" => Direction::Right,
                "L" => Direction::Left,
                "U" => Direction::Up,
                "D" => Direction::Down,
                _ => unreachable!(),
            };

            Move { dir, dist: dist.parse::<usize>().unwrap() }
        })
        .collect()
}

fn part1(moves: &[Move]) -> usize {
    let mut grid = Grid::default();

    moves.iter()
        .fold(HashSet::new(), |mut tail_positions, m| {
            (0..m.dist).for_each(|_| {
                grid.move_head(m.dir);
                tail_positions.insert(grid.tail);
            });

            tail_positions
        }).len()
}

fn part2(moves: &[Move]) -> usize {
    let mut grids = vec![Grid::default(); 9];

    moves.iter()
        .fold(HashSet::new(), |mut tail_positions, m| {
            (0..m.dist).for_each(|_| {
                for i in 0..grids.len() {
                    if i == 0 {
                        let grid = &mut grids[i];
                        grid.move_head(m.dir);
                        let new_tail = grid.tail;
                        let next_grid = &mut grids[i + 1];
                        next_grid.head = new_tail;
                        next_grid.follow_tail();
                    } else {
                        let prev_grid = grids[i - 1];
                        let grid = &mut grids[i];
                        grid.head = prev_grid.tail;
                        grid.follow_tail();

                        if i == 8 {
                            tail_positions.insert(grid.tail);
                        }
                    }
                }
            });

            tail_positions
        }).len()
}

fn main() {
    let lines = read_input();
    let parsed = parse(&lines);
    println!("part1: {}", part1(&parsed));
    println!("part2: {}", part2(&parsed));
}
