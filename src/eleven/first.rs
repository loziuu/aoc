use core::num;
use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

use crate::eight::first;

pub fn execute(file: &str, blinks: usize) -> usize {
    let mut file_lines = BufReader::new(File::open(file).unwrap()).lines();

    if let Ok(line) = file_lines.next().unwrap() {
        let stones: Vec<usize> = line
            .split_whitespace()
            .map(|it| it.parse().unwrap())
            .collect();
        return calc_after(&stones, blinks);
    }

    0
}

fn calc_after(stones: &[usize], blinks: usize) -> usize {
    let mut res = 0;

    let mut memo: HashMap<(usize, usize), usize> = HashMap::new();

    for val in stones {
        res += calc(*val, 0, blinks, &mut memo);
    }

    res
}

fn calc(
    value: usize,
    blinks: usize,
    target_blinks: usize,
    memo: &mut HashMap<(usize, usize), usize>,
) -> usize {
    if blinks == target_blinks {
        return 1;
    }

    let key = (value, target_blinks - blinks);
    if memo.contains_key(&key) {
        return *memo.get(&key).unwrap();
    }
    let number = format!("{}", value);

    let res = if value == 0 {
        return calc(1, blinks + 1, target_blinks, memo);
    } else if format!("{}", value).len() % 2 == 0 {
        let split = number.split_at(number.len() / 2);
        let first_half = split.0.parse::<usize>().unwrap();
        let second_half = split.1.parse::<usize>().unwrap();

        return calc(first_half, blinks + 1, target_blinks, memo)
            + calc(second_half, blinks + 1, target_blinks, memo);
    } else {
        calc(value * 2024, blinks + 1, target_blinks, memo)
    };

    memo.insert(key, res);
    res
}

#[cfg(test)]
mod tests {
    use super::calc_after;

    #[test]
    fn first() {
        let result = calc_after(&[125, 17], 25);

        assert_eq!(55312, result);
    }
}
