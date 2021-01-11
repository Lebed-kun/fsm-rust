use std::hash::Hash;
use std::fmt::Debug;
use crate::fsm::{FSM, FSMError};
use crate::types::{Effector, StreamData};

pub fn test_valid_string<'a, State, Effect>(
    fsm: &'a FSM<State, Effect>, 
    string: &'a String,
    effector: Option<&'a mut dyn Effector<Effect>>
) 
    where State: Eq + PartialEq + Copy + Hash + Debug,
          Effect: Eq + PartialEq + Copy
{
    let result = fsm.proceed(string, effector);

    if result.is_err() {
        println!(
            "\n========\n\n{:?}\n\n=========\n", 
            result.unwrap_err()
        );
    }

    assert!(result.is_ok());
}

pub fn test_invalid_string<'a, State, Effect>(
    fsm: &'a FSM<State, Effect>, 
    string: &'a String,
    index: usize,
    character: char,
    effector: Option<&'a mut dyn Effector<Effect>>
) 
    where State: Eq + PartialEq + Copy + Hash + Debug,
          Effect: Eq + PartialEq + Copy
{
    let result = fsm.proceed(string, effector);
    assert!(result.is_err());

    let error_from_res = result.unwrap_err();

    assert!(
        matches!(
            error_from_res,
            FSMError::NoValidTransition { 
                input_data: StreamData { index, character, .. }, 
                .. 
            }
        )
    );

    println!(
        "\n========\n\n{:?}\n\n=========\n", 
        error_from_res
    );
}

pub fn is_letter(ch: char) -> bool {
    ('A'..='Z').contains(&ch) ||
        ('a'..='z').contains(&ch)
}

pub fn is_digit(ch: char) -> bool {
    ('0'..='9').contains(&ch)
}
