use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader},
};

type Graph = HashMap<String, Vec<String>>;

pub fn execute(file_path: &str) -> String {
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
        group_contains_host_with_t(k, &graph, &mut groups);
    }
    let longest = groups.iter().max_by(|a, b| a.len().cmp(&b.len())).unwrap();

    longest.join(",")
}

fn group_contains_host_with_t(key: &str, graph: &Graph, groups: &mut HashSet<Vec<String>>) {
    let neighbors = graph.get(key).unwrap();

    for i in 0..neighbors.len() {
        let mut grp = vec![key.to_string(), neighbors[i].to_string()];

        for j in i + 1..neighbors.len() {
            let j_neigh = graph.get(&(*neighbors[j])).unwrap();

            if grp.iter().all(|it| j_neigh.contains(it)) {
                grp.push(neighbors[j].to_string());
            }
        }

        grp.sort();
        groups.insert(grp);
    }
}
