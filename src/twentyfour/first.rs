use std::{collections::HashMap, fs::read_to_string, hash::Hash};

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

pub fn execute(file_path: &str) -> usize {
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

    let mut result = 0;

    (0..z.len()).for_each(|i| {
        if get_val(z[i], &wires) {
            result |= 1 << i;
        }
    });

    result
}

fn get_val(a: &str, wires: &HashMap<String, Wire>) -> bool {
    match wires.get(a).unwrap() {
        Wire::Value(v) => *v,
        Wire::Formula(formula) => {
            let left = get_val(&formula.left, wires);
            let right = get_val(&formula.right, wires);
            formula.gate.calc(left, right)
        }
    }
}
