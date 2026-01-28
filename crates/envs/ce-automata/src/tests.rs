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


/*
    Tests:
        -explicit dead state doesnt matter
        -empty graph equivalent with itself
        -empty graph not equivalent with non empty graph
        -added states only leading to dead states do not matter for equivalence
        -Simple correct "HelloWorld" positive test
        -Casing of input doesnt matter
        -only 1 edge for non-letters
        -splits on  |
        -if input contains non-alphabet character, then must return Err

*/