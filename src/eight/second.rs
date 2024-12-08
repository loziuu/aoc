use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader},
};

pub fn execute(file: &str) -> usize {
    let matrix: Vec<Vec<char>> = BufReader::new(File::open(file).unwrap())
        .lines()
        .map_while(Result::ok)
        .map(|line| line.chars().collect())
        .collect();

    calc(matrix)
}

fn calc(matrix: Vec<Vec<char>>) -> usize {
    let m = matrix.len() as i32;
    let n = matrix[0].len() as i32;

    let mut map: HashMap<char, Vec<(i32, i32)>> = HashMap::new();
    let mut antinodes: HashSet<(i32, i32)> = HashSet::new();

    for i in 0..matrix.len() {
        for j in 0..matrix[0].len() {
            if matrix[i][j] != '.' {
                map.get(&matrix[i][j])
                    .unwrap_or(&vec![])
                    .iter()
                    .for_each(|(x, y)| {
                        let delta_x = i as i32 - *x;
                        let delta_y = j as i32 - *y;

                        add(*x, *y, -delta_x, -delta_y, m, n, &mut antinodes);
                        add(i as i32, j as i32, delta_x, delta_y, m, n, &mut antinodes);
                    });

                map.entry(matrix[i][j])
                    .or_insert(Vec::new())
                    .push((i as i32, j as i32));
            }
        }
    }

    antinodes.len()
}

fn add(x: i32, y: i32, dx: i32, dy: i32, m: i32, n: i32, set: &mut HashSet<(i32, i32)>) {
    let mut new_x = x;
    let mut new_y = y;

    while new_x >= 0 && new_x < m && new_y >= 0 && new_y < n {
        set.insert((new_x, new_y));

        new_x += dx;
        new_y += dy;
    }
}

#[cfg(test)]
mod tests {
    use super::execute;

    #[test]
    fn first() {
        let result = execute("inputs/eight_test");

        assert_eq!(34, result);
    }
}
