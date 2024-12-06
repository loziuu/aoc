use std::{
    fs::File,
    io::{BufRead, BufReader},
};

const DIR: [(i32, i32); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

pub fn execute(file: &str) -> i32 {
    let mut matrix: Vec<Vec<char>> = BufReader::new(File::open(file).unwrap())
        .lines()
        .map_while(Result::ok)
        .map(|line| {
            let v: Vec<char> = line.chars().collect();
            v
        })
        .collect();

    mark_patroled(&mut matrix);

    let mut result = 0;

    for i in 0..matrix.len() {
        for j in 0..matrix[0].len() {
            if matrix[i][j] == 'X' {
                result += 1;
            }
        }
    }
    result
}

fn mark_patroled(matrix: &mut Vec<Vec<char>>) {
    for i in 0..matrix.len() {
        for j in 0..matrix[0].len() {
            if matrix[i][j] == '^' {
                traverse(i as i32, j as i32, 0, matrix);
                return;
            }
        }
    }
}

fn traverse(i: i32, j: i32, dir: usize, matrix: &mut [Vec<char>]) -> i32 {
    if i < 0 || j < 0 || i >= matrix.len() as i32 || j >= matrix[0].len() as i32 {
        return 0;
    }

    if matrix[i as usize][j as usize] == '#' {
        return -1;
    }

    matrix[i as usize][j as usize] = 'X';
    let n = traverse(i + DIR[dir].0, j + DIR[dir].1, dir, matrix);

    if n == -1 {
        let dir = next(dir);
        return 1 + traverse(i + DIR[dir].0, j + DIR[dir].1, dir, matrix);
    }

    1 + n
}

fn next(dir: usize) -> usize {
    (dir + 1) % DIR.len()
}

#[cfg(test)]
mod tests {
    use super::execute;

    #[test]
    fn first() {
        let result = execute("inputs/six_test");

        assert_eq!(41, result);
    }
}
