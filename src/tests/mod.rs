use std::hash::Hash;
use std::fmt::Debug;
use crate::macros;
use crate::fsm::{FSM, FSMError};
use crate::types::{Transition, Effector, StreamData};

mod float_numbers;
mod count_words_and_numbers;

fn test_valid_string<'a, State, Effect>(
    fsm: &'a FSM<State, Effect>, 
    string: &'a String,
    effector: Option<&'a mut Effector<Effect>>
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

fn test_invalid_string<'a, State, Effect>(
    fsm: &'a FSM<State, Effect>, 
    string: &'a String,
    index: usize,
    character: char,
    effector: Option<&'a mut Effector<Effect>>
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