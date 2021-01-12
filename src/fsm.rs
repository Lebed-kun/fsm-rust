use std::collections::HashMap;
use std::hash::Hash;
use std::fmt::Debug;
use crate::types::{Transition, Effector, StreamData, StatesConnection};

/// Finite state machine with side effects (Mealy automata)
pub struct FSM<State, Effect>
    where State: Eq + PartialEq + Copy + Hash,
          Effect: Copy,
{
    /// State at beginning of running through stream
    initial_state: State,
    /// Transition graph that connects every state of FSM
    /// to some next states by transitions
    transition_table: HashMap<State, Vec<Transition<State, Effect>>>,
    /// - post_effect: side effect that occurs after proceeding 
    /// last character of string (ref. as "post-effect") 
    post_effect: Option<Effect>
}

/// Error that occurs during initialization or running with FSM
#[derive(Copy, Clone, Debug)]
pub enum FSMError<'a, State> 
    where State: Eq + PartialEq + Copy + Hash + Debug
{
    StateDoesNotExist(State),
    TransDoesNotExist(StatesConnection<State>),
    NoValidTransition {
        from: State,
        input_data: StreamData<'a>
    }
}

impl<State, Effect> FSM<State, Effect> 
    where State: Eq + PartialEq + Copy + Hash + Debug,
          Effect: Copy,
{
    /// Creates new instance of FSM
    /// - initial_state: starting state,
    /// - transition_table: transition graph
    /// - post_effect: post-effect
    pub fn new<'a>(
        initial_state: State, 
        transition_table: HashMap<State, Vec<Transition<State, Effect>>>,
        post_effect: Option<Effect>
    ) -> Result<Self, FSMError<'a, State>> {
        if !transition_table.contains_key(&initial_state) {
            Err(FSMError::StateDoesNotExist(initial_state))
        } else {
            Ok(Self {
                initial_state,
                transition_table,
                post_effect
            })
        }
    }

    /// Merges effects into existing fsm for its states
    /// aligned to order of transitions for each state
    /// - effects_map: map from pair of states ("from", "to") to ordered list of effects
    /// (This method is created mainly for testing, reusing the same states and 
    /// transition rules (i.e. partial fsm) for different effects configurations.
    /// For typical cases, 
    /// it's recommended to build fsm from its initialization (i.e. using "new" method))
    pub fn merge_effects<'a>(
        &mut self, 
        effects_map: &HashMap<StatesConnection<State>, Vec<Effect>>
    ) -> Result<(), FSMError<'a, State>> {
        for (conn, effects) in effects_map.iter() {
            if !self.transition_table.contains_key(&conn.to) {
                return Err(FSMError::StateDoesNotExist(conn.to));
            }

            match self.transition_table.get_mut(&conn.from) {
                Some(transitions) => {
                    let mut eff_counter: usize = 0;

                    for trans in transitions.iter_mut() {
                        if conn.to == trans.to {
                            trans.effect = Some(effects[eff_counter]);
                            eff_counter += 1;
                        }
                    }
                    
                    if eff_counter == 0 {
                        return Err(FSMError::TransDoesNotExist(*conn));
                    }
                },
                None => return Err(
                    FSMError::StateDoesNotExist(conn.from)
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

        if let (Some(effector), Some(effect)) = 
            (effector, self.post_effect) 
        {
            effector.dispatch(effect, StreamData {
                string,
                index: string.len(),
                character: '\0'
            })
        }

        Ok(())
    }
}
