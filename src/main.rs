use std::{
    collections::BinaryHeap,
    fs::File,
    io::{BufRead, BufReader},
};

mod one;

fn main() {
    let file_name = "input";

    let mut left: BinaryHeap<i32> = BinaryHeap::new();
    let mut right: BinaryHeap<i32> = BinaryHeap::new();

    let reader = BufReader::new(File::open(file_name).unwrap());
    for line in reader.lines() {
        if let Ok(l) = line {
            let mut split = l.split_whitespace();

            left.push(split.next().unwrap().parse().unwrap());
            right.push(split.next().unwrap().parse().unwrap());
        }
    }

    let mut res = 0;

    while !left.is_empty() {
        let left_val = left.pop().unwrap();
        let right_val = right.pop().unwrap();

        res += left_val.abs_diff(right_val);
    }

    print!("{}", res);
}
