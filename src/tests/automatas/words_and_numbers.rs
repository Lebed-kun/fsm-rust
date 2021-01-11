use std::hash::Hash;
use std::collections::HashMap;
use std::fmt::Debug;
use crate::macros;
use crate::fsm::{FSM, FSMError};
use crate::types::{Transition, Effector, StreamData};
use super::super::utils::{is_digit, is_letter};

#[derive(Eq, PartialEq, Clone, Copy, Hash, Debug)]
pub enum State {
    INIT,
    WORD,
    NUMBER_IP,
    NUMBER_FP
}

pub fn init_fsm<Effect>() -> FSM<State, Effect> 
    where Effect: Eq + PartialEq + Copy
{
    let fsm = FSM::new(
        State::INIT,
        map!(
            State::INIT => vec![
                Transition::new(
                    State::WORD,
                    Some(is_letter),
                    None
                ),
                Transition::new(
                    State::NUMBER_IP,
                    Some(is_digit),
                    None
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
                    None
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
                    None
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
                    None
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
