use std::collections::{HashMap, HashSet};

struct Node {
    name: String,
    distance: i32,
}

fn create_graph(s: &str) -> HashMap<String, Vec<String>> {
    let mut graph: HashMap<String, Vec<String>> = HashMap::new();

    for l in s.lines() {
        let lv: Vec<&str> = l.trim().split(")").collect();

        let parent = lv[0];
        let child = lv[1];

        match graph.get_mut(parent) {
            None => { graph.insert(String::from(parent), vec!(String::from(child))); }
            Some(entry) => entry.push(String::from(child)),
        };
    }
    graph
}

fn solve_a_impl(content: &str) {
    let (ans, _) = bfs(&create_graph(content), "COM", "");
    println!("ans = {}", ans);
}

fn solve_a() {
    let content = std::fs::read_to_string("./input/input6.txt").unwrap();
    solve_a_impl(&content);
}

fn bfs(graph: &HashMap<String, Vec<String>>, start_node: &str, end_node: &str) -> (i32, Vec<String>) {
    let mut q = std::collections::VecDeque::new();
    let mut discovered: HashSet<String> = HashSet::new();
    q.push_front(Node { name: String::from(start_node), distance: 0 });
    let mut visited_list: Vec<String> = Vec::new();

    let mut total_count = 0;

    while !q.is_empty() {
        let n = q.pop_front().unwrap();
        discovered.insert(n.name.clone());
        total_count += n.distance;
        visited_list.push(n.name.clone());

        if !graph.contains_key(&n.name) {
            continue;
        }
        for new_node in graph.get(&n.name).unwrap() {
            if new_node == end_node {
                return (total_count, visited_list);
            }
            if !discovered.contains(new_node) {
                q.push_back(Node { name: new_node.clone(), distance: n.distance + 1 });
            }
        }
    }
    (total_count, vec!())
}

fn bfs_inverted(graph: &HashMap<String, Vec<String>>) -> i32 {
    let mut invert_graph: HashMap<String, Vec<String>> = HashMap::new();

    for (k, v) in graph {
        for n in v {
            invert_graph.insert(n.clone(), vec!(k.clone()));
        }
    }

    let (_, a_nodes) = bfs(&invert_graph, "YOU", "COM");
    let (_, b_nodes) = bfs(&invert_graph, "SAN", "COM");

    let mut a_index = 0;
    let mut b_index = 0;

    let mut found = false;

    //eprintln!("a_nodes = {:#?}", a_nodes);
    //eprintln!("b_nodes = {:#?}", b_nodes);
    for a in a_nodes {
        b_index = 0;
        for b in &b_nodes {
            if &a == b {
                println!("Found: {}", b);
                found = true;
                break;
            }
            b_index += 1;
        }
        if found { break; }

        a_index += 1;
    }
    a_index + b_index - 2
}

fn solve_b_impl(content: &str) {
    let ans = bfs_inverted(&create_graph(content));
    println!("ans = {}", ans);
}


fn solve_b() {
    let content = std::fs::read_to_string("./input/input6.txt").unwrap();
    solve_b_impl(&content);
}

pub fn solve() {
    solve_a();
    solve_b();
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day6_1() {
        let t = "COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L";

        solve_a_impl(t);
    }

    #[test]
    fn test_day6_2() {
        let t = "COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L
K)YOU
I)SAN";
        solve_b_impl(t)
    }
}
