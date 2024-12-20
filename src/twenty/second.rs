use std::{
    collections::VecDeque,
    fs::File,
    io::{BufRead, BufReader},
};

const DIR: [(i32, i32); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

pub fn execute(file_path: &str) -> usize {
    let mut matrix: Vec<Vec<char>> = BufReader::new(File::open(file_path).unwrap())
        .lines()
        .map_while(Result::ok)
        .map(|lines| lines.chars().collect())
        .collect();

    helper(&mut matrix)
}

fn helper(matrix: &mut [Vec<char>]) -> usize {
    let dp = traverse(matrix);
    let max_picoseconds = 20;

    let mut result = 0;
    for i in 1..matrix.len() - 1 {
        for j in 1..matrix[0].len() - 1 {
            if dp[i][j] != i32::MAX {
                result += calc_cheat(i as i32, j as i32, &dp, max_picoseconds);
            }
        }
    }

    result
}

fn calc_cheat(start_i: i32, start_j: i32, dp: &[Vec<i32>], picoseconds: i32) -> usize {
    let m = dp.len() as i32;
    let n = dp[0].len() as i32;

    let mut result = 0;

    let x_range = (start_i - picoseconds).max(0)..(start_i + picoseconds + 1).min(m);

    for x in x_range {
        let y_base = picoseconds - x.abs_diff(start_i) as i32;
        let y_range = (start_j - y_base).max(0)..(start_j + y_base + 1).min(n);

        for y in y_range {
            let curr = dp[x as usize][y as usize];
            if curr != i32::MAX
                // picoseconds is wrong. It should be calculated
                && curr - dp[start_i as usize][start_j as usize] >= 100 + (start_i.abs_diff(x) as i32) + (start_j.abs_diff(y) as i32)
            {
                result += 1;
            }
        }
    }

    result
}

fn end_cords(matrix: &[Vec<char>]) -> (usize, usize) {
    for i in 0..matrix.len() {
        for j in 0..matrix[0].len() {
            if matrix[i][j] == 'E' {
                return (i, j);
            }
        }
    }
    (0, 0)
}

fn traverse(matrix: &[Vec<char>]) -> Vec<Vec<i32>> {
    let end_cords = end_cords(matrix);
    let mut dp = vec![vec![i32::MAX; matrix[0].len()]; matrix.len()];

    let mut vec = VecDeque::new();
    vec.push_back((0, end_cords.0, end_cords.1));
    dp[end_cords.0][end_cords.1] = 0;

    while !vec.is_empty() {
        let size = vec.len();

        for _ in 0..size {
            let pop = vec.pop_front().unwrap();

            if matrix[pop.1][pop.2] == '#' {
                continue;
            }

            if dp[pop.1][pop.2] < pop.0 {
                continue;
            }
            dp[pop.1][pop.2] = pop.0;

            for dir in 0..4 {
                let next = next_point(pop.1, pop.2, dir);
                vec.push_back((pop.0 + 1, next.0, next.1));
            }
        }
    }

    dp
}

fn next_point(x: usize, y: usize, dir: usize) -> (usize, usize) {
    (
        (x as i32 + DIR[dir].0) as usize,
        (y as i32 + DIR[dir].1) as usize,
    )
}

#[cfg(test)]
mod tests {
    use super::execute;

    #[test]
    fn first() {
        let result = execute("inputs/twenty_test");

        assert_eq!(2, result);
    }
}
