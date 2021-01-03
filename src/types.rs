pub type Predicate = fn(ch: char) -> bool;

/// Optional condition is for introducing epsilon (unconditional + without character read) transitions
/// which also require accepting states property and
/// are not the same as unconditional (fallback) transitions
pub struct Transition<State, Effect> 
    where State: Eq + PartialEq + Copy,
          Effect: Eq + PartialEq + Copy
{
    condition: Option<Predicate>,
    to: State,
    effect: Option<Effect>
}

impl<State, Effect> Transition<State, Effect> 
    where State: Eq + PartialEq + Copy,
          Effect: Eq + PartialEq + Copy
{
    pub fn new(to: State, condition: Option<Predicate>, effect: Option<Effect>) -> Self {
        Self {
            to,
            condition,
            effect
        }
    }

    pub fn transit(&self, ch: char) -> (Option<State>, Option<Effect>) {
        match self.condition {
            Some(condition) => {
                if condition(ch) {
                    (Some(self.to), self.effect)
                } else {
                    (None, None)
                }
            },
            /// TODO: edit this branch to match definition of epsilon transition
            None => (None, None)
        }
    }
}

pub trait Effector<Effect, State> 
    where Effect: Eq + PartialEq + Copy,
          State: Copy
{
    fn dispatch(&mut self, effect: Effect);
    fn state(&self) -> State;
}
