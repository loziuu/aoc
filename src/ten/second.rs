use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn execute(file_name: &str) -> i32 {
    let matrix: Vec<Vec<u32>> = BufReader::new(File::open(file_name).unwrap())
        .lines()
        .map_while(Result::ok)
        .map(|it| it.chars().map(|it| it.to_digit(10).unwrap()).collect())
        .collect();

    trails(&matrix)
}

fn trails(matrix: &[Vec<u32>]) -> i32 {
    let m = matrix.len();
    let n = matrix[0].len();
    let mut dp = vec![vec![-1; n]; m];

    let mut result = 0;
    for i in 0..m {
        for j in 0..n {
            result += count_trails(0, i as i32, j as i32, m as i32, n as i32, matrix, &mut dp);
        }
    }

    result
}

fn count_trails(
    expected_value: u32,
    i: i32,
    j: i32,
    m: i32,
    n: i32,
    matrix: &[Vec<u32>],
    visited: &mut Vec<Vec<i32>>,
) -> i32 {
    if i < 0 || j < 0 || i >= m || j >= n {
        return 0;
    }

    if matrix[i as usize][j as usize] != expected_value {
        return 0;
    }

    let cached = visited[i as usize][j as usize];
    if cached != -1 {
        return cached;
    }

    if expected_value == 9 {
        return 1;
    }

    let next = expected_value + 1;
    let mut res = 0;

    visited[i as usize][j as usize] = 0;
    res += count_trails(next, i + 1, j, m, n, matrix, visited);
    res += count_trails(next, i - 1, j, m, n, matrix, visited);
    res += count_trails(next, i, j + 1, m, n, matrix, visited);
    res += count_trails(next, i, j - 1, m, n, matrix, visited);
    visited[i as usize][j as usize] = res;

    res
}

#[cfg(test)]
mod tests {
    use crate::ten::second::trails;

    #[test]
    fn first() {
        let matrix = vec![
            vec![8, 9, 0, 1, 0, 1, 2, 3],
            vec![7, 8, 1, 2, 1, 8, 7, 4],
            vec![8, 7, 4, 3, 0, 9, 6, 5],
            vec![9, 6, 5, 4, 9, 8, 7, 4],
            vec![4, 5, 6, 7, 8, 9, 0, 3],
            vec![3, 2, 0, 1, 9, 0, 1, 2],
            vec![0, 1, 3, 2, 9, 8, 0, 1],
            vec![1, 0, 4, 5, 6, 7, 3, 2],
        ];

        assert_eq!(81, trails(&matrix));
    }
}
