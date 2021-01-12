#![cfg(test)]

use std::collections::HashMap;
use crate::types::{Effector, StreamData};
use crate::macros;
use super::utils::test_valid_string;
use super::automatas::words_and_numbers::*;

struct Store {
    words: Vec<String>,
    numbers_sum: f64,
    word_buffer: String,
    number_buffer: f64,
    precision_buffer: f64
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum BasicEffect {
    PushToWordbuffer,
    AcceptWordbuffer,
    // fpd - fraction part digit
    PushToNumbuffer {
        is_fpd: bool
    },
    AcceptNumbuffer
}

type Effect = (BasicEffect, Option<BasicEffect>);

impl Store {
    pub fn new() -> Self {
        Self {
            words: Vec::new(),
            numbers_sum: 0.0,
            word_buffer: String::new(),
            number_buffer: 0.0,
            precision_buffer: 0.1
        }
    }

    pub fn words(&self) -> &Vec<String>  {
        &self.words
    }

    pub fn numbers_sum(&self) -> f64 {
        self.numbers_sum
    }

    pub fn push_to_wordbuff(&mut self, ch: char) {
        self.word_buffer.push(ch);
    }
    
    pub fn accept_wordbuffer(&mut self) {
        let word = self.word_buffer.clone();
        self.words.push(word);
        self.word_buffer.clear();
    }

    // fpd - fraction part digit
    pub fn push_to_numbuffer(&mut self, ch: char, is_fpd: bool) {
        let digit = match ch.to_digit(10) {
            Some(digit) => digit,
            _ => panic!("Could not convert char '{}' to digit", ch)
        };

        if is_fpd {
            self.number_buffer += (digit as f64) * self.precision_buffer;
            self.precision_buffer /= 10.0; 
        } else {
            self.number_buffer = self.number_buffer * 10.0 + (digit as f64);
        }
    }

    pub fn accept_numbuffer(&mut self) {
        self.numbers_sum += self.number_buffer;
        self.number_buffer = 0.0;
        self.precision_buffer = 0.1;
    }

    pub fn apply_effect(&mut self, effect: BasicEffect, input_data: StreamData) {
        match effect {
            BasicEffect::PushToWordbuffer => self.push_to_wordbuff(input_data.character),
            BasicEffect::AcceptWordbuffer => self.accept_wordbuffer(),
            BasicEffect::PushToNumbuffer { is_fpd } => self.push_to_numbuffer(
                input_data.character, is_fpd
            ),
            BasicEffect::AcceptNumbuffer => self.accept_numbuffer()
        }
    }
}

impl Effector<Effect> for Store {
    fn dispatch(&mut self, effect: Effect, input_data: StreamData) {
        self.apply_effect(effect.0, input_data);

        if let Some(effect) = effect.1 {
            self.apply_effect(effect, input_data)
        }
    }
}

/*
fn setup_effects() -> HashMap<State, Vec<Option<Effect>>> {
    map!(
        State::INIT => vec![
            Some((BasicEffect::PushToWordbuffer, None)),
            Some((BasicEffect::PushToNumbuffer { is_fpd: false }, None))
        ],
        State::WORD => vec![
            Some((BasicEffect::PushToWordbuffer, None)),
            Some((
                BasicEffect::AcceptWordbuffer,
                Some(BasicEffect::PushToNumbuffer { is_fpd: false })
            )),
            Some((
                BasicEffect::AcceptWordbuffer,
                None
            )),
        ],
        State::NUMBER_IP => vec![
            Some(
                BasicEffect::AcceptNumbuffer,
                Some(BasicEffect::PushToWordbuffer)
            ),
        ]
    )
}
*/
