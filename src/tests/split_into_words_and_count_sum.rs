#![cfg(test)]

use std::collections::HashMap;
use crate::types::{Effector, StreamData, StatesConnection};
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
    AcceptNumbuffer,
    CleanupBuffers,
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

    pub fn cleanup_buffers(&mut self) {
        if self.word_buffer.len() > 0 {
            let word = self.word_buffer.clone();
            self.words.push(word);
            self.word_buffer.clear();
        }

        self.numbers_sum += self.number_buffer;
        self.number_buffer = 0.0;
    }

    pub fn apply_effect(&mut self, effect: BasicEffect, input_data: StreamData) {
        match effect {
            BasicEffect::PushToWordbuffer => self.push_to_wordbuff(input_data.character),
            BasicEffect::AcceptWordbuffer => self.accept_wordbuffer(),
            BasicEffect::PushToNumbuffer { is_fpd } => self.push_to_numbuffer(
                input_data.character, is_fpd
            ),
            BasicEffect::AcceptNumbuffer => self.accept_numbuffer(),
            BasicEffect::CleanupBuffers => self.cleanup_buffers(),
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

fn setup_effects() -> HashMap<StatesConnection<State>, Vec<Effect>> {
    map!(
        StatesConnection {
            from: State::INIT,
            to: State::WORD
        } => vec![
            (BasicEffect::PushToWordbuffer, None)
        ],

        StatesConnection {
            from: State::INIT,
            to: State::NUMBER_IP
        } => vec![
            (BasicEffect::PushToNumbuffer { is_fpd: false }, None)
        ],

        StatesConnection {
            from: State::WORD,
            to: State::WORD
        } => vec![
            (BasicEffect::PushToWordbuffer, None)
        ],

        StatesConnection {
            from: State::WORD,
            to: State::NUMBER_IP
        } => vec![
            (
                BasicEffect::AcceptWordbuffer,
                Some(BasicEffect::PushToNumbuffer { is_fpd: false })
            )
        ],

        StatesConnection {
            from: State::WORD,
            to: State::INIT
        } => vec![
            (BasicEffect::AcceptWordbuffer, None)
        ],

        StatesConnection {
            from: State::NUMBER_IP,
            to: State::WORD
        } => vec![
            (
                BasicEffect::AcceptNumbuffer,
                Some(BasicEffect::PushToWordbuffer)
            )
        ],

        StatesConnection {
            from: State::NUMBER_IP,
            to: State::NUMBER_IP
        } => vec![
            (
                BasicEffect::PushToNumbuffer { is_fpd: false },
                None
            )
        ],

        StatesConnection {
            from: State::NUMBER_IP,
            to: State::INIT
        } => vec![
            (
                BasicEffect::AcceptNumbuffer,
                None
            )
        ],

        StatesConnection {
            from: State::NUMBER_FP,
            to: State::WORD
        } => vec![
            (
                BasicEffect::AcceptNumbuffer,
                Some(BasicEffect::PushToWordbuffer)
            )
        ],

        StatesConnection {
            from: State::NUMBER_FP,
            to: State::NUMBER_FP
        } => vec![
            (
                BasicEffect::PushToNumbuffer { is_fpd: true },
                None
            )
        ],

        StatesConnection {
            from: State::NUMBER_FP,
            to: State::INIT
        } => vec![
            (
                BasicEffect::AcceptNumbuffer,
                None
            )
        ],
    )
}

#[test]
fn it_works_correctly() {
    let effects = setup_effects();
    let fsm = init_fsm::<Effect>(
        Some(&effects), 
        Some((BasicEffect::CleanupBuffers, None))
    );
    
    {
        let mut storage = Store::new();
        let string = String::from("the quick brown fox, 123, uw");

        test_valid_string(&fsm, &string, Some(&mut storage));

        let words = storage.words();
        let sum = storage.numbers_sum();

        assert_eq!(
            words.clone(),
            vec![
                String::from("the"),
                String::from("quick"),
                String::from("brown"),
                String::from("fox"),
                String::from("uw")
            ]
        );

        assert!((sum - 123.0).abs() <= f64::EPSILON);
    }

    {
        let mut storage = Store::new();
        let string = String::from("$%$&*&,,...");

        test_valid_string(&fsm, &string, Some(&mut storage));

        let words = storage.words();
        let sum = storage.numbers_sum();

        let empty_str: Vec<String> = Vec::new();

        assert_eq!(
            words.clone(),
            empty_str
        );

        assert!((sum - 0.0).abs() <= f64::EPSILON);
    }

    {
        let mut storage = Store::new();
        let string = String::from("1, 2, 2.5, 3 - go!");

        test_valid_string(&fsm, &string, Some(&mut storage));

        let words = storage.words();
        let sum = storage.numbers_sum();

        assert_eq!(
            words.clone(),
            vec![
                String::from("go"),
            ]
        );

        assert!((sum - 8.5).abs() <= f64::EPSILON);
    }
}
