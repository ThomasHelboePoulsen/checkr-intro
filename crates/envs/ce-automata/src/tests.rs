use ce_core::{Env, ValidationResult};
use crate::{Input, Output, AutomataEnv};


#[test]
fn non_equivalent_automata_simple() {
    let input = Input {
        regex: "a".to_string(),
    };

    let output = Output {
        dot: r#"
            digraph Automaton {
                rankdir=LR;
                "q0" -> "q1" [label="a"];
            }
        "#.to_string(),
    };

    match AutomataEnv::validate(&input, &output).unwrap() {
        ValidationResult::Mismatch { .. } => (),
        ValidationResult::Correct => panic!("expected mismatch, got correct"),
        ValidationResult::TimeOut => panic!("unexpected timeout"),
    }
}
