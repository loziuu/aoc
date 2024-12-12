use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn execute(file: &str) -> usize {
    let matrix: Vec<Vec<char>> = BufReader::new(File::open(file).unwrap())
        .lines()
        .map_while(Result::ok)
        .map(|line| line.chars().collect())
        .collect();

    calc(&matrix)
}

fn calc(matrix: &[Vec<char>]) -> usize {
    let mut visited = vec![vec![false; matrix[0].len()]; matrix.len()];

    let mut res = 0;
    for i in 0..matrix.len() {
        for j in 0..matrix[0].len() {
            if !visited[i][j] {
                let val = dfs(i as i32, j as i32, matrix[i][j], matrix, &mut visited);
                res += val.0 * val.1;
            }
        }
    }
    res
}

fn dfs(
    i: i32,
    j: i32,
    plant: char,
    matrix: &[Vec<char>],
    visited: &mut Vec<Vec<bool>>,
) -> (usize, usize) {
    if i < 0
        || j < 0
        || i >= matrix.len() as i32
        || j >= matrix[0].len() as i32
        || matrix[i as usize][j as usize] != plant
    {
        return (0, 1);
    }

    if visited[i as usize][j as usize] {
        return (0, 0);
    }
    visited[i as usize][j as usize] = true;

    let a = dfs(i + 1, j, plant, matrix, visited);
    let b = dfs(i - 1, j, plant, matrix, visited);
    let c = dfs(i, j + 1, plant, matrix, visited);
    let d = dfs(i, j - 1, plant, matrix, visited);

    (1 + a.0 + b.0 + c.0 + d.0, a.1 + b.1 + c.1 + d.1)
}

#[cfg(test)]
mod tests {
    use crate::twelve::first::calc;

    #[test]
    fn first() {
        let matrix = vec![
            vec!['R', 'R', 'R', 'R', 'I', 'I', 'C', 'C', 'F', 'F'],
            vec!['R', 'R', 'R', 'R', 'I', 'I', 'C', 'C', 'C', 'F'],
            vec!['V', 'V', 'R', 'R', 'R', 'C', 'C', 'F', 'F', 'F'],
            vec!['V', 'V', 'R', 'C', 'C', 'C', 'J', 'F', 'F', 'F'],
            vec!['V', 'V', 'V', 'V', 'C', 'J', 'J', 'C', 'F', 'E'],
            vec!['V', 'V', 'I', 'V', 'C', 'C', 'J', 'J', 'E', 'E'],
            vec!['V', 'V', 'I', 'I', 'I', 'C', 'J', 'J', 'E', 'E'],
            vec!['M', 'I', 'I', 'I', 'I', 'I', 'J', 'J', 'E', 'E'],
            vec!['M', 'I', 'I', 'I', 'S', 'I', 'J', 'E', 'E', 'E'],
            vec!['M', 'M', 'M', 'I', 'S', 'S', 'J', 'E', 'E', 'E'],
        ];
        let result = calc(&matrix);

        assert_eq!(1930, result);
    }
}
