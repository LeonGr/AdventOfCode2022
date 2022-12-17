use std::collections::HashSet;

fn read_input() -> String {
    let input = include_str!("../input");
    input.to_string()
}

#[derive(Debug)]
enum Instruction {
    Left,
    Right,
    Down,
}

fn parse(input: &str) -> Vec<Instruction> {
    input
        .trim()
        .chars()
        .map(|c| match c {
            '<' => Instruction::Left,
            '>' => Instruction::Right,
            _ => unreachable!(),
        })
        .collect()
}

type Pos = (i32, i32);

#[derive(Clone)]
struct Shape {
    dx: i32,
    dy: i32,
    height: i32,
    coords: Vec<Pos>,
}

fn get_shapes() -> Vec<Shape> {
    let minus = vec![(0, 0), (1, 0), (2, 0), (3, 0)];
    let plus = vec![(1, 0), (0, 1), (1, 1), (2, 1), (1, 2)];
    let reverse_l = vec![(0, 0), (2, 1), (2, 2), (1, 0), (2, 0)];
    let line = vec![(0, 0), (0, 1), (0, 2), (0, 3)];
    let square = vec![(0, 0), (1, 0), (0, 1), (1, 1)];

    [
        (minus, 1),
        (plus, 3),
        (reverse_l, 3),
        (line, 4),
        (square, 2),
    ]
    .iter()
    .map(|(coords, height)| Shape {
        coords: coords.clone(),
        dx: 2,
        dy: 0,
        height: *height,
    })
    .collect()
}

impl Shape {
    fn can_move(&mut self, instruction: &Instruction, spots_taken: &HashSet<Pos>) -> bool {
        let (dx, dy) = match instruction {
            Instruction::Left => (self.dx - 1, self.dy),
            Instruction::Right => (self.dx + 1, self.dy),
            Instruction::Down => (self.dx, self.dy - 1),
        };

        let mut hit_something = false;
        for (x, y) in &self.coords {
            let pos = (x + dx, y + dy);
            let (x, y) = pos;

            if !(0..7).contains(&x) || y < 0 {
                hit_something = true;
                break;
            }

            for spot in spots_taken {
                if pos == *spot {
                    hit_something = true;
                    break;
                }
            }
        }

        if hit_something {
            match instruction {
                Instruction::Left | Instruction::Right => return true,
                Instruction::Down => return false,
            }
        }

        self.dx = dx;
        self.dy = dy;

        true
    }
}

fn run_instructions(
    instructions: &[Instruction],
    target_rocks_stopped: usize,
) -> (usize, HashSet<Pos>) {
    let shapes = get_shapes();

    let mut turn = 0;

    let mut current_instruction_index = 0;

    let mut current_shape_index = 0;
    let mut current_shape = shapes[0].clone();

    let mut hightest_point = 0;
    current_shape.dy = hightest_point + current_shape.height + 3 - 1;

    let mut spots_taken: HashSet<Pos> = HashSet::new();
    let mut rocks_stopped = 0;

    loop {
        if rocks_stopped == target_rocks_stopped {
            break;
        }

        let instruction = if turn % 2 == 0 {
            let current = &instructions[current_instruction_index];

            current_instruction_index += 1;
            current_instruction_index %= instructions.len();

            current
        } else {
            &Instruction::Down
        };

        turn += 1;

        let can_move = current_shape.can_move(instruction, &spots_taken);
        if can_move {
        } else {
            rocks_stopped += 1;
            for (x, y) in current_shape.coords {
                let pos = (x + current_shape.dx, y + current_shape.dy);
                spots_taken.insert(pos);
                hightest_point = hightest_point.max(pos.1 + 1);
            }

            //speedup
            // let spots_copy = spots_taken.clone();
            // for (x, y) in spots_copy {
            // if y + 40 < hightest_point {
            // spots_taken.remove(&(x, y));
            // }
            // }

            current_shape_index += 1;
            current_shape_index %= shapes.len();
            current_shape = shapes[current_shape_index].clone();
            current_shape.dy = hightest_point + 3;
        }
    }

    (hightest_point as usize, spots_taken)
}

fn part1(instructions: &[Instruction]) -> usize {
    run_instructions(instructions, 2022).0
}

fn part2(instructions: &[Instruction]) -> usize {
    let (hightest_point, spots_taken) = run_instructions(instructions, 3800);

    let rocks_stopped_target: usize = 1_000_000_000_000;
    let (layers_before_period, layers_per_period, rocks_before_period, rocks_per_period) =
        find_periods(instructions, &spots_taken, hightest_point);
    println!("layers before period: {}", layers_before_period);
    println!("layers/period: {}", layers_per_period);
    println!("rocks before periods: {}", rocks_before_period);
    println!("rocks/period: {}", rocks_per_period);

    let rocks_needed = rocks_stopped_target - (rocks_before_period as usize);
    let periods_needed = rocks_needed / rocks_per_period;
    println!("periods_needed: {}", periods_needed);

    let rocks_left = rocks_needed % rocks_per_period;
    println!("rocks left: {}", rocks_left);
    if rocks_left == 0 {
        layers_before_period + (layers_per_period * periods_needed)
    } else {
        let x = rocks_left + rocks_before_period;
        let y = get_layers_for_rocks(instructions, x) - layers_before_period;
        layers_before_period + (layers_per_period * periods_needed) + y
    }
}

fn get_layers_for_rocks(instructions: &[Instruction], x: usize) -> usize {
    let shapes = get_shapes();

    let mut turn = 0;

    let mut current_instruction_index = 0;

    let mut current_shape_index = 0;
    let mut current_shape = shapes[0].clone();

    let mut hightest_point = 0;
    current_shape.dy = hightest_point + current_shape.height + 3 - 1;

    let mut spots_taken: HashSet<Pos> = HashSet::new();
    let mut rocks_stopped = 0;

    loop {
        if rocks_stopped == x {
            break;
        }

        let instruction = if turn % 2 == 0 {
            let current = &instructions[current_instruction_index];

            current_instruction_index += 1;
            current_instruction_index %= instructions.len();

            current
        } else {
            &Instruction::Down
        };

        turn += 1;

        let can_move = current_shape.can_move(instruction, &spots_taken);
        if can_move {
        } else {
            rocks_stopped += 1;
            for (x, y) in current_shape.coords {
                let pos = (x + current_shape.dx, y + current_shape.dy);
                spots_taken.insert(pos);
                hightest_point = hightest_point.max(pos.1 + 1);
            }

            //speedup
            let spots_copy = spots_taken.clone();
            for (x, y) in spots_copy {
                if y + 40 < hightest_point {
                    spots_taken.remove(&(x, y));
                }
            }

            current_shape_index += 1;
            current_shape_index %= shapes.len();
            current_shape = shapes[current_shape_index].clone();
            current_shape.dy = hightest_point + 3;
        }
    }

    hightest_point as usize
}

fn find_rocks_period(
    instructions: &[Instruction],
    layers_before_period: usize,
    layers_per_period: usize,
) -> (usize, usize) {
    let shapes = get_shapes();

    let mut turn = 0;

    let mut current_instruction_index = 0;

    let mut current_shape_index = 0;
    let mut current_shape = shapes[0].clone();

    let mut hightest_point = 0;
    current_shape.dy = hightest_point + current_shape.height + 3 - 1;

    let mut spots_taken: HashSet<Pos> = HashSet::new();
    let mut rocks_stopped = 0;

    let mut rocks_before_period = 0;
    let mut rocks_per_period = 0;

    loop {
        if (hightest_point as usize) == layers_before_period && rocks_before_period == 0 {
            rocks_before_period = rocks_stopped;
        }

        if (hightest_point as usize) <= (layers_before_period + layers_per_period) {
        } else {
            rocks_per_period = rocks_stopped - 1 - rocks_before_period;
            break;
        }

        let instruction = if turn % 2 == 0 {
            let current = &instructions[current_instruction_index];

            current_instruction_index += 1;
            current_instruction_index %= instructions.len();

            current
        } else {
            &Instruction::Down
        };

        turn += 1;

        let can_move = current_shape.can_move(instruction, &spots_taken);
        if can_move {
        } else {
            rocks_stopped += 1;
            for (x, y) in current_shape.coords {
                let pos = (x + current_shape.dx, y + current_shape.dy);
                spots_taken.insert(pos);
                hightest_point = hightest_point.max(pos.1 + 1);
            }

            current_shape_index += 1;
            current_shape_index %= shapes.len();
            current_shape = shapes[current_shape_index].clone();
            current_shape.dy = hightest_point + 3;
        }
    }

    (rocks_before_period, rocks_per_period)
}

fn find_periods(
    instructions: &[Instruction],
    spots_taken: &HashSet<(i32, i32)>,
    hightest_point: usize,
) -> (usize, usize, usize, usize) {
    let width = 7_usize;
    let height = hightest_point as usize;

    let mut grid = vec![vec!['.'; width]; height];

    (0..height).for_each(|y| {
        for x in 0..width {
            if spots_taken.contains(&(x as i32, y as i32)) {
                grid[y][x] = '#';
            }
        }
    });

    let window_size = 20;
    for (window_index, rows1) in grid.windows(window_size).enumerate() {
        for (other_index, rows2) in grid.windows(window_size).enumerate() {
            if window_index == other_index {
                continue;
            }

            let mut same = true;
            for i in 0..window_size {
                if rows1[i] != rows2[i] {
                    same = false;
                    break;
                }
            }

            if same {
                let layers_per_period = other_index - window_index;
                let layers_before_period = window_index;
                let (rocks_before_period, rocks_per_period) =
                    find_rocks_period(instructions, layers_before_period, layers_per_period);
                return (
                    layers_before_period,
                    layers_per_period,
                    rocks_before_period,
                    rocks_per_period,
                );
            }
        }
    }

    unreachable!()
}

fn main() {
    let input = read_input();
    let parsed = parse(&input);

    println!("part1: {}", part1(&parsed));
    println!("part2: {}", part2(&parsed));
}
