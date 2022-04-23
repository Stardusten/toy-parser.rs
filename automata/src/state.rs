use std::fmt::{Debug, Formatter};

#[derive(PartialOrd, Ord, PartialEq, Eq, Clone, Hash)]
pub struct State {
    pub state_id: String,
}

impl State {
    pub fn new(state_id: impl Into<String>) -> Self {
        State {
            state_id: state_id.into(),
        }
    }
}

impl Debug for State {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.state_id)
    }
}