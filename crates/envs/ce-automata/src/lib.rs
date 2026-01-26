use ce_core::{Env, Generate, ValidationResult, define_env, rand};
use serde::{Deserialize, Serialize};
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
        write!(f, "{}->{} [label=\"{}\"]"
        ,self.from,self.to,self.label)
    }
}

fn automata_edges_from_word(word : &str) -> Vec<Edge> {
    let mut edges: Vec<Edge> = Vec::new();
    for (i, c) in word.chars().enumerate() {
        let base = Edge {
            from: i.to_string(),
            to: (i+1).to_string(),
            label: String::new()
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
    let mut dot = String::from(" digraph Automaton {
    rankdir=LR;");

    for edge in edges {
        dot.push_str(&edge.to_string());
    }
    
    if let Some(last) = edges.last() {
        let line = format!("{} [accepting=true];", last.to);
        dot.push_str(&line);
    }

    
    dot.push_str(&String::from("}"));

    dot
}

impl Env for AutomataEnv {
    type Input = Input;

    type Output = Output;

    type Meta = ();

    fn run(_input: &Self::Input) -> ce_core::Result<Self::Output> {
        Ok(Output {
            dot: edges_to_dot(
                &automata_edges_from_word(&_input.regex)
            ),
        })
    }

    fn validate(_input: &Self::Input, _output: &Self::Output) -> ce_core::Result<ValidationResult> {
        Ok(ValidationResult::Correct)
    }
}

impl Generate for Input {
    type Context = ();

    fn gn<R: rand::Rng>(_cx: &mut Self::Context, _rng: &mut R) -> Self {
        Self::default()
    }
}

