use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;

#[derive(Debug)]
enum NodeType {
    Start,
    End,
    Small(String),
    Big(String),
}

impl NodeType {
    fn from(name: &str) -> NodeType {
        if name == "start" {
            NodeType::Start
        } else if name == "end" {
            NodeType::End
        } else if name.to_lowercase() == name {
            NodeType::Small(String::from(name))
        } else {
            NodeType::Big(String::from(name))
        }
    }
}

type NodePtr = Rc<RefCell<Node>>;

#[derive(Debug)]
struct Node {
    name: NodeType,
    edges: Vec<NodePtr>,
}

impl Node {
    fn from(name: &str) -> Node {
        Node {
            name: NodeType::from(name),
            edges: Vec::new(),
        }
    }
}

fn main() {
    let mut nodes: HashMap<String, NodePtr> = HashMap::new();

    loop {
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("failed to read from stdin");

        if input.trim().len() == 0 {
            break;
        }

        let edge = input.trim().split_once("-").expect("invalid input");
        let node_from = match nodes.get(edge.0) {
            Some(node) => node.clone(),
            None => {
                let node = Node::from(edge.0);
                nodes.insert(edge.0.to_string(), Rc::new(RefCell::new(node)));
                nodes.get(edge.0).unwrap().clone()
            }
        };
        let node_to = match nodes.get(edge.1) {
            Some(node) => node.clone(),
            None => {
                let node = Node::from(edge.1);
                nodes.insert(edge.1.to_string(), Rc::new(RefCell::new(node)));
                nodes.get(edge.1).unwrap().clone()
            }
        };

        node_from.borrow_mut().edges.push(node_to.clone());
        node_to.borrow_mut().edges.push(node_from.clone());
    }

    let start_node = nodes
        .values()
        .into_iter()
        .find(|node| {
            if let NodeType::Start = node.as_ref().borrow().name {
                true
            } else {
                false
            }
        })
        .unwrap();

    let path_count = count_paths(start_node.clone(), &mut HashSet::new(), None);

    let path_count_with_duplicate: i32 = nodes
        .values()
        .into_iter()
        .filter_map(|node| {
            if let NodeType::Small(name) = &node.as_ref().borrow().name {
                Some(name.clone())
            } else {
                None
            }
        })
        .map(|small_cave| {
            count_paths(start_node.clone(), &mut HashSet::new(), Some(small_cave)) - path_count
        })
        .sum();

    println!("Part 1: {}", path_count);
    println!("Part 2: {}", path_count_with_duplicate + path_count);
}

fn count_paths(
    start: NodePtr,
    small_caves_visited: &mut HashSet<String>,
    duplicable_path: Option<String>,
) -> i32 {
    let edges = &start.as_ref().borrow().edges;
    let mut available_paths = 0;

    for edge_ptr in edges {
        let edge = edge_ptr.as_ref().borrow();

        match &edge.name {
            NodeType::Start => continue,
            NodeType::End => {
                available_paths += 1;
            }
            NodeType::Small(name) => {
                let mut new_small_caves_visited = small_caves_visited.clone();
                let skip_set_verification = match &duplicable_path {
                    None => false,
                    Some(s) => s == name,
                };
                if skip_set_verification || new_small_caves_visited.insert(name.clone()) {
                    available_paths += count_paths(
                        edge_ptr.clone(),
                        &mut new_small_caves_visited,
                        if skip_set_verification {
                            None
                        } else {
                            duplicable_path.clone()
                        },
                    );
                }
            }
            NodeType::Big(_) => {
                available_paths += count_paths(
                    edge_ptr.clone(),
                    small_caves_visited,
                    duplicable_path.clone(),
                );
            }
        }
    }

    available_paths
}
