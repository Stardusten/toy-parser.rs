use crate::{state::State, result::{Error, IResult}};

pub trait FiniteAutomaton<'a> {
    /// 创建一个新的，空的有限自动机
    fn new() -> Self;
    /// 添加初态
    fn add_initial_states<I>(&mut self, initial_states: I) -> IResult<()>
        where I: Iterator<Item = &'a str>;
    /// 添加终态
    fn add_finite_states<I>(&mut self, finite_states: I) -> IResult<()>
        where I: Iterator<Item = &'a str>;
    /// 添加一条转换规则
    fn add_transfer_rule(&mut self, from_state_id: &str, input_str: &str, to_state_id: &str) -> IResult<()>;
    /// 返回一个包含当前有限状态机中所有状态的 `Iterator`
    fn get_all_states_iter(&'a self)                    -> Box<dyn Iterator<Item = &'a State> + 'a>;
    /// 返回一个包含当前有限状态机中所有状态的 `IntoIterator`
    fn get_all_states_into_iter(&'a self)               -> Box<dyn Iterator<Item = State> + 'a>;
    /// 返回一个包含当前有限状态机中所有终态的 `Iterator`
    fn get_all_finite_states_iter(&'a self)             -> Box<dyn Iterator<Item = &'a State> + 'a>;
    /// 返回一个包含当前有限状态机中所有终态的 `IntoIterator`
    fn get_all_finite_states_into_iter(&'a self)        -> Box<dyn Iterator<Item = State> + 'a>;
    /// 返回一个包含当前有限状态机中所有非终态的 `Iterator`
    fn get_all_infinite_states_iter(&'a self)           -> Box<dyn Iterator<Item = &'a State> + 'a>;
    /// 返回一个包含当前有限状态机中所有非终态的 `IntoIterator`
    fn get_all_infinite_states_into_iter(&'a self)      -> Box<dyn Iterator<Item = State> + 'a>;
    /// 返回当前有限状态机中所有状态总数
    fn get_states_num(&self)                            -> usize;
}