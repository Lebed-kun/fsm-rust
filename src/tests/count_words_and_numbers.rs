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
        fn dispatch(&mut self, effect: Effect) {
            match effect {
                Effect::INCREMENT_WORD_COUNT => self.increment_word_count(),
                Effect::INCREMENT_NUMBER_COUNT => self.increment_number_count()
            }
        }
    }

    fn setup_fsm() -> FSM<State, Effect> {
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
                        None,
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
                        None,
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
                        None,
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
                        None,
                        None
                    )
                ]
            )
        ); 

        assert!(fsm.is_ok());

        fsm.unwrap()
    }

    fn setup_counter() -> Counter {
        Counter {
            word_count: 0,
            number_count: 0
        }
    }

    #[test]
    fn it_counts_numbers_and_words_correctly() {
        let fsm = setup_fsm();
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