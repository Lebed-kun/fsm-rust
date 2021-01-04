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
            None => (Some(self.to), self.effect)
        }
    }
}

pub trait Effector<Effect> 
    where Effect: Eq + PartialEq + Copy
{
    fn dispatch(&mut self, effect: Effect);
}
