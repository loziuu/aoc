use std::{
    collections::{HashMap, VecDeque},
    fs::read_to_string,
    hash::Hash,
};

const DIR: [(i32, i32); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

pub fn execute(file_path: &str) -> usize {
    let input = read_to_string(file_path).unwrap();

    let mut lines = input.split('\n');

    let mut matrix: Vec<Vec<char>> = Vec::new();
    for line in lines.by_ref() {
        if line.is_empty() {
            break;
        }
        let chars = line
            .chars()
            .flat_map(|it| match it {
                '#' => vec!['#', '#'],
                '@' => vec!['@', '.'],
                '.' => vec!['.', '.'],
                'O' => vec!['[', ']'],
                _ => panic!("Wrong character"),
            })
            .collect();
        matrix.push(chars);
    }

    let mut dirs = Vec::new();
    for line in lines {
        if line.is_empty() {
            break;
        }
        let mut chars: Vec<char> = line.chars().collect();
        dirs.append(&mut chars);
    }

    traverse(&mut matrix, &dirs);
    print_matrix(&matrix);

    let mut res = 0;
    for i in 0..matrix.len() {
        for j in 0..matrix[0].len() {
            if matrix[i][j] == '[' {
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

    let mut queue = VecDeque::new();
    let mut moves = HashMap::new();

    for dir in dirs {
        let d = DIR[to_dir(*dir)];
        println!("Moving {}", dir);

        queue.push_back(robot_position);
        moves.insert(robot_position, '.');

        let temp = robot_position;
        robot_position = (
            (robot_position.0 as i32 + (d.0 as i32)) as usize,
            (robot_position.1 as i32 + (d.1 as i32)) as usize,
        );

        if d.1 != 0 {
            while !queue.is_empty() {
                let pop = queue.pop_front().unwrap();

                let next = (
                    (pop.0 as i32 + (d.0 as i32)) as usize,
                    (pop.1 as i32 + (d.1 as i32)) as usize,
                );

                if matrix[pop.0][pop.1] == '.' {
                    continue;
                }

                if matrix[next.0][next.1] == '#' {
                    robot_position = temp;
                    moves.clear();
                    queue.clear();
                    break;
                }

                queue.push_back(next);
                moves.insert(next, matrix[pop.0][pop.1]);
            }
        } else {
            while !queue.is_empty() {
                let pop = queue.pop_front().unwrap();

                let next = (
                    (pop.0 as i32 + (d.0 as i32)) as usize,
                    (pop.1 as i32 + (d.1 as i32)) as usize,
                );

                let curr = matrix[pop.0][pop.1];
                if curr == '.' {
                    continue;
                }

                if curr == '#' {
                    robot_position = temp;
                    moves.clear();
                    queue.clear();
                    break;
                }

                let next_ch = matrix[next.0][next.1];

                if curr == '@' {
                    if next_ch == '[' {
                        queue.push_back(next);
                        moves.insert(next, '@');
                        moves.insert((next.0, next.1 + 1), '.');
                        continue;
                    }

                    if next_ch == ']' {
                        queue.push_back((next.0, next.1 - 1));
                        moves.insert(next, '@');
                        moves.insert((next.0, next.1 - 1), '.');
                        continue;
                    }
                }

                if curr == ']' {
                    queue.push_back((pop.0, pop.1 - 1));
                    continue;
                }

                if curr == '[' {
                    println!("Pushing box at: [{},{}]", pop.0, pop.1);
                    if matrix[next.0][next.1] == '#' && matrix[next.0][next.1 + 1] == '#' {
                        robot_position = temp;
                        moves.clear();
                        queue.clear();
                        break;
                    }

                    moves.entry(pop).or_insert('.');
                    moves.entry((pop.0, pop.1 + 1)).or_insert('.');

                    queue.push_back((next.0, next.1));
                    queue.push_back((next.0, next.1 + 1));
                    moves.insert(next, '[');
                    moves.insert((next.0, next.1 + 1), ']');
                    continue;
                }

                queue.push_back(next);
                moves.insert(next, matrix[pop.0][pop.1]);
            }
        }

        for (k, v) in moves.iter() {
            matrix[k.0][k.1] = *v;
        }
        print_matrix(matrix);
    }
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
    4
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

        assert_eq!(9021, result);
    }
}
