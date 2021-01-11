use std::collections::HashMap;
use std::hash::Hash;
use std::fmt::Debug;
use crate::types::{Transition, Effector, StreamData};

/// Finite state machine with side effects (Mealy automata)
pub struct FSM<State, Effect>
    where State: Eq + PartialEq + Copy + Hash,
          Effect: Eq + PartialEq + Copy,
{
    /// State at beginning of running through stream
    initial_state: State,
    /// Transition graph that connects every state of FSM
    /// to some next states by transitions
    transition_table: HashMap<State, Vec<Transition<State, Effect>>>,
}

/// Error that occurs during initialization or running with FSM
#[derive(Copy, Clone, Debug)]
pub enum FSMError<'a, State> 
    where State: Eq + PartialEq + Copy + Debug
{
    StateDoesNotExist(State),
    NoValidTransition {
        from: State,
        input_data: StreamData<'a>
    }
}

impl<State, Effect> FSM<State, Effect> 
    where State: Eq + PartialEq + Copy + Hash + Debug,
          Effect: Eq + PartialEq + Copy,
{
    /// Creates new instance of FSM
    /// - initial_state: starting state,
    /// - transition_table: transition graph
    pub fn new<'a>(
        initial_state: State, 
        transition_table: HashMap<State, Vec<Transition<State, Effect>>>
    ) -> Result<Self, FSMError<'a, State>> {
        if !transition_table.contains_key(&initial_state) {
            Err(FSMError::StateDoesNotExist(initial_state))
        } else {
            Ok(Self {
                initial_state,
                transition_table
            })
        }
    }

    pub fn merge_effects<'a>(
        &mut self, 
        effects_map: &HashMap<State, Vec<Effect>>
    ) -> Result<(), FSMError<'a, State>> {
        for (state, effects) in effects_map.iter() {
            match self.transition_table.get_mut(state) {
                Some(transitions) => {
                    for i in 0..effects.len() {
                        transitions[i].effect = Some(effects[i]);
                    }
                },
                None => return Err(
                    FSMError::StateDoesNotExist(*state)
                )
            }
        }

        Ok(())
    }

    /// Runs some string through FSM to validate it (and apply some effects)
    /// - string: runnable string,
    /// - effector: module that mutates some data by effects
    pub fn proceed<'a>(
        &self, 
        string: &'a String,
        mut effector: Option<&'a mut dyn Effector<Effect>>
    ) -> Result<(), FSMError<'a, State>> 
    {
        let mut curr_state = self.initial_state;
        let mut char_id: usize = 0;

        for ch in string.chars() {
            match self.transition_table.get(&curr_state) {
                Some(transitions) => {
                    let mut accepted = false;
                    
                    for transition in transitions.iter() {
                        match transition.transit(ch) {
                            (Some(new_state), effect) => {
                                curr_state = new_state;
                                accepted = true;
                                
                                if let (Some(effector), Some(effect)) = 
                                    (effector.as_mut(), effect) 
                                {
                                    effector.dispatch(effect, StreamData {
                                        string,
                                        index: char_id,
                                        character: ch
                                    });    
                                }

                                break;
                            },
                            _ => {}
                        }
                    }

                    if !accepted {
                        return Err(FSMError::NoValidTransition {
                            from: curr_state,
                            input_data: StreamData {
                                string,
                                index: char_id,
                                character: ch
                            }
                        });
                    }
                },
                None => {
                    return Err(FSMError::StateDoesNotExist(curr_state));
                }
            }

            char_id += 1;
        }

        Ok(())
    }
}
