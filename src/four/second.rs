use std::{
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

fn calc(matrix: &Vec<Vec<char>>) -> i32 {
    let n = matrix.len();
    let m = matrix[0].len();

    let mut result = 0;
    for i in 1..n - 1 {
        for j in 1..m - 1 {
            if matrix[i][j] == 'A' && left(i, j, matrix) && right(i, j, matrix) {
                result += 1;
            }
        }
    }

    result
}

fn left(i: usize, j: usize, matrix: &[Vec<char>]) -> bool {
    (matrix[i - 1][j - 1] == 'M' && matrix[i + 1][j + 1] == 'S')
        || (matrix[i - 1][j - 1] == 'S' && matrix[i + 1][j + 1] == 'M')
}

fn right(i: usize, j: usize, matrix: &[Vec<char>]) -> bool {
    (matrix[i - 1][j + 1] == 'M' && matrix[i + 1][j - 1] == 'S')
        || (matrix[i - 1][j + 1] == 'S' && matrix[i + 1][j - 1] == 'M')
}

#[cfg(test)]
mod tests {
    use super::calc;

    #[test]
    fn second() {
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

        assert_eq!(9, result);
    }
}
