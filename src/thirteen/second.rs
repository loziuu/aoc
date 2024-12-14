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
    let det = a.0 * b.1 - a.1 * b.0;

    if det != 0 {
        if (prize.0 * b.1 - prize.1 * b.0) % det != 0 {
            return 0;
        }
        let first = (prize.0 * b.1 - prize.1 * b.0) / det;

        if (a.0 * prize.1 - a.1 * prize.0) % det != 0 {
            return 0;
        }
        let second = (a.0 * prize.1 - a.1 * prize.0) / det;

        return second + (3 * first);
    }

    0
}
