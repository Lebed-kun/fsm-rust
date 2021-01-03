use std::hash::Hash;
use std::fmt::Debug;
use crate::macros;
use crate::fsm::{FSM, FSMError};
use crate::types::Transition;

fn test_valid_string<State, Effect, EffectorState>(
    fsm: &mut FSM<State, Effect, EffectorState>, string: String
) 
    where State: Eq + PartialEq + Copy + Hash + Debug,
          Effect: Eq + PartialEq + Copy,
          EffectorState: Copy
{
    let result = fsm.proceed(&string);

    if result.is_err() {
        println!(
            "\n========\n\n{:?}\n\n=========\n", 
            result.unwrap_err()
        );
    }

    assert!(result.is_ok());
}

fn test_invalid_string<State, Effect, EffectorState>(
    fsm: &mut FSM<State, Effect, EffectorState>, 
    string: String,
    index: usize,
    character: char
) 
    where State: Eq + PartialEq + Copy + Hash + Debug,
          Effect: Eq + PartialEq + Copy,
          EffectorState: Copy
{
    let result = fsm.proceed(&string);
    assert!(result.is_err());

    let error_from_res = result.unwrap_err();

    assert!(
        matches!(
            error_from_res,
            FSMError::NoValidTransition { index, character, .. }
        )
    );

    println!(
        "\n========\n\n{:?}\n\n=========\n", 
        error_from_res
    );
}

#[cfg(test)]
mod float_numbers {
    use std::hash::Hash;
    use std::fmt::Debug;
    use crate::macros;
    use crate::fsm::{FSM, FSMError};
    use crate::types::Transition;

    use super::{test_invalid_string, test_valid_string};

    #[derive(Eq, PartialEq, Clone, Copy, Hash, Debug)]
    enum State {
        INIT,
        SIGN,
        INTEGER_PART,
        FRACTION_PART,
        ZERO,
    }

    fn setup_fsm() -> FSM<State, u8, u8> {
        let fsm = FSM::new(
            State::INIT,
            map!(
                State::INIT => vec![
                    Transition::new(
                        State::SIGN,
                        Some(
                            |ch| {
                                ch == '+' || ch == '-'
                            }
                        ),
                        None
                    ),
                    Transition::new(
                        State::INTEGER_PART,
                        Some(
                            |ch| {
                                ('1'..='9').contains(&ch)
                            }
                        ),
                        None
                    ),
                    Transition::new(
                        State::ZERO,
                        Some(
                            |ch| {
                                ch == '0'
                            }
                        ),
                        None
                    )
                ],
                State::SIGN => vec![
                    Transition::new(
                        State::INTEGER_PART,
                        Some(
                            |ch| {
                                ('1'..='9').contains(&ch)
                            }
                        ),
                        None
                    ),
                    Transition::new(
                        State::ZERO,
                        Some(
                            |ch| {
                                ch == '0'
                            }
                        ),
                        None
                    )
                ],
                State::INTEGER_PART => vec![
                    Transition::new(
                        State::INTEGER_PART,
                        Some(
                            |ch| {
                                ('0'..='9').contains(&ch)
                            }
                        ),
                        None
                    ),
                    Transition::new(
                        State::FRACTION_PART,
                        Some(
                            |ch| {
                                ch == '.'
                            }
                        ),
                        None
                    )
                ],
                State::FRACTION_PART => vec![
                    Transition::new(
                        State::FRACTION_PART,
                        Some(
                            |ch| {
                                ('0'..='9').contains(&ch)
                            }
                        ),
                        None
                    )
                ],
                State::ZERO => vec![
                    Transition::new(
                        State::FRACTION_PART,
                        Some(
                            |ch| {
                                ch == '.'
                            }
                        ),
                        None
                    )
                ]
            ),
            None
        );

        assert!(fsm.is_ok());

        fsm.unwrap()
    }

    #[test]
    fn it_validates_float_numbers() {
        let mut fsm = setup_fsm();

        test_valid_string(
            &mut fsm,
            String::from("0")
        );

        test_valid_string(
            &mut fsm,
            String::from("12345")
        );

        test_valid_string(
            &mut fsm,
            String::from("+12345")
        );

        test_valid_string(
            &mut fsm,
            String::from("-12345")
        );

        test_valid_string(
            &mut fsm,
            String::from("12345.9876")
        );

        test_valid_string(
            &mut fsm,
            String::from("-12345.9876")
        );

        test_valid_string(
            &mut fsm,
            String::from("+12345.9876")
        );

        test_valid_string(
            &mut fsm,
            String::from("0.12345")
        );

        test_valid_string(
            &mut fsm,
            String::from("-0.12345")
        );

        test_valid_string(
            &mut fsm,
            String::from("+0.12345")
        );
    }

    #[test]
    fn it_invalidates_incorrect_string() {
        let mut fsm = setup_fsm();

        // From INIT state
        test_invalid_string(
            &mut fsm,
            String::from("w1234"),
            0,
            'w'
        );

        // From SIGN state
        test_invalid_string(
            &mut fsm,
            String::from("++1234"),
            1,
            '+'
        );

        // From INTEGER_PART state
        test_invalid_string(
            &mut fsm,
            String::from("1110b"),
            4,
            'b'
        );

        // From ZERO state
        test_invalid_string(
            &mut fsm,
            String::from("001234"),
            1,
            '0'
        );

        // From FRACTION_PART state
        test_invalid_string(
            &mut fsm,
            String::from("12..0126"),
            3,
            '.'
        ); 
    }
}

#[cfg(test)]
mod count_words_and_numbers {
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
}