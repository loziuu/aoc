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

            count += 1;
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
        || can_evaluate(i + 1, concat(value, values[i] as i64), values, target)
}

fn concat(a: i64, b: i64) -> i64 {
    format!("{}{}", a, b).parse().unwrap()
}

#[cfg(test)]
mod tests {
    use super::{concat, execute};

    #[test]
    fn con() {
        let con = concat(12341, 1233);

        assert_eq!(con, 123411233);
    }

    #[test]
    fn second() {
        let result = execute("inputs/seven_test");

        assert_eq!(result, 11387);
    }
}
