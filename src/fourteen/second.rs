use std::{
    fs::File,
    io::{BufRead, BufReader},
};

type Tuple = (i32, i32);

pub fn execute(file_path: &str) {
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

            positions.push((pos, velocity));
        });

    for i in 0..10000 {
        println!("Iter: {}", i);
        let mut matrix = vec![vec!['.'; 101]; 103];

        for l in 0..positions.len() {
            let position = positions[l];

            matrix[position.0 .0 as usize][position.0 .1 as usize] = 'A';

            positions[l] = (next_position(position.0, position.1, 103, 101), position.1);
        }

        for row in matrix {
            println!("{:?}", row);
        }
    }
}

fn next_position(pos: Tuple, velo: Tuple, m: i32, n: i32) -> (i32, i32) {
    let final_x = modulo(pos.0 + velo.0, m);
    let final_y = modulo(pos.1 + velo.1, n);

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
