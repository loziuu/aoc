use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn execute(file: &str) -> i64 {
    let mut count = 1;
    BufReader::new(File::open(file).unwrap())
        .lines()
        .map_while(Result::ok)
        .map(|line| {
            let mut splitted = line.split(':');
            let target: i64 = splitted.next().unwrap().parse().unwrap();

            let values: Vec<i32> = splitted
                .next()
                .unwrap()
                .split_whitespace()
                .map(|it| it.parse::<i32>().unwrap())
                .collect();

            if can_evaluate(1, values[0] as i64, &values, target) {
                return target;
            }
            0
        })
        .sum()
}

fn can_evaluate(i: usize, value: i64, values: &[i32], target: i64) -> bool {
    if i >= values.len() {
        return target == value;
    }

    can_evaluate(i + 1, value + values[i] as i64, values, target)
        || can_evaluate(i + 1, value * values[i] as i64, values, target)
}

#[cfg(test)]
mod tests {
    use super::execute;

    #[test]
    fn first() {
        let result = execute("inputs/seven_test");

        assert_eq!(result, 3749);
    }
}
