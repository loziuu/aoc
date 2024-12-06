use std::{
    collections::VecDeque,
    fs::File,
    io::{BufRead, BufReader},
};

const WORD: [char; 4] = ['X', 'M', 'A', 'S'];

const DIR: [(i32, i32); 8] = [
    (1, 0),
    (0, 1),
    (1, 1),
    (1, -1),
    (-1, 1),
    (-1, 0),
    (0, -1),
    (-1, -1),
];

pub fn execute(file: File) -> i32 {
    let matrix: Vec<Vec<char>> = BufReader::new(file)
        .lines()
        .map_while(Result::ok)
        .map(|line| line.chars().collect())
        .collect();

    calc(&matrix)
}

fn calc(matrix: &[Vec<char>]) -> i32 {
    let n = matrix.len();
    let m = matrix[0].len();

    let mut result = 0;
    for i in 0..n {
        for j in 0..m {
            if matrix[i][j] == WORD[0] {
                result += dfs_fwd(i as i32, j as i32, matrix);
            }
        }
    }

    result
}

fn dfs_fwd(i: i32, j: i32, matrix: &[Vec<char>]) -> i32 {
    let mut q = VecDeque::new();

    for d in DIR {
        q.push_back((i, j, d));
    }

    let mut ch = 0;
    let mut res = 0;

    while !q.is_empty() {
        let size = q.len();

        for _ in 0..size {
            let pop = q.pop_front().unwrap();

            if matrix[pop.0 as usize][pop.1 as usize] != WORD[ch] {
                continue;
            }

            if ch == 3 {
                res += 1;
                continue;
            }

            let x = pop.0 + pop.2 .0;
            let y = pop.1 + pop.2 .1;

            if x < matrix.len() as i32 && y < matrix[0].len() as i32 && x >= 0 && y >= 0 {
                q.push_back((x, y, pop.2));
            }
        }

        if ch == 3 {
            break;
        }

        ch += 1;
    }

    res
}

#[cfg(test)]
mod tests {
    use super::calc;

    #[test]
    fn first() {
        let matrix = vec![
            vec!['M', 'M', 'M', 'S', 'X', 'X', 'M', 'A', 'S', 'M'],
            vec!['M', 'S', 'A', 'M', 'X', 'M', 'S', 'M', 'S', 'A'],
            vec!['A', 'M', 'X', 'S', 'X', 'M', 'A', 'A', 'M', 'M'],
            vec!['M', 'S', 'A', 'M', 'A', 'S', 'M', 'S', 'M', 'X'],
            vec!['X', 'M', 'A', 'S', 'A', 'M', 'X', 'A', 'M', 'M'],
            vec!['X', 'X', 'A', 'M', 'M', 'X', 'X', 'A', 'M', 'A'],
            vec!['S', 'M', 'S', 'M', 'S', 'A', 'S', 'X', 'S', 'S'],
            vec!['S', 'A', 'X', 'A', 'M', 'A', 'S', 'A', 'A', 'A'],
            vec!['M', 'A', 'M', 'M', 'M', 'X', 'M', 'M', 'M', 'M'],
            vec!['M', 'X', 'M', 'X', 'A', 'X', 'M', 'A', 'S', 'X'],
        ];
        let result = calc(&matrix);

        assert_eq!(18, result);
    }
}
