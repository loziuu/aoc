use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
};

pub fn execute(file_path: &str) -> usize {
    let input = read_to_string(file_path).unwrap();
    let mut lines = input.split("\n");

    let available: Vec<&str> = lines.next().unwrap().split(", ").collect();
    lines.next().unwrap();
    let targets: Vec<&str> = lines.filter(|it| !it.is_empty()).collect();

    collect_towels_ref(available, targets)
}

fn collect_towels_ref(mut available: Vec<&str>, target: Vec<&str>) -> usize {
    let mut res = 0;
    let mut cache: HashMap<String, usize> = HashMap::new();

    for t in target {
        res += check_target(t, &mut available, &mut cache);
    }
    res
}

fn check_target(target: &str, available: &mut [&str], cache: &mut HashMap<String, usize>) -> usize {
    if cache.contains_key(target) {
        return *cache.get(target).unwrap();
    }

    if target.is_empty() {
        return 1;
    }

    let mut res = 0;
    for i in 0..available.len() {
        if let Some(next) = target.strip_prefix(available[i]) {
            res += check_target(next, available, cache);
        }
    }

    cache.insert(target.to_string(), res);
    res
}

fn check_target_no_repeat(
    target: &str,
    available: &mut [String],
    taken: &mut HashSet<usize>,
) -> bool {
    if target.is_empty() {
        return true;
    }

    for i in 0..available.len() {
        if taken.contains(&i) {
            continue;
        }

        if let Some(next) = target.strip_prefix(&available[i]) {
            taken.insert(i);
            let res = check_target_no_repeat(next, available, taken);
            taken.remove(&i);

            if res {
                return res;
            }
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use crate::nineteen::second::collect_towels_ref;

    #[test]
    fn first() {
        let available = vec!["r", "wr", "b", "g", "bwu", "rb", "gb", "br"];
        let target = vec![
            "brwrr", "bggr", "gbbr", "rrbgbr", "ubwu", "bwurrg", "brgr", "bbrgwb",
        ];

        assert_eq!(16, collect_towels_ref(available, target));
    }
}
