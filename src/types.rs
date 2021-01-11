/// Input character validator
pub type Predicate = fn(ch: char) -> bool;

/// Transition to next state which is validated by condition
pub struct Transition<State, Effect> 
    where State: Eq + PartialEq + Copy,
          Effect: Eq + PartialEq + Copy
{
    /// Predicate that validates current character of stream.
    /// If None then transition is unconditional (i.e. succeeds for every input character)
    condition: Option<Predicate>,
    /// Next state
    to: State,
    /// Side effect that is generated after successful validation of transition
    /// If None then no effect is generated
    effect: Option<Effect>
}

impl<State, Effect> Transition<State, Effect> 
    where State: Eq + PartialEq + Copy,
          Effect: Eq + PartialEq + Copy
{
    /// Creates new transition
    /// - to: next state,
    /// - condition: predicate for character,
    /// - effect: side effect
    pub fn new(to: State, condition: Option<Predicate>, effect: Option<Effect>) -> Self {
        Self {
            to,
            condition,
            effect
        }
    }

    /// Matches next state and side effect for current character
    /// - ch: current character (of stream) 
    pub fn transit(&self, ch: char) -> (Option<State>, Option<Effect>) {
        match self.condition {
            Some(condition) => {
                if condition(ch) {
                    (Some(self.to), self.effect)
                } else {
                    (None, None)
                }
            },
            None => (Some(self.to), self.effect)
        }
    }
}

/// Information for debugging and effects
#[derive(Copy, Clone, Debug)]
pub struct StreamData<'a> {
    /// Reference to input string
    pub string: &'a String,
    /// Current character position in input string
    pub index: usize,
    /// Current character in input string
    pub character: char
}

/// Generic type for executor of side effects 
/// applied to some persistent data
pub trait Effector<Effect> 
    where Effect: Eq + PartialEq + Copy
{
    /// Applies side effect to mutate some data
    /// - effect: side effect,
    /// - input_data: additional dependencies for effects
    fn dispatch(&mut self, effect: Effect, input_data: StreamData);
}
