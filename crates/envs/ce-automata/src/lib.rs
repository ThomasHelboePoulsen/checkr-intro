use ce_core::rand::seq::IndexedRandom;
use ce_core::{Env, Generate, ValidationResult, define_env, rand};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
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

impl fmt::Display for Edge {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "\"{}\"->\"{}\" [label=\"{}\"]",
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
        Ok(ValidationResult::Correct)
    }
}

impl Generate for Input {
    type Context = ();

    fn gn<R: rand::Rng>(_cx: &mut Self::Context, _rng: &mut R) -> Self {
        let alphabet = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890 -!\"";
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
