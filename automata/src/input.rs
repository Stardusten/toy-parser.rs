use std::fmt::{Debug, Formatter};

#[derive(PartialOrd, Ord, PartialEq, Eq, Clone, Hash)]
pub struct Input {
    input_str: String,
}

impl Input {
    pub fn new(input_str: impl Into<String>) -> Self {
        Input {
            input_str: input_str.into(),
        }
    }

    pub fn get_str(&self) -> &str {
        self.input_str.as_str()
    }
}

impl Debug for Input {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.input_str)
    }
}