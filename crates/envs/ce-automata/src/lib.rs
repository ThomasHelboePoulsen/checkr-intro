mod dot;
#[cfg(test)]
mod tests;

use ce_core::rand::seq::IndexedRandom;
use ce_core::{Env, Generate, ValidationResult, define_env, rand};
use petgraph::visit::EdgeRef;
use serde::{Deserialize, Serialize};
use std::collections::{HashSet,HashMap, VecDeque};
use std::fmt;

define_env!(AutomataEnv);

#[derive(tapi::Tapi, Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct Input {
    pub regex: String,
}

#[derive(tapi::Tapi, Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct Output {
    pub dot: String,
}

#[derive(Clone)]
struct Edge {
    from: String,
    to: String,
    label: String,
}
struct Node {
    name : String,
    ingoing : Vec<usize>,
    outgoing: Vec<usize>
}

impl Node {
    fn is_accepting(&self) -> bool {
        self.outgoing.len() == 0
    }

    fn is_root(&self) -> bool {
        self.ingoing.len() == 0
    }
}

impl fmt::Display for Edge {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "\"{}\"->\"{}\" [label=\"{}\"];",
            self.from, self.to, self.label
        )
    }
}

fn automata_edges_from_word(word: &str, id: usize) -> Vec<Edge> {
    let mut edges: Vec<Edge> = Vec::new();
    for (i, c) in word.chars().enumerate() {
        let base = Edge {
            from: (if i > 0 {
                format!("{}-{}", id, i)
            } else {
                i.to_string()
            }),
            to: format!("{}-{}", id, (i + 1)),
            label: String::new(),
        };
        let upper = c.to_string().to_uppercase();
        let lower = c.to_string().to_lowercase();
        if lower == upper {
            edges.push(Edge {
                label: upper,
                ..base.clone()
            });
            continue;
        }
        edges.push(Edge {
            label: upper,
            ..base.clone()
        });
        edges.push(Edge {
            label: lower,
            ..base
        });
    }
    edges
}

fn edges_to_dot(edges: &[Edge]) -> String {
    let mut dot = String::from(
        " digraph Automaton {
    rankdir=LR;",
    );

    for edge in edges {
        dot.push_str(&edge.to_string());
    }

    let mut from_nodes: HashSet<String> = HashSet::new();
    let mut to_nodes: HashSet<String> = HashSet::new();

    for edge in edges {
        to_nodes.insert(edge.to.clone());
        from_nodes.insert(edge.from.clone());
    }
    for accepting_node in to_nodes.difference(&from_nodes).into_iter() {
        let line = format!("\"{}\" [accepting=true];", accepting_node);
        dot.push_str(&line);
    }

    dot.push_str(&String::from("}"));

    dot
}

fn split_and_collect(regex: &str) -> Vec<Edge> {
    let mut edges: Vec<Edge> = Vec::new();
    let words: Vec<&str> = regex.split("|").collect();

    for (i, word) in words.iter().enumerate() {
        edges.extend(automata_edges_from_word(word, i));
    }
    edges
}


impl Env for AutomataEnv {
    type Input = Input;

    type Output = Output;

    type Meta = ();

    fn run(_input: &Self::Input) -> ce_core::Result<Self::Output> {
        Ok(Output {
            dot: edges_to_dot(&split_and_collect(&_input.regex)),
        })
    }



    fn validate(_input: &Self::Input, _output: &Self::Output) -> ce_core::Result<ValidationResult> {
        let t_g = match dot::dot_to_petgraph(&_output.dot) {
            Ok(t_g) => t_g,
            Err(err) => {
                return Ok(ValidationResult::Mismatch {
                    reason: format!("failed to parse dot: {err}"),
                });
            }
        };
        let actual_edges: Vec<Edge> = t_g.edges();
        let actual_nodes = derive_nodes(&actual_edges);
        let actual_start_idx = match actual_nodes.iter().position(|n| n.is_root()) {
            Some(idx) => idx,
            None => {
                return Ok(ValidationResult::Mismatch {
                    reason: String::from("No initial state in actual solution"),
                });
            }
        };
        
        let reference_edges = split_and_collect(&_input.regex);
        let reference_nodes = derive_nodes(&reference_edges);
        let reference_start_idx = match actual_nodes.iter().position(|n| n.is_root()) {
            Some(idx) => idx,
            None => {
                return Ok(ValidationResult::Mismatch {
                    reason: String::from("No initial state in reference solution"),
                });
            }
        };

        if equivalent(&actual_nodes,&actual_edges,&reference_nodes,&reference_edges,actual_start_idx,reference_start_idx) {
            Ok(ValidationResult::Correct)
        } else {
            Ok(ValidationResult::Mismatch { reason: "Not equivalent".to_string() })
        }
    }
}

impl Generate for Input {
    type Context = ();

    fn gn<R: rand::Rng>(_cx: &mut Self::Context, _rng: &mut R) -> Self {
        let alphabet = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890";
        let mut regex = String::new();
        let words: usize = _rng.random_range(1..11);
        for i in 0..words {
            let word_len = _rng.random_range(1..20);
            for _ in 0..word_len {
                regex.push(alphabet.choose(_rng).unwrap().clone() as char);
            }
            if i != words-1 {
                regex.push('|');
            }
        }
        Input { regex }
    }
}

fn equivalent(
    nodes1: &[Node],
    edges1: &[Edge],
    nodes2: &[Node],
    edges2: &[Edge],
    start1: usize,
    start2: usize,
) -> bool {
    let live1 = compute_live(nodes1, edges1);
    let live2 = compute_live(nodes2, edges2);
    
    let alphabet: HashSet<String> =
        alphabet(edges1).union(&alphabet(edges2)).cloned().collect();

    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();

    queue.push_back((start1, start2));
    visited.insert((start1, start2));

    while let Some((s1, s2)) = queue.pop_front() {
        if (nodes1[s1].is_accepting() && live1[s1]) 
            != (nodes2[s2].is_accepting() && live2[s2]) {
            return false;
        }

        for label in &alphabet {
            let t1 = transition(nodes1, edges1, s1, label).filter(|&s| live1[s]);
            let t2 = transition(nodes2, edges2, s2, label).filter(|&s| live2[s]);

            match (t1, t2) {
                (Some(a), Some(b)) => {
                    if visited.insert((a, b)) {
                        queue.push_back((a, b));
                    }
                }
                (None, None) => {}
                _ => return false,
            }
        }
    }

    true
}

fn derive_nodes(edges: &[Edge]) -> Vec<Node> {
    let mut nodes: Vec<Node> = Vec::new();
    let mut index_of: HashMap<String, usize> = HashMap::new();

    fn get_or_create(
        name: &str,
        nodes: &mut Vec<Node>,
        index_of: &mut HashMap<String, usize>,
    ) -> usize {
        if let Some(&idx) = index_of.get(name) {
            return idx;
        }

        let idx = nodes.len();
        nodes.push(Node {
            name: name.to_string(),
            ingoing: Vec::new(),
            outgoing: Vec::new(),
        });
        index_of.insert(name.to_string(), idx);
        return idx;
    }

    for (edge_id, edge) in edges.iter().enumerate() {
        let from_idx = get_or_create(&edge.from, &mut nodes, &mut index_of);
        let to_idx = get_or_create(&edge.to, &mut nodes, &mut index_of);

        nodes[from_idx].outgoing.push(edge_id);
        nodes[to_idx].ingoing.push(edge_id);
    }

    nodes
}


fn transition(
    nodes: &[Node],
    edges: &[Edge],
    node_idx: usize,
    label: &str,
) -> Option<usize> {
    for &e_idx in &nodes[node_idx].outgoing {
        let e = &edges[e_idx];
        if e.label == label {
            return nodes.iter().position(|n| n.name == e.to);
        }
    }
    None
}

fn alphabet(edges: &[Edge]) -> HashSet<String> {
    edges.iter().map(|e| e.label.clone()).collect()
}




impl dot::ParsedGraph {
    pub fn edges(&self) -> Vec<Edge> {
        self.graph
            .edge_references()
            .map(|e| {
                let from = &self.graph[e.source()];
                let to = &self.graph[e.target()];
                let label = e.weight().to_string(); // assuming Action: Display

                Edge {
                    from: from.clone(),
                    to: to.clone(),
                    label,
                }
            })
            .collect()
    }
}

fn compute_live(nodes: &[Node], edges: &[Edge]) -> Vec<bool> {
    let mut live = vec![false; nodes.len()];
    let mut stack = Vec::new();

    for (i, n) in nodes.iter().enumerate() {
        if n.is_accepting() {
            live[i] = true;
            stack.push(i);
        }
    }

    while let Some(s) = stack.pop() {
        for &e in &nodes[s].ingoing {
            let from = nodes.iter().position(|n| n.name == edges[e].from).unwrap();
            if !live[from] {
                live[from] = true;
                stack.push(from);
            }
        }
    }

    live
}
