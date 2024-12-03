use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader},
};

pub fn execute(file_name: &str) -> i32 {
    let mut left: HashSet<i32> = HashSet::new();
    let mut right: HashMap<i32, i32> = HashMap::new();

    let reader = BufReader::new(File::open(file_name).unwrap());

    reader.lines().map_while(Result::ok).for_each(|l| {
        let mut split = l.split_whitespace();

        left.insert(split.next().unwrap().parse().unwrap());
        *right
            .entry(split.next().unwrap().parse().unwrap())
            .or_insert(0) += 1;
    });

    let mut res = 0;

    for val in left {
        res += val * right.get(&val).unwrap_or(&0);
    }

    res
}
