use std::hash::Hash;
use std::fmt::Debug;
use crate::macros;
use crate::fsm::{FSM, FSMError};
use crate::types::Transition;

fn test_valid_string<State, Effect>(fsm: &FSM<State, Effect>, string: String) 
    where State: Eq + PartialEq + Copy + Hash + Debug,
          Effect: Eq + PartialEq + Copy
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

fn test_invalid_string<State, Effect>(
    fsm: &FSM<State, Effect>, 
    string: String,
    index: usize,
    character: char
) 
    where State: Eq + PartialEq + Copy + Hash + Debug,
          Effect: Eq + PartialEq + Copy
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
    use crate::types::{Transition, Effector};

    use super::{test_invalid_string, test_valid_string};

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
            ),
            None
        );

        assert!(fsm.is_ok());

        fsm.unwrap()
    }

    #[test]
    fn it_validates_float_numbers() {
        let fsm = setup_fsm();

        test_valid_string(
            &fsm,
            String::from("0")
        );

        test_valid_string(
            &fsm,
            String::from("12345")
        );

        test_valid_string(
            &fsm,
            String::from("+12345")
        );

        test_valid_string(
            &fsm,
            String::from("-12345")
        );

        test_valid_string(
            &fsm,
            String::from("12345.9876")
        );

        test_valid_string(
            &fsm,
            String::from("-12345.9876")
        );

        test_valid_string(
            &fsm,
            String::from("+12345.9876")
        );

        test_valid_string(
            &fsm,
            String::from("0.12345")
        );

        test_valid_string(
            &fsm,
            String::from("-0.12345")
        );

        test_valid_string(
            &fsm,
            String::from("+0.12345")
        );
    }

    #[test]
    fn it_invalidates_incorrect_string() {
        let fsm = setup_fsm();

        // From INIT state
        test_invalid_string(
            &fsm,
            String::from("w1234"),
            0,
            'w'
        );

        // From SIGN state
        test_invalid_string(
            &fsm,
            String::from("++1234"),
            1,
            '+'
        );

        // From INTEGER_PART state
        test_invalid_string(
            &fsm,
            String::from("1110b"),
            4,
            'b'
        );

        // From ZERO state
        test_invalid_string(
            &fsm,
            String::from("001234"),
            1,
            '0'
        );

        // From FRACTION_PART state
        test_invalid_string(
            &fsm,
            String::from("12..0126"),
            3,
            '.'
        ); 
    }
}

/*
#[cfg(test)]
mod count_words_and_numbers {
    use std::hash::Hash;
    use std::fmt::Debug;
    use std::rc::Rc;
    use std::cell::RefCell;
    use crate::macros;
    use crate::fsm::{FSM, FSMError};
    use crate::types::Transition;

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

    type Count = Rc<RefCell<usize>>;

    fn setup_fsm_and_counters() -> (FSM<State, u8>, Count, Count) {
        let word_counter = Rc::new(RefCell::new(0));
        let number_counter = Rc::new(RefCell::new(0));

        let word_counter_in_closure = word_counter.clone();
        let number_counter_in_closure = number_counter.clone();

        let mut increment_words = 
            Rc::new(RefCell::new(
                move || {
                    word_counter_in_closure.replace(
                        word_counter_in_closure.clone().into_inner() + 1
                    );
                }
            ))
        ;

        let mut increment_numbers = 
            Rc::new(RefCell::new(
                move || {
                    number_counter_in_closure.replace(
                        number_counter_in_closure.clone().into_inner() + 1
                    );
                }
            ))
        ;
        
        let fsm = FSM::new(
            State::INIT,
            map!(
                State::INIT => vec![
                    Transition::new(
                        State::WORD,
                        Some(is_letter),
                        Some(Box::new(
                            Rc::clone(&increment_words).into_inner()
                        ))
                    ),
                    Transition::new(
                        State::NUMBER_IP,
                        Some(is_digit),
                        Some(Box::new(
                            Rc::clone(&increment_numbers).into_inner()
                        ))
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
                        Some(Box::new(
                            Rc::clone(&increment_numbers).into_inner()
                        ))
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
                        Some(Box::new(
                            Rc::clone(&increment_words).into_inner()
                        ))
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
                        Some(Box::new(
                            Rc::clone(&increment_words).into_inner()
                        ))
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
            )
        ); 

        assert!(fsm.is_ok());

        (fsm.unwrap(), word_counter.clone(), number_counter.clone())
    }
}
*/