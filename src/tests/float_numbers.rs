#![cfg(test)]

use std::hash::Hash;
    use std::fmt::Debug;
    use crate::macros;
    use crate::fsm::{FSM, FSMError};
    use crate::types::Transition;

    use super::utils::{test_valid_string, test_invalid_string};

    #[derive(Eq, PartialEq, Clone, Copy, Hash, Debug)]
    enum State {
        INIT,
        SIGN,
        INTEGER_PART,
        FRACTION_PART,
        ZERO,
    }

    fn setup_fsm() -> FSM<State, u8> {
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
            )
        );

        assert!(fsm.is_ok());

        fsm.unwrap()
    }

    #[test]
    fn it_validates_float_numbers() {
        let fsm = setup_fsm();

        {
            let string = String::from("0");
            
            test_valid_string(
                &fsm,
                &string,
                None
            );
        }

        {
            let string = String::from("12345");

            test_valid_string(
                &fsm,
                &string,
                None
            );
        }

        {
            let string = String::from("+12345");

            test_valid_string(
                &fsm,
                &string,
                None
            );
        }

        {
            let string = String::from("-12345");

            test_valid_string(
                &fsm,
                &string,
                None
            );
        }

        {
            let string = String::from("12345.9876");

            test_valid_string(
                &fsm,
                &string,
                None
            );
        }

        {
            let string = String::from("-12345.9876");

            test_valid_string(
                &fsm,
                &string,
                None
            );
        }

        {
            let string = String::from("+12345.9876"); 

            test_valid_string(
                &fsm,
                &string,
                None
            );
        }

        {
            let string = String::from("0.12345");

            test_valid_string(
                &fsm,
                &string,
                None
            );
        }

        {
            let string = String::from("-0.12345");

            test_valid_string(
                &fsm,
                &string,
                None
            );
        }

        {
            let string = String::from("+0.12345");

            test_valid_string(
                &fsm,
                &string,
                None
            );
        }
    }

    #[test]
    fn it_invalidates_incorrect_string() {
        let fsm = setup_fsm();

        // From INIT state
        {
            let string = String::from("w1234");

            test_invalid_string(
                &fsm,
                &string,
                0,
                'w',
                None
            );
        }

        // From SIGN state
        {
            let string = String::from("++1234");

            test_invalid_string(
                &fsm,
                &string,
                1,
                '+',
                None
            );
        }

        // From INTEGER_PART state
        {
            let string = String::from("1110b");

            test_invalid_string(
                &fsm,
                &string,
                4,
                'b',
                None
            );
        }

        // From ZERO state
        {
            let string = String::from("001234");

            test_invalid_string(
                &fsm,
                &string,
                1,
                '0',
                None
            );
        }

        // From FRACTION_PART state
        {
            let string = String::from("12..0126");

            test_invalid_string(
                &fsm,
                &string,
                3,
                '.',
                None
            );
        }
    }