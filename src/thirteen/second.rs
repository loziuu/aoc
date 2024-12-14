use std::fs::read_to_string;

type Tuple = (i64, i64);

const OFFSET: i64 = 10000000000000;

pub fn execute(file: &str) -> i64 {
    let contents = read_to_string(file).unwrap();

    let mut result = 0;

    let mut lines = contents.split('\n');
    let mut i = 0;
    let mut a = (0, 0);
    let mut b = (0, 0);
    while let Some(next) = lines.next() {
        let mut blocks = next.split_whitespace();

        if i == 0 {
            blocks.next().unwrap();
            blocks.next().unwrap();

            a = (
                blocks
                    .next()
                    .unwrap()
                    .strip_prefix("X+")
                    .unwrap()
                    .strip_suffix(",")
                    .unwrap()
                    .parse()
                    .unwrap(),
                blocks
                    .next()
                    .unwrap()
                    .strip_prefix("Y+")
                    .unwrap()
                    .parse()
                    .unwrap(),
            );
        }

        if i == 1 {
            blocks.next().unwrap();
            blocks.next().unwrap();

            b = (
                blocks
                    .next()
                    .unwrap()
                    .strip_prefix("X+")
                    .unwrap()
                    .strip_suffix(",")
                    .unwrap()
                    .parse()
                    .unwrap(),
                blocks
                    .next()
                    .unwrap()
                    .strip_prefix("Y+")
                    .unwrap()
                    .parse()
                    .unwrap(),
            );
        }

        if i == 2 {
            blocks.next().unwrap();

            let prize = (
                OFFSET
                    + blocks
                        .next()
                        .unwrap()
                        .strip_prefix("X=")
                        .unwrap()
                        .strip_suffix(",")
                        .unwrap()
                        .parse::<i64>()
                        .unwrap(),
                OFFSET
                    + blocks
                        .next()
                        .unwrap()
                        .strip_prefix("Y=")
                        .unwrap()
                        .parse::<i64>()
                        .unwrap(),
            );
            result += calc(a, b, prize);
        }
        i = (i + 1) % 4;
    }

    result
}

fn calc(a: Tuple, b: Tuple, prize: Tuple) -> i64 {
    traverse(a, b, prize)
}

fn traverse(a: Tuple, b: Tuple, prize: Tuple) -> i64 {
    let mut max_b = (prize.0 / b.0).min(prize.1 / b.1);

    while max_b > 1 {
        let a_0 = if (prize.0 - (max_b * b.0)) % a.0 == 0 {
            (prize.0 - (max_b * b.0)) / a.0
        } else {
            -1
        };
        let a_1 = if (prize.1 - (max_b * b.1)) % a.1 == 0 {
            (prize.1 - (max_b * b.1)) / a.1
        } else {
            -1
        };

        if a_0 != -1 && a_1 != -1 && a_0 == a_1 {
            return max_b + (a_0 * 3);
        }

        max_b -= 1;
    }

    0
}

#[cfg(test)]
mod tests {
    use crate::thirteen::second::{calc, OFFSET};

    #[test]
    fn first() {
        let a = (94, 34);
        let b = (22, 67);
        let prize = (OFFSET + 8400, OFFSET + 5400);

        assert_eq!(0, calc(a, b, prize));
    }

    #[test]
    fn second() {
        let a = (26, 66);
        let b = (67, 21);
        let prize = (OFFSET + 12748, OFFSET + 12176);

        assert_eq!(0, calc(a, b, prize));
    }

    #[test]
    fn third() {
        let a = (17, 86);
        let b = (84, 37);
        let prize = (OFFSET + 7870, OFFSET + 6450);

        assert_eq!(0, calc(a, b, prize));
    }

    #[test]
    fn fourth() {
        let a = (69, 23);
        let b = (27, 71);
        let prize = (OFFSET + 18641, OFFSET + 10279);

        assert_eq!(0, calc(a, b, prize));
    }
}
