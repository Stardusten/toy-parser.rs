use std::collections::BTreeSet;
use std::fmt::{Debug, Formatter};
use crate::input::Input;

pub struct Edge {
    pub input_set: BTreeSet<Input>
}

impl Edge {
    /// 创建一个新的，包含指定输入字符的 Edge
    pub fn with_inputs<I>(inputs: I) -> Self
        where I: IntoIterator<Item = Input> {
        Edge {
            input_set: BTreeSet::from_iter(inputs)
        }
    }

    /// 判断是否包含指定输入字符
    pub fn contains_input(&self, input_str: &str) -> bool {
        self.input_set.contains(&Input::new(input_str))
    }

    /// 增加一个输入字符
    pub fn add_input(&mut self, input_str: &str) {
        self.input_set.insert(Input::new(input_str));
    }
}

impl Debug for Edge {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_set().entries(self.input_set.iter()).finish()
    }
}