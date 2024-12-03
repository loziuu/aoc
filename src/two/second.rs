use std::{
    fs::File,
    io::{BufRead, BufReader},
    ops::RangeInclusive,
};

const DECREASING: RangeInclusive<i32> = -3..=-1;
const INCREASING: RangeInclusive<i32> = 1..=3;

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

    check_dp(level, DECREASING, 1) || check_dp(level, INCREASING, 1)
}

fn check_dp(level: &[i32], range: RangeInclusive<i32>, skippable: usize) -> bool {
    let n = level.len();
    let mut dp = vec![1; n];

    for i in 1..level.len() {
        let mut val = 0;

        for j in 0..=skippable {
            let prev_i = i as i32 - 1 - j as i32;
            if prev_i >= 0 && range.contains(&(level[i] - level[prev_i as usize])) {
                val = val.max(dp[prev_i as usize]);
            }
        }

        dp[i] = val + 1;

        if dp[i] >= n - 1 {
            return true;
        }
    }

    false
}

#[cfg(test)]
mod tests {

    use crate::two::second::is_valid;

    #[test]
    fn first() {
        assert!(is_valid(vec![7, 6, 4, 2, 1].as_slice()));
        assert!(!is_valid(vec![1, 2, 7, 8, 9].as_slice()));
        assert!(!is_valid(vec![9, 7, 6, 2, 1].as_slice()));
        assert!(is_valid(vec![8, 6, 4, 4, 1].as_slice()));
        assert!(is_valid(vec![1, 3, 6, 7, 9].as_slice()));
        assert!(!is_valid(vec![4, 4, 4, 4, 4].as_slice()));
        assert!(is_valid(vec![4, 5, 6, 4, 7].as_slice()));
        assert!(is_valid(vec![1, 2, 0, -1, -2].as_slice()));
        assert!(is_valid(vec![1, 3, 5, 6, 8, 9, 12, 9].as_slice()));
        assert!(is_valid(vec![76, 78, 81, 84, 85, 89, 92, 96].as_slice()));
    }
}
