use std::collections::HashSet;

// extern crate pathfinding;

fn read_input() -> Vec<String> {
    let input = include_str!("../input");
    input.lines().map(std::string::ToString::to_string).collect()
}

// type Pos = (usize, usize);
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Pos(usize, usize);

type Height = u8;
type HeightMap = Vec<Vec<(char, Height)>>;
struct State {
    map: HeightMap,
    start: Pos,
    end: Pos,
}

fn parse(input: &[String]) -> State {
    let mut start = None;
    let mut end = None;

    let map =
        input.iter()
            .enumerate()
            .map(|(row, line)| line.chars().enumerate().map(|(col, c)| {
                match c {
                    'S' => {
                        // start = Some((col, row));
                        start = Some(Pos(col, row));
                        ('a', 0)
                    }
                    'E' => {
                        // end = Some((col, row));
                        end = Some(Pos(col, row));
                        ('z', b'z' - b'a')
                    }
                    c => (c, (c as Height) - b'a')
                }
            }).collect()).collect();

    State {
        map,
        start: start.unwrap(),
        end: end.unwrap()
    }
}

impl State {
    fn get_pos(&self, pos: &Pos, displacement: (i32, i32)) -> Option<Pos> {
        // let (x, y) = pos;
        let (x, y) = (pos.0, pos.1);
        let (dx, dy) = displacement;

        let p = (x as i32) + dx;
        let q = (y as i32) + dy;

        if p < 0 || q < 0 || (q as usize) >= self.map.len() || (p as usize) >= self.map.get(0).unwrap().len() {
            return None;
        }

        // println!("{:?}", (q, p));

        // Some(((p as usize), (q as usize)))
        Some(Pos(p as usize, q as usize))
    }

    fn get_height(&self, pos: &Pos) -> Height {
        let (x, y) = (pos.0 as usize, pos.1 as usize);
        // println!("get_height: {:?}", (x, y));
        self.map.get(y).unwrap().get(x).unwrap().1
    }

    fn reachable(&self, pos: &Pos) -> HashSet<(Pos, usize)> {
        let displacements = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];

        let source_height = self.get_height(pos);

        displacements.iter()
            .filter_map(|displacement| {
                match self.get_pos(pos, *displacement) {
                    Some(pos) => {
                        let target_height = self.get_height(&pos);
                        if source_height + 1 >= target_height {
                            Some((pos, 1))
                        } else {
                            None
                        }
                    },
                    None => None,
                }
            })
            .collect()
    }

}

// fn backtrack(state: &State, pos: Pos, target: Pos, seen: &Vec<Pos>) -> Option<Vec<Pos>> {
    // let (path, len) = state.reachable(pos).iter()
        // .filter_map(|reach| {
            // if seen.contains(reach) {
                // return None;
            // }

            // let mut path = seen.clone();
            // path.push(*reach);

            // if *reach == target {
                // let len = path.len();
                // return Some((path, len));
            // }

            // if let Some(new_path) = backtrack(state, *reach, target, &path) {
                // let len = new_path.len();
                // return Some((new_path, len));
            // }

            // None
        // })
        // .fold((vec![], 0), |(acc, acc_len), (path, path_len)| {
            // if acc_len == 0 || path_len < acc_len {
                // (path, path_len)
            // } else {
                // (acc, acc_len)
            // }
        // });

    // if len > 0 {
        // return Some(path);
    // }

    // None
// }

impl Pos {
  fn distance(&self, other: &Pos) -> usize {
    (self.0.abs_diff(other.0) + self.1.abs_diff(other.1)) as usize
  }
}

fn part1(state: &State) -> usize {
    // println!("{:?}", state.reachable((0, 0)));

    // let path = vec![];

    // match backtrack(state, state.start, state.end, &path) {
        // Some(path) => println!("{:?}: {}", path, path.len()),
        // None => println!("None"),
    // }
    let result = pathfinding::prelude::astar(&state.start, |p| state.reachable(p), |p| p.distance(&state.end), |p| *p == state.end);
    // println!("{:?}", );

    result.unwrap().1
}

fn part2(state: &State) -> usize {
    let starts: HashSet<Pos> = state.map.iter().enumerate()
        .flat_map(|(row_index, row)| {
            row.iter().enumerate()
                .filter_map(|(col_index, (c, _))| {
                    if *c == 'a' {
                        Some(Pos(col_index, row_index))
                    } else {
                        None
                    }
                }).collect::<HashSet<Pos>>()
        }).collect();

    starts.iter()
        .filter_map(|start| {
            pathfinding::prelude::astar(start, |p| state.reachable(p), |p| p.distance(&state.end), |p| *p == state.end)
        })
        .map(|x| x.1)
        .min().unwrap()
}

fn main() {
    let input = read_input();
    let state = parse(&input);

    println!("part1: {}", part1(&state));
    println!("part2: {}", part2(&state));
}
