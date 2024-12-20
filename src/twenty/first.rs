use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet, VecDeque},
    fs::File,
    io::{BufRead, BufReader},
    usize,
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
    let fastest = traverse(matrix);

    let mut result = 0;
    for i in 1..matrix.len() - 1 {
        for j in 1..matrix[0].len() - 1 {
            if matrix[i][j] == '#' {
                let mut min_val = i32::MAX;
                let mut max_val = -1;

                for d in 0..4 {
                    let n = next_point(i, j, d);

                    if fastest[n.0][n.1] == i32::MAX {
                        continue;
                    }

                    min_val = min_val.min(fastest[n.0][n.1]);
                    max_val = max_val.max(fastest[n.0][n.1]);
                }

                // 100 threshold + 2 extra steps to step over cheat
                if max_val - min_val >= 102 {
                    result += 1;
                }
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
