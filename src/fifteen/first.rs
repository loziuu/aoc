use std::fs::read_to_string;

const DIR: [(i32, i32); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

pub fn execute(file_path: &str) -> usize {
    let input = read_to_string(file_path).unwrap();

    let mut lines = input.split('\n');

    let mut matrix: Vec<Vec<char>> = Vec::new();
    while let Some(line) = lines.next() {
        if line.is_empty() {
            break;
        }
        let chars = line.chars().collect();
        matrix.push(chars);
    }

    let mut dirs = Vec::new();
    while let Some(line) = lines.next() {
        if line.is_empty() {
            break;
        }
        let mut chars: Vec<char> = line.chars().collect();
        dirs.append(&mut chars);
    }

    traverse(&mut matrix, &dirs);

    let mut res = 0;
    for i in 0..matrix.len() {
        for j in 0..matrix[0].len() {
            if matrix[i][j] == 'O' {
                res += 100 * i + j;
            }
        }
    }
    res
}

fn traverse(matrix: &mut Vec<Vec<char>>, dirs: &[char]) {
    let mut robot_position = (0, 0);

    for i in 0..matrix.len() {
        for j in 0..matrix[0].len() {
            if matrix[i][j] == '@' {
                robot_position = (i, j);
                break;
            }
        }
    }

    for dir in dirs {
        let new_position = move_cell(robot_position.0, robot_position.1, to_dir(*dir), matrix);
        if new_position != robot_position {
            matrix[robot_position.0][robot_position.1] = '.';
        }
        robot_position = new_position;
    }
}

fn move_cell(i: usize, j: usize, dir: usize, matrix: &mut Vec<Vec<char>>) -> (usize, usize) {
    if matrix[i][j] == '#' {
        return (i, j);
    }

    if matrix[i][j] == '.' {
        return (0, 0);
    }

    let d = DIR[dir];
    let next_cell = ((i as i32 + d.0) as usize, (j as i32 + d.1) as usize);
    let next = move_cell(next_cell.0, next_cell.1, dir, matrix);

    if next_cell == next {
        return (i, j);
    }

    matrix[next_cell.0][next_cell.1] = matrix[i][j];
    next_cell
}

fn to_dir(ch: char) -> usize {
    if ch == '^' {
        return 0;
    } else if ch == '>' {
        return 1;
    } else if ch == 'v' {
        return 2;
    } else if ch == '<' {
        return 3;
    }
    return 4;
}

fn print_matrix(matrix: &Vec<Vec<char>>) {
    for row in matrix {
        println!("{:?}", row);
    }
}

#[cfg(test)]
mod tests {
    use super::execute;

    #[test]
    fn first() {
        let result = execute("inputs/fifteen_test");

        assert_eq!(10092, result);
    }
}
