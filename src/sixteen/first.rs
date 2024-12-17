use std::{
    collections::{BinaryHeap, HashSet},
    fs::File,
    io::{BufRead, BufReader},
};

const DIR: [(i32, i32); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

type Node = (usize, usize);

#[derive(Debug, Eq, PartialEq)]
struct Path {
    position: Node,
    cost: usize,
    dir: usize,
    nodes: HashSet<Node>,
}

impl PartialOrd for Path {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(other.cost.cmp(&self.cost))
    }
}

impl Ord for Path {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.cost.cmp(&other.cost)
    }
}

impl Path {
    fn zero(x: usize, y: usize) -> Path {
        Self {
            position: (x, y),
            cost: 0,
            dir: 1,
            nodes: HashSet::new(),
        }
    }

    fn make_move(&self, x: usize, y: usize, cost: usize, dir: usize) -> Path {
        Self {
            position: (x, y),
            cost: self.cost + cost,
            dir,
            nodes: self.nodes.clone(),
        }
    }
}

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

    let mut q: BinaryHeap<Path> = BinaryHeap::new();
    q.push(Path::zero(start.0, start.1));
    let mut visited = HashSet::new();

    while !q.is_empty() {
        let pop = q.pop().unwrap();

        let point = pop.position;
        if visited.contains(&(point.0, point.1, pop.dir)) {
            continue;
        }
        visited.insert((point.0, point.1, pop.dir));

        if maze[point.0][point.1] == 'E' {
            return pop.cost;
        }

        // Forward move
        let next = next_point(point.0, point.1, pop.dir);
        if maze[next.0][next.1] != '#' {
            q.push(pop.make_move(next.0, next.1, 1, pop.dir));
        }

        // Clockwise turn
        let next_dir = clockwise(pop.dir);
        let next = next_point(point.0, point.1, next_dir);
        if maze[next.0][next.1] != '#' {
            q.push(pop.make_move(next.0, next.1, 1001, next_dir));
        }

        // Counterclockwise turn
        let next_dir = counterwise(pop.dir);
        let next = next_point(point.0, point.1, next_dir);
        if maze[next.0][next.1] != '#' {
            q.push(pop.make_move(next.0, next.1, 1001, next_dir));
        }
    }

    0
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

#[cfg(test)]
mod tests {
    use super::execute;

    #[test]
    fn first() {
        let result = execute("inputs/sixteen_test");

        assert_eq!(11048, result);
    }
}
