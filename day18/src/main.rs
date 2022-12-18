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

#[derive(Debug, Eq, Hash, PartialEq)]
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

            Cube {
                x: coords[0],
                y: coords[1],
                z: coords[2],
            }
        })
        .collect()
}

fn total_surface_area(cubes: &Vec<Cube>) -> usize {
    let mut surface_area = 0;

    for cube1 in cubes {
        let mut sides_free = 6;

        for cube2 in cubes {
            if cube1 == cube2 {
                continue;
            }

            if cube1.is_adjacent(cube2) {
                sides_free -= 1;
            }
        }

        surface_area += sides_free;
    }

    surface_area
}

fn part1(cubes: &Vec<Cube>) -> usize {
    total_surface_area(cubes)
}

fn get_all_outside_air(cubes: &[Cube], x_max: Coord, y_max: Coord, z_max: Coord) -> HashSet<Pos> {
    let mut outside_air = HashSet::new();
    let mut checked = HashSet::new();

    let cubes: HashSet<Pos> = cubes
        .iter()
        .map(|cube| (cube.x, cube.y, cube.z))
        .collect();

    let mut queue = VecDeque::new();
    queue.push_back((0, 0, 0));

    let displacements = [(1, 0, 0), (-1, 0, 0), (0, 1, 0), (0, -1, 0), (0, 0, 1), (0, 0, -1)];

    while !queue.is_empty() {
        let n = queue.pop_front().unwrap();
        if checked.contains(&n) {
            continue;
        }

        checked.insert(n);
        println!("n: {:?}", n);

        if !cubes.contains(&n) {
            outside_air.insert(n);

            let (x, y, z) = n;

            for (dx, dy, dz) in displacements {
                let (nx, ny, nz)=  (x + dx, y + dy, z + dz);

                if nx < 0 || ny < 0 || nz < 0 || nx > x_max || ny > y_max || nz > z_max {
                    continue;
                }

                queue.push_back((nx, ny, nz));
            }
        }
    }

    outside_air
}

fn part2(cubes: &Vec<Cube>) -> usize {
    let x_max = cubes.iter().map(|cube| cube.x).max().unwrap();
    let y_max = cubes.iter().map(|cube| cube.y).max().unwrap();
    let z_max = cubes.iter().map(|cube| cube.z).max().unwrap();

    let cube_positions: HashSet<Pos> = cubes.iter().map(|cube| (cube.x, cube.y, cube.z)).collect();

    let mut inside_air: HashSet<Pos> = HashSet::new();
    let all_outside_air: HashSet<Pos> = get_all_outside_air(cubes, x_max, y_max, z_max);

    for z in 0..=(z_max) {
        println!("z = {}", z);
        let cubes_at_z: Vec<&Cube> = cubes.iter().filter(|cube| cube.z == z).collect();

        let mut grid = vec![vec![' '; (x_max + 2) as usize]; (y_max + 2) as usize];

        for cube in &cubes_at_z {
            grid[cube.y as usize][cube.x as usize] = '#';
        }

        for air in &all_outside_air {
            if air.2 != z {
                continue;
            }

            let (x, y, z) = (air.0, air.1, air.2);

            if x < 0 || y < 0 || z < 0 {
                continue;
            }
            if grid[y as usize][x as usize] == ' ' {
                grid[y as usize][x as usize] = '~';
            } else {
                panic!();
            }
        }

        for x in 0..x_max {
            for y in 0..y_max {
                if cube_positions.contains(&(x, y, z)) {
                    continue;
                }

                if !all_outside_air.contains(&(x, y, z)) {
                    if grid[y as usize][x as usize] == ' ' {
                        grid[y as usize][x as usize] = '-';
                    } else {
                        panic!();
                    }
                    inside_air.insert((x, y, z));
                }
            }
        }

        for row in grid {
            println!("{}", row.iter().collect::<String>());
        }
    }

    let inside_air_cubes = inside_air
        .into_iter()
        .map(|(x, y, z)| Cube { x, y, z })
        .collect();

    let cube_surface_area = total_surface_area(cubes);
    println!("cube_surface_area: {}", cube_surface_area);
    let air_surface_area = total_surface_area(&inside_air_cubes);
    println!("air_surface_area: {}", air_surface_area);

    cube_surface_area - air_surface_area
}

fn main() {
    let lines = read_input();
    let parsed = parse(&lines);

    println!("part1: {}", part1(&parsed));
    println!("part2: {}", part2(&parsed));
}
