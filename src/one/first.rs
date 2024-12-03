use std::{
    collections::BinaryHeap,
    fs::File,
    io::{BufRead, BufReader},
};

pub fn execute(file_name: &str) -> i32 {
    let mut left: BinaryHeap<i32> = BinaryHeap::new();
    let mut right: BinaryHeap<i32> = BinaryHeap::new();

    let reader = BufReader::new(File::open(file_name).unwrap());

    reader.lines().map_while(Result::ok).for_each(|l| {
        let mut split = l.split_whitespace();

        left.push(split.next().unwrap().parse().unwrap());
        right.push(split.next().unwrap().parse().unwrap());
    });

    let mut res = 0;

    while !left.is_empty() {
        let left_val = left.pop().unwrap();
        let right_val = right.pop().unwrap();

        res += left_val.abs_diff(right_val);
    }

    res as i32
}
