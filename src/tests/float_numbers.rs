#![cfg(test)]

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