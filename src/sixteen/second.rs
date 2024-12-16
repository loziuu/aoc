use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet, VecDeque},
    fs::File,
    io::{BufRead, BufReader},
};

const DIR: [(i32, i32); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

pub fn execute(file_path: &str) -> usize {
    let maze = BufReader::new(File::open(file_path).unwrap())
        .lines()
        .map_while(Result::ok)
        .map(|line| line.chars().collect())
        .collect();

    solve(maze)
}

fn solve(maze: Vec<Vec<char>>) -> usize {
    let mut start = (0, 0);

    for i in 0..maze.len() {
        for j in 0..maze[0].len() {
            if maze[i][j] == 'S' {
                start = (i, j);
            }
        }
    }

    // cost, (x, y, dir)
    let mut q: BinaryHeap<Reverse<(usize, (usize, usize, usize))>> = BinaryHeap::new();
    q.push(Reverse((0, (start.0, start.1, 1))));
    let mut visited = HashSet::new();

    while !q.is_empty() {
        let pop = q.pop().unwrap().0;

        let point = pop.1;

        if visited.contains(&(point.0, point.1, point.2)) {
            continue;
        }
        visited.insert((point.0, point.1, point.2));

        if maze[point.0][point.1] == 'E' {
            return backtrack(point.0, point.1, &maze, point.0);
        }

        // Forward move
        let next = next_point(point.0, point.1, point.2);
        if maze[next.0][next.1] != '#' {
            q.push(Reverse((pop.0 + 1, (next.0, next.1, point.2))));
        }

        // Clockwise turn
        let next_dir = clockwise(point.2);
        let next = next_point(point.0, point.1, next_dir);
        if maze[next.0][next.1] != '#' {
            q.push(Reverse((pop.0 + 1001, (next.0, next.1, next_dir))));
        }

        // Counterclockwise turn
        let next_dir = counterwise(point.2);
        let next = next_point(point.0, point.1, next_dir);
        if maze[next.0][next.1] != '#' {
            q.push(Reverse((pop.0 + 1001, (next.0, next.1, next_dir))));
        }
    }

    usize::MAX
}

fn clockwise(dir: usize) -> usize {
    (dir + 1) % 4
}

fn counterwise(dir: usize) -> usize {
    if dir == 0 {
        return 3;
    }
    dir - 1
}

fn next_point(x: usize, y: usize, dir: usize) -> (usize, usize) {
    (
        (x as i32 + DIR[dir].0) as usize,
        (y as i32 + DIR[dir].1) as usize,
    )
}

fn backtrack(i: usize, j: usize, maze: &Vec<Vec<char>>, shortest_path: usize) -> usize {
    let mut q: VecDeque<(usize, usize, i32)> = VecDeque::new();

    for i 

    let mut result = 0;

    while !q.is_empty() {
        
    }
}

#[cfg(test)]
mod tests {
    use super::execute;

    #[test]
    fn first() {
        let result = execute("inputs/sixteen_test");

        assert_eq!(11048, result);
    }
}
