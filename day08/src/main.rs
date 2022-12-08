use std::ops::Range;

fn read_input() -> Vec<String> {
    let input = include_str!("../input");
    input.to_string().lines().map(|s| s.to_string()).collect()
}

fn parse(input: &[String]) -> Vec<Vec<u8>> {
    input.iter()
        .map(|line| line.chars().map(|c| c.to_string().parse::<u8>().unwrap()).collect())
        .collect()

}

fn cartesian_product(x: Range<usize>, y: Range<usize>) -> Vec<(usize, usize)> {
    x
        .map(|a| y.clone().map(|b| (a, b)).collect::<Vec<_>>())
        .collect::<Vec<_>>()
        .concat()
}

fn is_visible(position: (usize, usize), grid: &Vec<Vec<u8>>) -> bool {
    let (i, j) = position;
    let height = grid[i][j];

    if i == 0 || j == 0  || i == grid.len() - 1 || j == grid.len() - 1 {
        return true;
    }

    let mut visible1 = true;
    for x in 0..j {
        if grid[i][x] >= height {
            visible1 = false;
        }
    }

    let mut visible2 = true;
    for x in (j + 1)..grid[0].len() {
        if grid[i][x] >= height {
            visible2 = false;
        }
    }

    let mut visible3 = true;
    for y in 0..i {
        if grid[y][j] >= height {
            visible3 = false;
        }
    }

    let mut visible4 = true;
    for y in (i + 1)..grid.len() {
        if grid[y][j] >= height {
            visible4 = false;
        }
    }

    visible1 || visible2 || visible3 || visible4
}

fn part1(grid: &Vec<Vec<u8>>) -> usize {
    cartesian_product(0..grid.len(), 0..grid.len())
        .iter()
        .filter(|(i, j)| is_visible((*i, *j), grid))
        .count()
}

fn scenic_score(position: (usize, usize), grid: &Vec<Vec<u8>>) -> usize {
    let (i, j) = position;
    let height = grid[i][j];

    let mut score1 = 0;
    for x in (0..j).rev() {
        score1 += 1;
        if grid[i][x] >= height {
            break
        }
    }

    let mut score2 = 0;
    for x in (j + 1)..grid[0].len() {
        score2 += 1;
        if grid[i][x] >= height {
            break
        }
    }

    let mut score3 = 0;
    for y in (0..i).rev() {
        score3 += 1;
        if grid[y][j] >= height {
            break
        }
    }

    let mut score4 = 0;
    for y in (i + 1)..grid.len() {
        score4 += 1;
        if grid[y][j] >= height {
            break
        }
    }

    score1 * score2 * score3 * score4
}

fn part2(grid: &Vec<Vec<u8>>) -> usize {
    cartesian_product(0..grid.len(), 0..grid.len())
        .iter()
        .map(|(i, j)| scenic_score((*i, *j), grid))
        .max()
        .unwrap()
}

fn main() {
    let lines = read_input();
    let grid = parse(&lines);
    println!("part1: {}", part1(&grid));
    println!("part2: {}", part2(&grid));
}
