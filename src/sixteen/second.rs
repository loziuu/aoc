use std::{
    collections::{BinaryHeap, HashSet, VecDeque},
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

fn solve(mut maze: Vec<Vec<char>>) -> usize {
    let mut start = (0, 0);

    for i in 0..maze.len() {
        for j in 0..maze[0].len() {
            if maze[i][j] == 'S' {
                start = (i, j);
            }
        }
    }

    // cost, (x, y, dir)
    let mut q: BinaryHeap<Path> = BinaryHeap::new();
    q.push(Path::zero(start.0, start.1));
    let mut visited = vec![vec![vec![usize::MAX; 4]; maze[0].len()]; maze.len()];

    while !q.is_empty() {
        let pop = q.pop().unwrap();
        let point = pop.position;

        if pop.cost > visited[pop.position.0][pop.position.1][pop.dir] {
            continue;
        }
        visited[pop.position.0][pop.position.1][pop.dir] = pop.cost;

        if maze[point.0][point.1] == 'E' {
            visited[point.0][point.1][0] = pop.cost;
            visited[point.0][point.1][1] = pop.cost;
            visited[point.0][point.1][2] = pop.cost;
            visited[point.0][point.1][3] = pop.cost;
            return backtrack(start, (point.0, point.1), visited, &mut maze);
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

fn backtrack(
    start: (usize, usize),
    end: (usize, usize),
    visited: Vec<Vec<Vec<usize>>>,
    maze: &mut Vec<Vec<char>>,
) -> usize {
    let mut vec = VecDeque::new();
    let mut nodes = HashSet::new();
    vec.push_back((end.0, end.1, 0));
    vec.push_back((end.0, end.1, 1));
    vec.push_back((end.0, end.1, 2));
    vec.push_back((end.0, end.1, 3));

    while !vec.is_empty() {
        let size = vec.len();

        for _ in 0..size {
            let pop = vec.pop_front().unwrap();

            if nodes.contains(&pop) {
                continue;
            }
            nodes.insert(pop);
            maze[pop.0][pop.1] = 'O';

            if (pop.0, pop.1) == start {
                break;
            }

            let cached = visited[pop.0][pop.1][pop.2];

            if cached != usize::MAX {
                let rev_dir = reverse(pop.2);
                let reverse = (add(pop.0, DIR[rev_dir].0), add(pop.1, DIR[rev_dir].1));

                for next_dir in 0..DIR.len() {
                    if cached - 1 == visited[reverse.0][reverse.1][next_dir]
                        || cached - 1001 == visited[reverse.0][reverse.1][next_dir]
                    {
                        vec.push_back((reverse.0, reverse.1, next_dir));
                    }
                }
            }
        }
    }

    let mut count = 0;
    for row in maze {
        count += row.iter().filter(|ch| *ch == &'O').count();
    }

    count
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

fn reverse(dir: usize) -> usize {
    (dir + 2) % 4
}

fn next_point(x: usize, y: usize, dir: usize) -> (usize, usize) {
    (
        (x as i32 + DIR[dir].0) as usize,
        (y as i32 + DIR[dir].1) as usize,
    )
}

fn add(x: usize, y: i32) -> usize {
    (x as i32 + y) as usize
}

fn dir_to_char(dir: usize) -> char {
    match dir {
        0 => '^',
        1 => '>',
        2 => 'v',
        3 => '<',
        _ => panic!("chuj"),
    }
}

#[cfg(test)]
mod tests {
    use super::execute;

    #[test]
    fn first() {
        let result = execute("inputs/sixteen_test");

        assert_eq!(64, result);
    }
}
