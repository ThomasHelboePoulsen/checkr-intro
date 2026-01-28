use ce_core::rand::prelude::IndexedRandom;
use ce_core::{Env, Generate, ValidationResult, define_env, rand};
use serde::{Deserialize, Serialize};

define_env!(HelloWorldEnv);

#[derive(tapi::Tapi, Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct Input {
    pub source: String,
}

#[derive(tapi::Tapi, Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct Output {
    pub transformed: String,
}

impl Env for HelloWorldEnv {
    type Input = Input;
    type Output = Output;
    type Meta = ();

    fn run(_input: &Self::Input) -> ce_core::Result<Self::Output> {
        let words: Vec<String> = _input
            .source
            .split_whitespace()
            .enumerate()
            .map(|(i, w)| {
                if i % 2 == 1 {
                    w.to_uppercase()
                } else {
                    w.to_string()
                }
            })
            .collect();

        Ok(Output {
            transformed: words.join(" "),
        })
    }

    fn validate(_input: &Self::Input, _output: &Self::Output) -> ce_core::Result<ValidationResult> {
        let expected = Self::run(_input)?.transformed;
        if expected == _output.transformed {
            Ok(ValidationResult::Correct)
        } else {
            Ok(ValidationResult::Mismatch {
                reason: format!("Expected '{}', got '{}'", expected, _output.transformed),
            })
        }
    }
}

impl Generate for Input {
    type Context = ();

    fn gn<R: rand::Rng>(_cx: &mut Self::Context, _rng: &mut R) -> Self {
        let words = [
            "hello",
            "world",
            "i",
            "fun",
            "love",
            "rust",
            "code",
            "cake",
            "svelte",
            "typescript",
            "css",
            "like",
            "is",
            "cool",
            "Thomas",
            "Janus",
        ];

        let n_words = _rng.random_range(3..100);

        let sentence = (0..n_words)
            .map(|_| words.choose(_rng).unwrap())
            .cloned()
            .collect::<Vec<&str>>()
            .join(" ");

        Input { source: sentence }
    }
}
