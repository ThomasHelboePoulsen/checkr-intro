use crate::{AutomataEnv, Input, Output};
use ce_core::{Env, ValidationResult};

#[test]
fn non_equivalent_automata_simple() {
    let input = Input {
        regex: "a".to_string(),
    };

    let output = Output {
        dot: r#"
            digraph Automaton {
                rankdir=LR;
                "0" -> "0-1" [label="b"];
                "0" -> "0-1" [label="B"];
            }
        "#
        .to_string(),
    };

    match AutomataEnv::validate(&input, &output).unwrap() {
        ValidationResult::Mismatch { .. } => (), // expected
        ValidationResult::Correct => panic!("expected mismatch, got correct"),
        ValidationResult::TimeOut => panic!("unexpected timeout"),
    }
}

#[test] //TODO
fn dead_end_equivalence() {
    let input = Input {
        regex: "abb".to_string(),
    };
    let output = Output {
        dot: r#"
            digraph Automaton {
                rankdir=LR;
                "0" -> "0-1" [label="a"];
                "0" -> "0-1" [label="A"];
                "0-1" -> "0-2" [label="b"];
                "0-1" -> "0-2" [label="B"];
                "0-2" -> "0-3" [label="b"];
                "0-2" -> "0-3" [label="B"];
                "0" -> "k" [label="A"];
            }
        "#
        .to_string(),
    };

    match AutomataEnv::validate(&input, &output).unwrap() {
        ValidationResult::Correct => (),
        ValidationResult::Mismatch { reason } => panic!("unexpected mismatch: {}", reason),
        ValidationResult::TimeOut => panic!("unexpected timeout"),
    }
}

#[test]
fn empty_graph_non_empty_graph_non_equivalence() {
    let input = Input {
        regex: "abc".to_string(),
    };
    let output = Output {
        dot: r#"
 digraph Automaton {
    rankdir=LR;}
        "#
        .to_string(),
    };

    match AutomataEnv::validate(&input, &output).unwrap() {
        ValidationResult::Correct => panic!("unexpected match"),
        ValidationResult::Mismatch { .. } => (),
        ValidationResult::TimeOut => panic!("unexpected timeout"),
    }
}

#[test]
fn empty_graph_equivalence() {
    let input = Input {
        regex: "".to_string(),
    };

    let output = Output {
        dot: r#"
            digraph Automaton {
                rankdir=LR;
            }
        "#
        .to_string(),
    };

    match AutomataEnv::validate(&input, &output).unwrap() {
        ValidationResult::Correct => (),
        ValidationResult::Mismatch { reason } => panic!("unexpected mismatch: {}", reason),
        ValidationResult::TimeOut => panic!("unexpected timeout"),
    }
}

#[test]
fn explicit_dead_state_doesnt_matter() {
    let input = Input {
        regex: "a".to_string(),
    };

    let output = Output {
        dot: r#"
            digraph Automaton {
            "0" -> "0-1" [label = "a"]
            "0" -> "0-1" [label = "A"]
            "dead"
                rankdir=LR;
            }
        "#
        .to_string(),
    };

    match AutomataEnv::validate(&input, &output).unwrap() {
        ValidationResult::Correct => (),
        ValidationResult::Mismatch { reason } => panic!("unexpected mismatch: {}", reason),
        ValidationResult::TimeOut => panic!("unexpected timeout"),
    }
}

#[test]
fn hello_world_graph_equivalence_pass() {
    let input = Input {
        regex: "HelloWorld".to_string(),
    };

    let output = Output {
        dot: r#"
            digraph Automaton {
                rankdir=LR;
                "0" -> "0-1" [label="h"];
                "0" -> "0-1" [label="H"];
                "0-1" -> "0-2" [label="e"];
                "0-1" -> "0-2" [label="E"];
                "0-2" -> "0-3" [label="l"];
                "0-2" -> "0-3" [label="L"];
                "0-3" -> "0-4" [label="l"];
                "0-3" -> "0-4" [label="L"];
                "0-4" -> "0-5" [label="o"];
                "0-4" -> "0-5" [label="O"];
                "0-5" -> "0-6" [label="w"];
                "0-5" -> "0-6" [label="W"];
                "0-6" -> "0-7" [label="o"];
                "0-6" -> "0-7" [label="O"];
                "0-7" -> "0-8" [label="r"];
                "0-7" -> "0-8" [label="R"];
                "0-8" -> "0-9" [label="l"];
                "0-8" -> "0-9" [label="L"];
                "0-9" -> "0-10" [label="d"];
                "0-9" -> "0-10" [label="D"];

            }
        "#
        .to_string(),
    };

    match AutomataEnv::validate(&input, &output).unwrap() {
        ValidationResult::Correct => (),
        ValidationResult::Mismatch { reason } => panic!("unexpected mismatch: {}", reason),
        ValidationResult::TimeOut => panic!("unexpected timeout"),
    }
}

#[test]
fn hello_world_graph_equivalence_fail() {
    let input = Input {
        regex: "HelloWorld".to_string(),
    };

    // Final label as number to force mismatch
    let output = Output {
        dot: r#"
            digraph Automaton {
                rankdir=LR;
                "0" -> "0-1" [label="h"];
                "0" -> "0-1" [label="H"];
                "0-1" -> "0-2" [label="e"];
                "0-1" -> "0-2" [label="E"];
                "0-2" -> "0-3" [label="l"];
                "0-2" -> "0-3" [label="L"];
                "0-3" -> "0-4" [label="l"];
                "0-3" -> "0-4" [label="L"];
                "0-4" -> "0-5" [label="o"];
                "0-4" -> "0-5" [label="O"];
                "0-5" -> "0-6" [label="w"];
                "0-5" -> "0-6" [label="W"];
                "0-6" -> "0-7" [label="o"];
                "0-6" -> "0-7" [label="O"];
                "0-7" -> "0-8" [label="r"];
                "0-7" -> "0-8" [label="R"];
                "0-8" -> "0-9" [label="l"];
                "0-8" -> "0-9" [label="L"];
                "0-9" -> "0-10" [label="d"];
                "0-9" -> "0-10" [label="1"]; 

            }
        "#
        .to_string(),
    };

    match AutomataEnv::validate(&input, &output).unwrap() {
        ValidationResult::Correct => panic!("unexpected mismatch"),
        ValidationResult::Mismatch { .. } => (),
        ValidationResult::TimeOut => panic!("unexpected timeout"),
    }
}

#[test]
fn split_on_bar() {
    let input = Input {
        regex: "a|b".to_string(),
    };

    let output = Output {
        dot: r#"
 digraph Automaton {
    rankdir=LR;
    "0"->"0-1" [label="A"];
    "0"->"0-1" [label="a"];
    "0"->"1-1" [label="B"];
    "0"->"1-1" [label="b"];
    }
        "#
        .to_string(),
    };

    match AutomataEnv::validate(&input, &output).unwrap() {
        ValidationResult::Correct => (),
        ValidationResult::Mismatch { reason } => panic!("unexpected mismatch: {}", reason),
        ValidationResult::TimeOut => panic!("unexpected timeout"),
    }
}

#[test]
fn invalid_character_fails() {
    let input = Input {
        regex: "a+b".to_string(),
    };

    let result = AutomataEnv::run(&input).unwrap();

    assert!(
        result
            .dot
            .contains("Only alhpanumerical chars and | is allowed"),
        "Expected error message, got: {}",
        result.dot
    );
}

#[test]
fn casing_does_not_matter() {
    let input = Input {
        regex: "aA".to_string(),
    };

    // This DOT represents the same automaton as "ab", but with uppercase letters
    let output = Output {
        dot: r#"
            digraph Automaton {
                rankdir=LR;
                "0" -> "0-1" [label="A"];
                "0" -> "0-1" [label="a"];
                "0-1" -> "0-2" [label="A"];
                "0-1" -> "0-2" [label="a"];
            }
        "#
        .to_string(),
    };

    match AutomataEnv::validate(&input, &output).unwrap() {
        ValidationResult::Correct => (),
        ValidationResult::Mismatch { reason } => {
            panic!("expected equivalence, got mismatch: {}", reason)
        }
        ValidationResult::TimeOut => panic!("unexpected timeout"),
    }
}

/*
    Tests:
        -explicit dead state doesnt matter --- DONE
        -empty graph equivalent with itself ---DONE
        -empty graph not equivalent with non empty graph ---DONE
        -added states only leading to dead states do not matter for equivalence --DONE
        -Simple correct "HelloWorld" positive test --- DONE
        -Casing of input doesnt matter --- DONE
        -splits on  | --- DONE
        -if input contains non-alphabet character, then must return Err ---DONE

*/
