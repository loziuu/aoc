use std::{
    collections::HashSet,
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

    let mut sides = HashSet::new();
    let mut res = 0;
    for i in 0..matrix.len() {
        for j in 0..matrix[0].len() {
            if !visited[i][j] {
                let val = dfs(
                    i as i32,
                    j as i32,
                    matrix[i][j],
                    matrix,
                    &mut sides,
                    &mut visited,
                );

                let cnt = calc_sides(&sides);
                res += val * cnt;
                sides.clear();
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
    sides: &mut HashSet<(i32, i32, i32)>,
    visited: &mut Vec<Vec<bool>>,
) -> usize {
    if visited[i as usize][j as usize] {
        return 0;
    }
    visited[i as usize][j as usize] = true;

    let m = matrix.len();
    let n = matrix[0].len();

    let mut res = 1;
    if is_in_bounds(i + 1, j, m, n) && matrix[i as usize + 1][j as usize] == plant {
        res += dfs(i + 1, j, plant, matrix, sides, visited);
    } else {
        sides.insert((0, i + 1, j));
    }

    if is_in_bounds(i - 1, j, m, n) && matrix[i as usize - 1][j as usize] == plant {
        res += dfs(i - 1, j, plant, matrix, sides, visited);
    } else {
        sides.insert((1, i - 1, j));
    }

    if is_in_bounds(i, j + 1, m, n) && matrix[i as usize][j as usize + 1] == plant {
        res += dfs(i, j + 1, plant, matrix, sides, visited);
    } else {
        sides.insert((2, i, j + 1));
    }

    if is_in_bounds(i, j - 1, m, n) && matrix[i as usize][j as usize - 1] == plant {
        res += dfs(i, j - 1, plant, matrix, sides, visited);
    } else {
        sides.insert((3, i, j - 1));
    }

    res
}

fn is_in_bounds(i: i32, j: i32, m: usize, n: usize) -> bool {
    i >= 0 && j >= 0 && i < m as i32 && j < n as i32
}

fn calc_sides(set: &HashSet<(i32, i32, i32)>) -> usize {
    let mut res = 0;

    for i in 0..4 {
        let mut a: Vec<&(i32, i32, i32)> = set.iter().filter(|it| it.0 == i).collect();

        let mut r = 1;
        if i == 0 || i == 1 {
            a.sort_by_key(|it| (it.1, it.2));

            for i in 1..a.len() {
                if a[i].1 != a[i - 1].1 || a[i].2.abs_diff(a[i - 1].2) != 1 {
                    r += 1;
                }
            }
        } else {
            a.sort_by_key(|it| (it.2, it.1));

            for i in 1..a.len() {
                if a[i].2 != a[i - 1].2 || a[i].1.abs_diff(a[i - 1].1) != 1 {
                    r += 1;
                }
            }
        }

        res += r;
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn second() {
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

        assert_eq!(1206, result);
    }
}
