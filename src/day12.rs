use crate::{answer, common::Answer};
use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt;
use std::rc::Rc;

pub const ANSWER: Answer = answer!(10, 36);

/* Impl */

#[derive(Eq, PartialEq, Hash, Debug)]
struct Node {
    name: String,
    large: bool,
}

#[derive(Eq, PartialEq, Hash)]
struct Edge {
    left: Rc<Node>,
    right: Rc<Node>,
}

struct Graph {
    nodes: HashSet<Rc<Node>>,
    edges: Vec<Rc<Edge>>,
}

impl fmt::Debug for Graph {
    #[allow(clippy::write_with_newline)]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "Graph of nodes: {}, edges: {}\ngraph g {{",
            self.nodes.len(),
            self.edges.len()
        )
        .unwrap();
        for edge in &self.edges {
            writeln!(f, " {} -- {};", edge.left.name, edge.right.name).unwrap();
        }
        writeln!(f, "}}")
    }
}

fn make_node(graph: &Graph, name: &str) -> Rc<Node> {
    let node: Rc<Node> = match graph.nodes.iter().find(|n| n.name == name) {
        Some(found) => Rc::clone(found),
        None => Rc::new(Node {
            name: name.to_string(),
            large: name.to_uppercase() == name,
        }),
    };
    node
}

fn get_node(graph: &Graph, name: &str) -> Rc<Node> {
    graph.nodes.iter().find(|n| n.name == name).unwrap().clone()
}

fn make_edge(graph: &Graph, left: &str, right: &str) -> Edge {
    Edge {
        left: get_node(graph, left),
        right: get_node(graph, right),
    }
}

fn make_graph(lines: Vec<String>) -> Graph {
    let mut graph = Graph {
        nodes: HashSet::new(),
        edges: vec![],
    };
    for line in &lines {
        let (a, b) = line.split_once('-').unwrap();
        graph.nodes.insert(make_node(&graph, a));
        graph.nodes.insert(make_node(&graph, b));
        let edge = make_edge(&graph, a, b);
        graph.edges.push(Rc::new(edge));
    }
    graph
}

struct Path {
    path: Vec<Rc<Node>>,
}

impl fmt::Debug for Path {
    #[allow(clippy::write_with_newline)]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Path {{").unwrap();
        for node in &self.path {
            write!(f, "{}, ", node.name).unwrap();
        }
        write!(f, "}}")
    }
}

//fn find_adjacent_edges(graph: &Graph, node: Rc<Node>) -> Vec<Rc<Edge>> {
//graph
//.edges
//.iter()
//.filter(|e| e.left == node || e.right == node)
//.map(|x| x.clone())
//.collect()
//}

/// Classic DFS
#[allow(clippy::too_many_arguments)]
fn _dfs(
    graph: &Graph,
    path: &mut Path,
    paths: &mut Vec<Path>,
    visited_nodes: &mut Vec<Rc<Node>>,
    visited_edges: &mut Vec<Rc<Edge>>,
    back_edges: &mut Vec<Rc<Edge>>,
    node: Rc<Node>,
    end: Rc<Node>,
) {
    println!("DFS called at {}, path is {:?}", node.name, path.path);
    if end == node {
        paths.push(Path {
            path: path.path.clone(),
        });
    }

    let adj_unexplored_edges: Vec<Rc<Edge>> = graph
        .edges
        .iter()
        .filter(|e| e.left == node || e.right == node)
        //.filter(|e| visited_edges.iter().find( |x| e == x ).is_none() )
        .cloned()
        .collect();
    for rc_edge in adj_unexplored_edges {
        let edge = &*rc_edge; // Rc::get_mut(rc_edge).unwrap();
        let another_node: &Rc<Node> = if node == edge.left {
            &edge.right
        } else {
            &edge.left
        };
        let is_node_unexplored = !visited_nodes.iter().any(|x| x == another_node);
        if is_node_unexplored {
            println!(" look to edge from {} to {}", node.name, another_node.name);
            visited_nodes.push(another_node.clone());
            visited_edges.push(rc_edge.clone());
            path.path.push(another_node.clone());

            _dfs(
                graph,
                path,
                paths,
                visited_nodes,
                visited_edges,
                back_edges,
                another_node.clone(),
                end.clone(),
            );

            path.path.pop();
            //visited_nodes.pop();
            //visited_edges.pop();
        } else {
            back_edges.push(rc_edge.clone());
        }
    }
}

/// Invalid approach
#[allow(dead_code)]
fn is_cycle(path: &Path) -> bool {
    let n = path.path.len();
    if n <= 1 {
        return false;
    }
    for i in 0..n - 1 {
        for j in i + 1..n - 1 {
            if path.path[i] == path.path[j] && path.path[i + 1] == path.path[j + 1] {
                return true;
            }
        }
    }
    /*
    let mut visited_nodes : Vec<Rc<Node>> = Vec::new();
    for node in &path.path {
        if visited_nodes.iter().find(|x| x.name == node.name ).is_some() {
            return true;
        }
        visited_nodes.push(node.clone());
    }*/
    false
    /*
    let mut tmp_path = Path{ path: vec![] };
    let mut paths = Vec::new();
    let mut visited_nodes = Vec::new();
    let mut visited_edges = Vec::new();
    let mut back_edges = Vec::new();
    dfs(graph, &mut tmp_path, &mut paths, &mut visited_nodes, &mut visited_edges, &mut back_edges,
        path.path[0].clone(), path.path.last().unwrap().clone());
    println!("For path {:?} back edges {}", path.path, back_edges.len());
    // graph has a cycle if DFS found back edges
    !back_edges.is_empty()*/
}

// Specific path score functions
//
#[derive(PartialEq)]
enum EvalPathKind {
    Pre,
    Post,
}

type EvalPath = fn(&Path, EvalPathKind) -> bool;

fn eval_path_a(path: &Path, _kind: EvalPathKind) -> bool {
    let mut counter: HashMap<Rc<Node>, u32> = HashMap::new();
    for node in &path.path {
        let prev: u32 = match counter.get(node) {
            Some(x) => *x,
            None => 0,
        };
        counter.insert(node.clone(), prev + 1);
    }
    for node in &path.path {
        let count = counter[node];
        if !node.large && count > 1 {
            return false;
        }
    }
    true
}

fn eval_path_b(path: &Path, kind: EvalPathKind) -> bool {
    let mut counter: HashMap<&String, u32> = HashMap::with_capacity(path.path.len());
    for node in &path.path {
        let name = &node.name;
        let prev: u32 = match counter.get(&name) {
            Some(x) => *x,
            None => 0,
        };
        counter.insert(&name, prev + 1);
    }
    for node in &path.path {
        let count = counter[&node.name];
        if node.name == "start" && count > 1 {
            return false;
        }
        if node.name == "end" && count > 1 {
            return false;
        }
        if !node.large && count > 2 {
            return false;
        }
        if !node.large && kind == EvalPathKind::Post && count == 2 {
            // remaining must be 1
            for node1 in &path.path {
                if !node1.large && node1 != node && counter[&node1.name] > 1 {
                    return false;
                }
            }
        }
    }
    true
}

fn find_all_paths(
    graph: &Graph,
    path: &mut Path,
    paths: &mut Vec<Path>,
    node: Rc<Node>,
    end: Rc<Node>,
    eval_path: EvalPath,
) {
    //println!("DFS called at {}, path is {:?}", node.name, path.path);

    if !eval_path(path, EvalPathKind::Pre) {
        //println!(" skip path");
        return;
    }

    if end == node {
        if eval_path(path, EvalPathKind::Post) {
            println!(
                "Register path {} of len {:<2}: {:?}",
                paths.len(),
                path.path.len(),
                path
            );
            paths.push(Path {
                path: path.path.clone(),
            });
        }
        return;
    }

    let adj_edges = graph
        .edges
        .iter()
        .filter(|e| e.left == node || e.right == node)
        .cloned();
    //        .collect();
    for rc_edge in adj_edges {
        let edge = &*rc_edge;
        let another_node: &Rc<Node> = if node == edge.left {
            &edge.right
        } else {
            &edge.left
        };
        //println!(" look to edge from {} to {}", node.name, another_node.name);

        path.path.push(another_node.clone());
        find_all_paths(
            &graph,
            path,
            paths,
            another_node.clone(),
            end.clone(),
            eval_path,
        );
        path.path.pop();
    }
}

fn enumerate_paths(graph: &Graph, eval_path: EvalPath) -> Vec<Path> {
    let start = get_node(graph, "start");
    let end = get_node(graph, "end");
    let mut paths = Vec::new();
    let mut path = Path {
        path: vec![start.clone()],
    };
    find_all_paths(&graph, &mut path, &mut paths, start, end, eval_path);
    for path in &paths {
        println!(" path {:?}", path.path);
    }
    paths
}

pub fn process_a(lines: Vec<String>) -> u64 {
    let graph = &make_graph(lines);
    let paths = enumerate_paths(graph, eval_path_a);
    for path in &paths {
        println!("Path is {:?}", path);
    }
    paths.len() as u64
}

pub fn process_b(lines: Vec<String>) -> u64 {
    let graph = &make_graph(lines);
    let paths = enumerate_paths(graph, eval_path_b);
    for path in &paths {
        println!("Path is {:?}", path);
    }
    paths.len() as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    const G1: &'static str = r#"start-A
start-b
A-c
A-b
b-d
A-end
b-end
"#;

    const G2: &'static str = r#"dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc
"#;

    const G3: &'static str = r#"fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW
"#;

    fn make_test_graph(gs: &'static str) -> Graph {
        let lines: Vec<String> = gs.lines().map(String::from).collect();
        make_graph(lines)
    }

    fn make_path(graph: &Graph, spath: &str) -> Path {
        Path {
            path: spath
                .split_terminator(',')
                .map(|word| get_node(&graph, word.trim()))
                .collect(),
        }
    }

    #[test]
    fn test_fmt() {
        let graph = make_test_graph(G1);
        let s = format!("{:?}", graph);
        assert!(s.contains("graph"));
        assert!(s.contains("--"));
    }

    #[test]
    fn test_cycle() {
        let graph = make_test_graph(G1);
        assert_eq!(is_cycle(&make_path(&graph, "start,A")), false);
        assert_eq!(is_cycle(&make_path(&graph, "start,A,b,end")), false);
        assert_eq!(is_cycle(&make_path(&graph, "start,start")), false);
        assert_eq!(is_cycle(&make_path(&graph, "start,A,A")), false);
        assert_eq!(is_cycle(&Path { path: vec![] }), false);

        assert_eq!(is_cycle(&make_path(&graph, "start,A,b,end,A,b,d")), true);
        assert_eq!(is_cycle(&make_path(&graph, "start,A,b,A,d")), false);
    }

    #[test]
    fn test_paths_a1() {
        let graph = make_test_graph(G1);
        let paths = enumerate_paths(&graph, eval_path_a);
        assert_eq!(paths.len(), 10);
    }

    #[test]
    fn test_paths_a2() {
        let graph = make_test_graph(G2);
        let paths = enumerate_paths(&graph, eval_path_a);
        assert_eq!(paths.len(), 19);
    }

    #[test]
    fn test_paths_a3() {
        let graph = make_test_graph(G3);
        let paths = enumerate_paths(&graph, eval_path_a);
        assert_eq!(paths.len(), 226);
    }

    #[test]
    fn test_paths_b1() {
        let graph = make_test_graph(G1);
        let paths = enumerate_paths(&graph, eval_path_b);
        assert_eq!(paths.len(), 36);
    }

    #[test]
    fn test_paths_b2() {
        let graph = make_test_graph(G2);
        let paths = enumerate_paths(&graph, eval_path_b);
        assert_eq!(paths.len(), 103);
    }

    #[test]
    fn test_paths_b3() {
        let graph = make_test_graph(G3);
        let paths = enumerate_paths(&graph, eval_path_b);
        assert_eq!(paths.len(), 3509);
    }
}
