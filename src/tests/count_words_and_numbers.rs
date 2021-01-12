#![cfg(test)]
    use std::collections::HashMap;
    use crate::types::{Effector, StreamData, StatesConnection};

    use super::utils::test_valid_string;
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

    fn setup_effects() -> HashMap<StatesConnection<State>, Vec<Effect>> {
        map!(
            StatesConnection { 
                from: State::INIT,
                to: State::WORD
            } => vec![Effect::INCREMENT_WORD_COUNT],

            StatesConnection {
                from: State::INIT,
                to: State::NUMBER_IP
            } => vec![Effect::INCREMENT_NUMBER_COUNT],

            StatesConnection {
                from: State::WORD,
                to: State::NUMBER_IP
            } => vec![Effect::INCREMENT_NUMBER_COUNT],

            StatesConnection {
                from: State::NUMBER_IP,
                to: State::WORD
            } => vec![Effect::INCREMENT_WORD_COUNT],

            StatesConnection {
                from: State::NUMBER_FP,
                to: State::WORD
            } => vec![Effect::INCREMENT_WORD_COUNT],
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
        let fsm = init_fsm(Some(&effects), None);
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