#![allow(clippy::cast_sign_loss)]

use std::collections::{HashSet, VecDeque};

fn read_input() -> Vec<String> {
    let input = include_str!("../input");
    input
        .lines()
        .map(std::string::ToString::to_string)
        .collect()
}

type Coord = i8;
type Pos = (Coord, Coord, Coord);

#[derive(Eq, Hash, PartialEq)]
struct Cube {
    x: Coord,
    y: Coord,
    z: Coord,
}
impl Cube {
    fn is_adjacent(&self, other: &Cube) -> bool {
        (self.x == other.x && self.y == other.y && (self.z - other.z).abs() == 1)
            || (self.x == other.x && (self.y - other.y).abs() == 1 && self.z == other.z)
            || ((self.x - other.x).abs() == 1 && self.y == other.y && self.z == other.z)
    }
}

fn parse(lines: &[String]) -> Vec<Cube> {
    lines
        .iter()
        .map(|line| {
            let coords: Vec<Coord> = line
                .split(',')
                .map(|coord| coord.parse().unwrap())
                .collect();

            Cube { x: coords[0], y: coords[1], z: coords[2] }
        })
        .collect()
}

fn total_surface_area(cubes: &[Cube]) -> usize {
    cubes.iter().fold(0, |acc, cube1| {
        acc + cubes.iter().fold(6, |acc, cube2| {
            if cube1.is_adjacent(cube2) {
                acc - 1
            } else {
                acc
            }
        })
    })
}

fn part1(cubes: &[Cube]) -> usize {
    total_surface_area(cubes)
}

fn get_all_outside_air(cubes: &[Cube], x_max: Coord, y_max: Coord, z_max: Coord) -> HashSet<Pos> {
    let mut outside_air = HashSet::new();
    let mut checked = HashSet::new();

    let cubes_positions: HashSet<Pos> = cubes.iter().map(|cube| (cube.x, cube.y, cube.z)).collect();

    let mut queue = VecDeque::from([(0, 0, 0)]);

    let displacements = [ (1, 0, 0), (-1, 0, 0), (0, 1, 0), (0, -1, 0), (0, 0, 1), (0, 0, -1) ];

    while !queue.is_empty() {
        let n @ (x, y, z) = queue.pop_front().unwrap();
        if checked.contains(&n) {
            continue;
        }

        checked.insert(n);

        if !cubes_positions.contains(&n) {
            outside_air.insert(n);

            for (dx, dy, dz) in displacements {
                let (nx, ny, nz) = (x + dx, y + dy, z + dz);

                if nx < 0 || ny < 0 || nz < 0 || nx > x_max || ny > y_max || nz > z_max {
                    continue;
                }

                queue.push_back((nx, ny, nz));
            }
        }
    }

    outside_air
}

fn part2(cubes: &[Cube]) -> usize {
    let (x_max, y_max, z_max) = cubes.iter().fold((0, 0, 0), |(x, y, z), cube| {
        (x.max(cube.x), y.max(cube.y), z.max(cube.z))
    });

    let cube_positions: HashSet<Pos> = cubes.iter().map(|cube| (cube.x, cube.y, cube.z)).collect();
    let outside_air_positions: HashSet<Pos> = get_all_outside_air(cubes, x_max, y_max, z_max);

    let mut inside_air: Vec<Cube> = vec![];
    for x in 0..x_max {
        for y in 0..y_max {
            for z in 0..z_max {
                if cube_positions.contains(&(x, y, z)) {
                    continue;
                }

                if !outside_air_positions.contains(&(x, y, z)) {
                    inside_air.push(Cube { x, y, z });
                }
            }
        }
    }

    total_surface_area(cubes) - total_surface_area(&inside_air)
}

fn main() {
    let lines = read_input();
    let parsed = parse(&lines);

    println!("part1: {}", part1(&parsed));
    println!("part2: {}", part2(&parsed));
}
