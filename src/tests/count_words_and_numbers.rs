#![cfg(test)]

use std::hash::Hash;
    use std::fmt::Debug;
    use std::collections::HashMap;
    use crate::macros;
    use crate::fsm::{FSM, FSMError};
    use crate::types::{Transition, Effector, StreamData};

    use super::utils::{test_invalid_string, test_valid_string, is_letter, is_digit};
    use super::automatas::words_and_numbers::*;

    struct Counter {
        word_count: usize,
        number_count: usize
    }

    #[derive(PartialEq, Eq, Clone, Copy)]
    enum Effect {
        INCREMENT_WORD_COUNT,
        INCREMENT_NUMBER_COUNT
    }

    impl Counter {
        fn increment_word_count(&mut self) {
            self.word_count += 1;
        }

        fn increment_number_count(&mut self) {
            self.number_count += 1;
        }

        fn word_count(&self) -> usize {
            self.word_count
        }

        fn number_count(&self) -> usize {
            self.number_count
        }

        fn clear_counts(&mut self) {
            self.word_count = 0;
            self.number_count = 0;
        }
    }

    impl Effector<Effect> for Counter {
        fn dispatch(&mut self, effect: Effect, _data: StreamData) {
            match effect {
                Effect::INCREMENT_WORD_COUNT => self.increment_word_count(),
                Effect::INCREMENT_NUMBER_COUNT => self.increment_number_count()
            }
        }
    }

    fn setup_effects() -> HashMap<State, Vec<Option<Effect>>> {
        map!(
            State::INIT => vec![Some(Effect::INCREMENT_WORD_COUNT), Some(Effect::INCREMENT_NUMBER_COUNT)],
            State::WORD => vec![None, Some(Effect::INCREMENT_NUMBER_COUNT)],
            State::NUMBER_IP => vec![Some(Effect::INCREMENT_WORD_COUNT)],
            State::NUMBER_FP => vec![Some(Effect::INCREMENT_WORD_COUNT)]
        )
    }

    fn setup_counter() -> Counter {
        Counter {
            word_count: 0,
            number_count: 0
        }
    }

    #[test]
    fn it_counts_numbers_and_words_correctly() {
        let effects = setup_effects();
        let fsm = init_fsm(Some(&effects));
        let mut counter = setup_counter();

        {
            let string = String::from("the123fox jumps,,,,");

            test_valid_string(
                &fsm,
                &string,
                Some(&mut counter)
            );

            let word_count = counter.word_count();
            let number_count = counter.number_count();

            assert_eq!(
                word_count,
                3
            );

            assert_eq!(
                number_count,
                1
            );

            counter.clear_counts();
        }

        {
            let string = String::from("!@#..,.?");

            test_valid_string(
                &fsm,
                &string,
                Some(&mut counter)
            );

            let word_count = counter.word_count();
            let number_count = counter.number_count();

            assert_eq!(
                word_count,
                0
            );

            assert_eq!(
                number_count,
                0
            );

            counter.clear_counts();
        }

        {
            let string = String::from("Add 1.5 pinches of salt and 2 cups of water!");

            test_valid_string(
                &fsm,
                &string,
                Some(&mut counter)
            );

            let word_count = counter.word_count();
            let number_count = counter.number_count();

            assert_eq!(
                word_count,
                8
            );

            assert_eq!(
                number_count,
                2
            );

            counter.clear_counts();
        }
    }