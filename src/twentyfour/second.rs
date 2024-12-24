use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
};

enum Gate {
    And,
    Or,
    Xor,
}

impl Gate {
    pub fn calc(&self, a: bool, b: bool) -> bool {
        match self {
            Gate::And => a & b,
            Gate::Or => a | b,
            Gate::Xor => a ^ b,
        }
    }
}

impl From<&str> for Gate {
    fn from(value: &str) -> Self {
        match value {
            "AND" => Gate::And,
            "OR" => Gate::Or,
            "XOR" => Gate::Xor,
            _ => panic!("Invalid gate"),
        }
    }
}

struct Formula {
    gate: Gate,
    left: String,
    right: String,
}

impl Formula {
    pub fn new(left: String, right: String, gate: String) -> Self {
        Self {
            left,
            right,
            gate: gate.as_str().into(),
        }
    }
}

enum Wire {
    Value(bool),
    Formula(Formula),
}

pub fn execute(file_path: &str) -> String {
    let val = read_to_string(file_path).unwrap();
    let mut wires = HashMap::new();

    let mut z = Vec::new();

    let mut lines = val.lines();

    for next in lines.by_ref() {
        if next.is_empty() {
            break;
        }
        let mut split_whitespace = next.split_whitespace();

        let var = split_whitespace.next().unwrap().strip_suffix(":").unwrap();
        let val = if let Some(val) = split_whitespace.next() {
            val == "1"
        } else {
            false
        };

        wires.insert(var.to_string(), Wire::Value(val));
    }

    for next in lines {
        if next.is_empty() {
            break;
        }
        let mut split_whitespace = next.split_whitespace();

        let a = split_whitespace.next().unwrap();
        let gate = split_whitespace.next().unwrap();
        let b = split_whitespace.next().unwrap();
        split_whitespace.next().unwrap();
        let c = split_whitespace.next().unwrap();
        if c.starts_with("z") {
            z.push(c);
        }

        wires.insert(
            c.to_string(),
            Wire::Formula(Formula::new(a.to_owned(), b.to_owned(), gate.to_owned())),
        );
    }

    z.sort();

    let mut mixed = HashSet::new();
    z.iter().for_each(|it| {
        println!("Checking {}", it);
        verfiy(it, &wires, &mut mixed, it);
    });

    let mut v: Vec<String> = mixed.into_iter().collect();
    v.sort();
    v.join(",")
}

fn verfiy(a: &str, wires: &HashMap<String, Wire>, mixed: &mut HashSet<String>, expected: &str) {
    if let Wire::Formula(formula) = wires.get(a).unwrap() {
        println!(
            "Checking {} and {} for {}. Exptected: {}",
            formula.left, formula.right, a, expected
        );
        let is_x = formula.left.starts_with("x");
        let is_y = formula.left.starts_with("x");

        if !is_x && !is_y {
            verfiy(&formula.left, wires, mixed, expected);
            verfiy(&formula.right, wires, mixed, expected);
        } else {
            let left = if is_x {
                formula.left.strip_prefix("x").unwrap()
            } else {
                formula.left.strip_prefix("y").unwrap()
            };
            let target = expected.strip_prefix("z").unwrap();

            if left != target {
                println!(
                    "{} and {} is mixed up. Adding {}",
                    formula.left, expected, a
                );
                mixed.insert(a.to_owned());
            }
            println!("Its correct");
        }
    }
}
