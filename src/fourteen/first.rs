use std::{
    fs::File,
    io::{BufRead, BufReader},
};

type Tuple = (i32, i32);

pub fn execute(file_path: &str) -> usize {
    let mut positions = Vec::new();
    BufReader::new(File::open(file_path).unwrap())
        .lines()
        .map_while(Result::ok)
        .for_each(|line| {
            let mut segments = line.split_whitespace();

            let position = segments.next().unwrap().strip_prefix("p=").unwrap();
            let mut cords = position.split(',');

            let y = cords.next().unwrap().parse::<i32>().unwrap();
            let x = cords.next().unwrap().parse::<i32>().unwrap();

            let pos = (x, y);

            let velo = segments.next().unwrap().strip_prefix("v=").unwrap();
            let mut v = velo.split(',');

            let y = v.next().unwrap().parse::<i32>().unwrap();
            let x = v.next().unwrap().parse::<i32>().unwrap();
            let velocity = (x, y);

            positions.push(final_position(pos, velocity, 103, 101, 100));
        });

    let mut res = 1;

    res *= count_quadrant(&positions, (0, 51), (0, 50));
    res *= count_quadrant(&positions, (0, 51), (51, 101));
    res *= count_quadrant(&positions, (52, 103), (0, 50));
    res *= count_quadrant(&positions, (52, 103), (51, 101));

    res
}

fn count_quadrant(positions: &[(i32, i32)], x_bounds: (i32, i32), y_bounds: (i32, i32)) -> usize {
    positions
        .iter()
        .filter(|it| {
            it.0 >= x_bounds.0 && it.0 < x_bounds.1 && it.1 >= y_bounds.0 && it.1 < y_bounds.1
        })
        .count()
}

fn final_position(pos: Tuple, velo: Tuple, m: i32, n: i32, seconds: i32) -> (i32, i32) {
    let final_x = modulo(pos.0 + (seconds * velo.0), m);
    let final_y = modulo(pos.1 + (seconds * velo.1), n);

    (final_x, final_y)
}

fn modulo(dividend: i32, divisor: i32) -> i32 {
    let result = dividend % divisor;

    if result < 0 {
        result + divisor
    } else {
        result
    }
}
