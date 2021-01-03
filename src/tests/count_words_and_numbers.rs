#![cfg(test)]

use std::hash::Hash;
    use std::fmt::Debug;
    use crate::macros;
    use crate::fsm::{FSM, FSMError};
    use crate::types::{Transition, Effector};

    use super::{test_invalid_string, test_valid_string};

    #[derive(Eq, PartialEq, Clone, Copy, Hash, Debug)]
    enum State {
        INIT,
        WORD,
        NUMBER_IP,
        NUMBER_FP
    }

    fn is_letter(ch: char) -> bool {
        ('A'..='Z').contains(&ch) ||
            ('a'..='z').contains(&ch)
    }

    fn is_digit(ch: char) -> bool {
        ('0'..='9').contains(&ch)
    }

    fn fallback(_: char) -> bool {
        true
    }
    struct Counter {
        pub word_count: usize,
        pub number_count: usize
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
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    struct CounterData {
        pub word_count: usize,
        pub number_count: usize
    }

    impl Effector<Effect, CounterData> for Counter {
        fn dispatch(&mut self, effect: Effect) {
            match effect {
                Effect::INCREMENT_WORD_COUNT => self.increment_word_count(),
                Effect::INCREMENT_NUMBER_COUNT => self.increment_number_count()
            }
        }

        fn state(&self) -> CounterData {
            CounterData {
                word_count: self.word_count,
                number_count: self.number_count
            }
        }

        fn clear_state(&mut self) {
            self.word_count = 0;
            self.number_count = 0;
        }
    }

    fn setup_fsm() -> FSM<State, Effect, CounterData> {
        let counter = Box::new(
            Counter {
                word_count: 0,
                number_count: 0
            }
        );
        
        let fsm = FSM::new(
            State::INIT,
            map!(
                State::INIT => vec![
                    Transition::new(
                        State::WORD,
                        Some(is_letter),
                        Some(Effect::INCREMENT_WORD_COUNT)
                    ),
                    Transition::new(
                        State::NUMBER_IP,
                        Some(is_digit),
                        Some(Effect::INCREMENT_NUMBER_COUNT)
                    ),
                    Transition::new(
                        State::INIT,
                        Some(fallback),
                        None
                    )
                ],
                State::WORD => vec![
                    Transition::new(
                        State::WORD,
                        Some(is_letter),
                        None
                    ),
                    Transition::new(
                        State::NUMBER_IP,
                        Some(is_digit),
                        Some(Effect::INCREMENT_NUMBER_COUNT)
                    ),
                    Transition::new(
                        State::INIT,
                        Some(fallback),
                        None
                    )
                ],
                State::NUMBER_IP => vec![
                    Transition::new(
                        State::WORD,
                        Some(is_letter),
                        Some(Effect::INCREMENT_WORD_COUNT)
                    ),
                    Transition::new(
                        State::NUMBER_IP,
                        Some(is_digit),
                        None
                    ),
                    Transition::new(
                        State::NUMBER_FP,
                        Some(|ch| {
                            ch == '.'
                        }),
                        None
                    ),
                    Transition::new(
                        State::INIT,
                        Some(fallback),
                        None
                    )
                ],
                State::NUMBER_FP => vec![
                    Transition::new(
                        State::WORD,
                        Some(is_letter),
                        Some(Effect::INCREMENT_WORD_COUNT)
                    ),
                    Transition::new(
                        State::NUMBER_FP,
                        Some(is_digit),
                        None
                    ),
                    Transition::new(
                        State::INIT,
                        Some(fallback),
                        None
                    )
                ]
            ),
            Some(counter)
        ); 

        assert!(fsm.is_ok());

        fsm.unwrap()
    }

    #[test]
    fn it_counts_numbers_and_words_correctly() {
        let mut fsm = setup_fsm();

        {
            test_valid_string(
                &mut fsm,
                String::from("the123fox jumps,,,,")
            );

            let state = fsm.effector().as_ref().unwrap().state();

            assert_eq!(
                state.word_count,
                3
            );

            assert_eq!(
                state.number_count,
                1
            );

            fsm.effector().as_mut().unwrap().clear_state();
        }

        {
            test_valid_string(
                &mut fsm,
                String::from("!@#..,.?")
            );

            let state = fsm.effector().as_ref().unwrap().state();

            assert_eq!(
                state.word_count,
                0
            );

            assert_eq!(
                state.number_count,
                0
            );

            fsm.effector().as_mut().unwrap().clear_state();
        }

        {
            test_valid_string(
                &mut fsm,
                String::from("Add 1.5 pinches of salt and 2 cups of water!")
            );

            let state = fsm.effector().as_ref().unwrap().state();

            assert_eq!(
                state.word_count,
                8
            );

            assert_eq!(
                state.number_count,
                2
            );

            fsm.effector().as_mut().unwrap().clear_state();
        }
    }