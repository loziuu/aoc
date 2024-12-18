use std::{
    cmp::Reverse,
    collections::BinaryHeap,
    fs::File,
    io::{BufRead, BufReader},
};

type Path = (usize, usize, usize);

const DIR: [(i32, i32); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

pub fn execute(file_path: &str) -> usize {
    let bytes = BufReader::new(File::open(file_path).unwrap())
        .lines()
        .map_while(Result::ok)
        .map(|line| {
            let mut splitted = line.split(",");
            (
                splitted.next().unwrap().parse::<usize>().unwrap(),
                splitted.next().unwrap().parse::<usize>().unwrap(),
            )
        })
        .collect();
    traverse(71, bytes, 1024)
}

fn traverse(n: usize, bytes: Vec<(usize, usize)>, first_n_bytes: usize) -> usize {
    let mut matrix = vec![vec![i32::MAX; n]; n];

    for byte in bytes.iter().take(first_n_bytes) {
        matrix[byte.1][byte.0] = -1;
    }

    let mut q: BinaryHeap<Reverse<Path>> = BinaryHeap::new();
    q.push(Reverse((0, 0, 0)));

    while let Some(pop) = q.pop() {
        let val = pop.0;

        if val.1 == n - 1 && val.2 == n - 1 {
            return val.0;
        }

        if matrix[val.1][val.2] <= val.0 as i32 {
            continue;
        }
        matrix[val.1][val.2] = val.0 as i32;

        for i in 0..4 {
            let next = next_point(val.1, val.2, i);

            if next.0 >= n || next.1 >= n {
                continue;
            }

            if matrix[next.0][next.1] != -1 {
                q.push(Reverse((val.0 + 1, next.0, next.1)));
            }
        }
    }

    usize::MAX
}

fn next_point(x: usize, y: usize, dir: usize) -> (usize, usize) {
    (
        (x as i32 + DIR[dir].0) as usize,
        (y as i32 + DIR[dir].1) as usize,
    )
}

#[cfg(test)]
mod tests {
    use super::traverse;

    #[test]
    fn first() {
        let bytes = vec![
            (5, 4),
            (4, 2),
            (4, 5),
            (3, 0),
            (2, 1),
            (6, 3),
            (2, 4),
            (1, 5),
            (0, 6),
            (3, 3),
            (2, 6),
            (5, 1),
            (1, 2),
            (5, 5),
        ];
        let result = traverse(7, bytes, 12);

        assert_eq!(22, result);
    }
}
