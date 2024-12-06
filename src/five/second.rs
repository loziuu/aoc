use std::{
    cmp::Ordering,
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

type Graph = HashMap<i32, Vec<i32>>;

pub fn execute(file: &str) -> i32 {
    let mut g = HashMap::new();
    let mut updates = Vec::new();

    let mut ordering_lines = 0;

    BufReader::new(File::open(file).unwrap())
        .lines()
        .map_while(Result::ok)
        .take_while(|line| !line.is_empty())
        .for_each(|line| {
            let mut splitted = line.split('|');
            let from = splitted.next().unwrap().parse::<i32>().unwrap();
            let to = splitted.next().unwrap().parse::<i32>().unwrap();

            g.entry(from).or_insert(vec![]).push(to);
            ordering_lines += 1;
        });

    BufReader::new(File::open(file).unwrap())
        .lines()
        .skip(ordering_lines + 1)
        .map_while(Result::ok)
        .for_each(|line| {
            let update: Vec<i32> = line.split(',').map(|it| it.parse().unwrap()).collect();

            updates.push(update);
        });

    updates
        .iter()
        .map(|update| {
            if check(update, &g) {
                return 0;
            }

            order(update.to_vec(), &g)
        })
        .sum()
}

fn check(updates: &[i32], g: &Graph) -> bool {
    for i in 0..updates.len() - 1 {
        if g.get(&updates[i + 1])
            .map(|it| (*it).contains(&updates[i]))
            .unwrap_or(false)
        {
            return false;
        }
    }
    true
}

fn order(mut update: Vec<i32>, g: &Graph) -> i32 {
    update.sort_by(|a, b| {
        if g.get(a).map(|it| (*it).contains(b)).unwrap_or(false) {
            return Ordering::Greater;
        }
        Ordering::Less
    });

    update[update.len() / 2]
}

#[cfg(test)]
mod tests {
    use crate::five::second::execute;

    #[test]
    fn first() {
        let result = execute("inputs/five_test");

        assert_eq!(123, result);
    }
}
