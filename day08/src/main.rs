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

fn is_visible((i, j): (usize, usize), grid: &Vec<Vec<u8>>) -> bool {
    let height = grid[i][j];

    i == 0 || j == 0  || i == grid.len() - 1 || j == grid.len() - 1 ||
    (0..j)
        .all(|x| grid[i][x] < height) ||
    ((j + 1)..grid[0].len())
        .all(|x| grid[i][x] < height) ||
    (0..i)
        .all(|y| grid[y][j] < height) ||
    ((i + 1)..grid.len())
        .all(|y| grid[y][j] < height)
}

fn part1(grid: &Vec<Vec<u8>>) -> usize {
    cartesian_product(0..grid.len(), 0..grid.len())
        .iter()
        .filter(|(i, j)| is_visible((*i, *j), grid))
        .count()
}

fn scenic_score((i, j): (usize, usize), grid: &Vec<Vec<u8>>) -> usize {
    fn score<R: Iterator<Item = usize>>(range: R, predicate: &dyn Fn(usize) -> bool) -> usize {
        let mut score = 0;
        for y in range {
            score += 1;
            if predicate(y) {
                break;
            }
        }

        score
    }

    let height = grid[i][j];

    let score1 = score((0..j).rev(), &|x| grid[i][x] >= height);
    let score2 = score((j + 1)..grid[0].len(), &|x| grid[i][x] >= height);
    let score3 = score((0..i).rev(), &|y| grid[y][j] >= height);
    let score4 = score((i + 1)..grid.len(), &|y| grid[y][j] >= height);

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
