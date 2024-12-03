use std::{
    fs::File,
    io::{BufRead, BufReader},
    ops::RangeInclusive,
};

pub fn execute(file_name: File) -> usize {
    let reader = BufReader::new(file_name);

    reader
        .lines()
        .map_while(Result::ok)
        .filter(|it| {
            let level: Vec<i32> = it
                .split_whitespace()
                .map(|val| val.parse::<i32>().unwrap())
                .collect();
            is_valid(&level)
        })
        .count()
}

fn is_valid(level: &[i32]) -> bool {
    if level.len() == 1 {
        return true;
    }

    check_dp(level, -3..=-1) || check_dp(level, 1..=3)
}

fn check_dp(level: &[i32], range: RangeInclusive<i32>) -> bool {
    let n = level.len();
    let mut dp = vec![0; n];

    for i in 1..level.len() {
        if range.contains(&(level[i] - level[i - 1])) {
            dp[i] = dp[i - 1] + 1;
        }
    }

    dp[n - 1] >= n - 1
}
