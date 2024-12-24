use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader},
};

type Graph = HashMap<String, Vec<String>>;

pub fn execute(file_path: &str) -> usize {
    let mut graph = Graph::new();

    BufReader::new(File::open(file_path).unwrap())
        .lines()
        .map_while(Result::ok)
        .for_each(|it| {
            let mut splitted = it.split("-");
            let a = splitted.next().unwrap();
            let b = splitted.next().unwrap();

            graph.entry(a.to_string()).or_default().push(b.to_string());
            graph.entry(b.to_string()).or_default().push(a.to_string());
        });

    let mut groups = HashSet::new();
    for (k, _) in graph.iter() {
        if k.starts_with("t") {
            group_contains_host_with_t(k, &graph, &mut groups);
        }
    }
    groups.len()
}

fn group_contains_host_with_t(key: &str, graph: &Graph, groups: &mut HashSet<Vec<String>>) {
    let neighbors = graph.get(key).unwrap();

    for i in 0..neighbors.len() {
        let i_neigh = graph.get(&(*neighbors[i])).unwrap();

        for j in i + 1..neighbors.len() {
            if i_neigh.contains(&neighbors[j]) {
                let mut v = vec![
                    key.to_string(),
                    neighbors[i].to_string(),
                    neighbors[j].to_string(),
                ];
                v.sort();
                groups.insert(v);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::execute;

    #[test]
    fn first() {
        let result = execute("inputs/twentythree_test");

        assert_eq!(result, 7);
    }
}
