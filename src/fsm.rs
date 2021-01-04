use std::collections::HashMap;
use std::hash::Hash;
use std::fmt::Debug;
use crate::types::{Transition, Effector};

pub struct FSM<State, Effect>
    where State: Eq + PartialEq + Copy + Hash,
          Effect: Eq + PartialEq + Copy,
{
    initial_state: State,
    transition_table: HashMap<State, Vec<Transition<State, Effect>>>,
}

#[derive(Copy, Clone, Debug)]
pub enum FSMError<'a, State> 
    where State: Eq + PartialEq + Copy + Debug
{
    StateDoesNotExist(State),
    NoValidTransition {
        from: State,
        string: &'a String,
        index: usize,
        character: char
    }
}

impl<State, Effect> FSM<State, Effect> 
    where State: Eq + PartialEq + Copy + Hash + Debug,
          Effect: Eq + PartialEq + Copy,
{
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
                                    effector.dispatch(effect);    
                                }

                                break;
                            },
                            _ => {}
                        }
                    }

                    if !accepted {
                        return Err(FSMError::NoValidTransition {
                            from: curr_state,
                            string,
                            index: char_id,
                            character: ch
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
