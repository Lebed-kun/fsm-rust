use std::hash::Hash;
use std::fmt::Debug;
use crate::macros;
use crate::fsm::{FSM, FSMError};
use crate::types::Transition;

mod float_numbers;
mod count_words_and_numbers;

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