use std::borrow::Borrow;
use std::collections::{BTreeMap, BTreeSet, HashMap, VecDeque};
use std::fmt::{Debug, Formatter, write};
use std::hash::Hash;
use std::iter::once;
use std::ptr::NonNull;
use bimap::BiBTreeMap;
use crate::automaton::FiniteAutomaton;
use crate::dfa::DFA;
use crate::edge::Edge;
use crate::input::Input;
use crate::result::{Error, IResult};
use crate::state::State;

pub struct NFA {
    /// 初态集，要求非空
    pub initial_states: BTreeSet<State>,
    /// 终态集，可空
    pub finite_states: BTreeSet<State>,
    /// 合法输入字符集
    pub feasible_inputs: BTreeSet<Input>,
    /// 邻接矩阵，用于存储状态转换图中的所有弧
    pub adjacency_matrix: BTreeMap<State, BTreeMap<State, Edge>>,
    /// ɛ 闭包矩阵，key state s 对应的 value 为从 s 出发经任意条 ɛ 弧而能到达的任何状态集
    pub epsilon_closure_matrix: Option<BTreeMap<State, BTreeSet<State>>>,
}

impl<'a> FiniteAutomaton<'a> for NFA {
    fn new() -> Self {
        NFA {
            initial_states: BTreeSet::new(),
            finite_states: BTreeSet::new(),
            feasible_inputs: BTreeSet::new(),
            adjacency_matrix: BTreeMap::new(),
            epsilon_closure_matrix: None,
        }
    }

    fn add_initial_states<I>(&mut self, initial_states: I) -> IResult<()>
        where I: Iterator<Item = &'a str> {
        initial_states.for_each(|s| {
            self.initial_states.insert(State::new(s));
        });
        Ok(())
    }

    fn add_finite_states<I>(&mut self, finite_states: I) -> IResult<()>
        where I: Iterator<Item = &'a str> {
        finite_states.for_each(|s| {
            self.finite_states.insert(State::new(s));
        });
        Ok(())
    }

    fn add_transfer_rule(&mut self, from_state_id: &str, input_str: &str, to_state_id: &str)  -> IResult<()> {
        let from_state = State::new(from_state_id);
        let to_state = State::new(to_state_id);
        let input = Input::new(input_str);

        if input_str != "ɛ" {
            self.feasible_inputs.insert(input.clone());
        }

        self.adjacency_matrix
            .entry(from_state)
            .or_insert(BTreeMap::new())
            .entry(to_state.clone())
            .and_modify(|e| e.add_input(input_str))
            .or_insert(Edge::with_inputs([input]));

        self.adjacency_matrix.try_insert(to_state, BTreeMap::new());

        self.epsilon_closure_matrix = None;

        Ok(())
    }

    fn get_all_states_iter(&'a self) -> Box<dyn Iterator<Item = &'a State> + 'a> {
        Box::new(self.adjacency_matrix.keys())
    }

    fn get_all_states_into_iter(&'a self) -> Box<dyn Iterator<Item = State> + 'a> {
        Box::new(self.get_all_states_iter().map(|s| s.to_owned()))
    }

    fn get_all_finite_states_iter(&'a self) -> Box<dyn Iterator<Item = &'a State> + 'a> {
        Box::new(self.finite_states.iter())
    }

    fn get_all_finite_states_into_iter(&'a self) -> Box<dyn Iterator<Item = State> + 'a> {
        Box::new(self.get_all_finite_states_iter().map(|s| s.to_owned()))
    }

    fn get_all_infinite_states_iter(&'a self) -> Box<dyn Iterator<Item = &'a State> + 'a> {
        Box::new(self.get_all_states_iter()
            .filter(|s| !self.finite_states.contains(s)))
    }

    fn get_all_infinite_states_into_iter(&'a self) -> Box<dyn Iterator<Item = State> + 'a> {
        Box::new(self.get_all_infinite_states_iter().map(|s| s.to_owned()))
    }

    fn get_states_num(&self) -> usize {
        self.adjacency_matrix.len()
    }
}

impl NFA {
    /// 计算 ɛ 闭包矩阵，使用 Warshall 算法
    pub fn calc_epsilon_closure_matrix(&mut self) {
        let mut epsilon_closure_matrix = BTreeMap::new();
        for s in self.get_all_states_iter() {
            let mut set = BTreeSet::from_iter(self.adjacency_matrix.get(s).unwrap().iter()
                .filter(|(_,v)| v.contains_input("ɛ"))
                .map(|(key,_)| key.to_owned()));
            set.insert(s.to_owned());
            epsilon_closure_matrix.insert(s.to_owned(), set);
        }

        for sk in self.get_all_states_iter() {
            for si in self.get_all_states_iter() {
                for sj in self.get_all_states_iter() {
                    if epsilon_closure_matrix.get(&si).unwrap().contains(&sk) &&
                        epsilon_closure_matrix.get(&sk).unwrap().contains(&sj) {
                        epsilon_closure_matrix.get_mut(&si).unwrap().insert(sj.to_owned());
                    }
                }
            }
        }

        self.epsilon_closure_matrix.replace(epsilon_closure_matrix);
    }

    /// 获得一个 query_states 集的 ɛ 闭包
    /// 注意：调用此方法前，需要先调用 [`NFA::calc_epsilon_closure_matrix`] 计算 ɛ 闭包矩阵，否则将抛出 [`Error::Uninitialized`]
    pub fn get_epsilon_closure<'a, I>(&self, query_states: I) -> IResult<BTreeSet<State>>
        where I: Iterator<Item = &'a State> {
        if let Some(epsilon_closure_matrix) = &self.epsilon_closure_matrix {
            Ok(query_states
                    .map(|s| epsilon_closure_matrix.get(s).unwrap())
                    .flat_map(|s| s.to_owned())
                    .collect()
            )
        } else {
            Err(Error::Uninitialized("You need to invoke NFA::calc_epsilon_closure_matrix first."))
        }
    }

    /// 获得从 query_states 集中任一结点出发，经过一条 by_input_str 弧到达的任何状态集
    fn straight_reachable_states<'a, I>(&self, query_states: I, by_input_str: &str) -> BTreeSet<State>
        where I: Iterator<Item = &'a State> {
        BTreeSet::from_iter(query_states
            .filter_map(|s| self.adjacency_matrix.get(s))
            .flat_map(|map| {
                map.iter()
                    .filter(|(_, v)| v.contains_input(by_input_str))
                    .map(|(k, _)| k.to_owned())
            }))
    }

    /// 将一个 NFA 转换为 DFA
    fn to_dfa(& self) -> NFA {
        let mut dfa = NFA::new();
        let start_state = self.get_epsilon_closure(self.initial_states.iter()).unwrap();
        let mut search_queue = VecDeque::new(); // 搜索队列
        let mut known_states = BTreeMap::new(); // 保存所有已知的状态
        // 初始状态入队
        search_queue.push_back(start_state.clone());
        known_states.insert(start_state, "0".to_string());
        dfa.add_initial_states(once("0"));
        // 循环直至搜索队列为空
        while let Some(front_state) = search_queue.pop_front() { // 取出队首 front_state
            let new_front_state_id = known_states.get(&front_state).unwrap().to_owned();
            // 计算从 front_state 接受 input 所转换到的状态
            for input in &self.feasible_inputs {
                let j = self.straight_reachable_states(front_state.iter(), input.get_str());
                let transfered_state = self.get_epsilon_closure(j.iter()).unwrap();
                // 如果这一状态没有被计算过，则将其加入搜索队列
                if !known_states.contains_key(&transfered_state) {
                    search_queue.push_back(transfered_state.clone());
                }
                // 更新已知状态
                let num_known_states = known_states.len();
                let transfered_state_id = known_states.entry(transfered_state.clone())
                    .or_insert( num_known_states.to_string());
                // 添加一条转换规则
                dfa.add_transfer_rule(&new_front_state_id, input.get_str(), &transfered_state_id);
                // 如果当前状态含有原终态，则是新的终态
                if self.finite_states.iter().any(|s| {
                    transfered_state.contains(s)
                }) {
                    dfa.add_finite_states(once(transfered_state_id.as_str()));
                }
            }
        }
        println!("{:?}", known_states);
        return dfa;
    }
}

impl Debug for NFA {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut ret = String::new();
        ret.push_str("FiniteAutomaton {\n    initial_states: ");
        ret.push_str(&format!("{:?}", self.initial_states));
        ret.push_str("\n    finite_states: ");
        ret.push_str(&format!("{:?}", self.finite_states));
        ret.push_str("\n    transfer_rules: ");
        for (from_state, to_map) in &self.adjacency_matrix {
            for (to_state, edge) in to_map {
                ret.push_str(&format!("\n        {:?} => {:?} => {:?}", from_state, edge, to_state));
            }
        }
        ret.push_str("\n}");
        write!(f, "{}", ret)
    }
}

macro_rules! nfa {
    (initial_states: $($initial_state: expr),* ;
     finite_states: $($finite_state: expr),* ;
     transfer_rules: $($from_state: expr => $input: expr => $to_state: expr),*) => {{
        let mut nfa = NFA::new();
        $(nfa.add_initial_states(once($initial_state));)*
        $(nfa.add_finite_states(once($finite_state));)*
        $(nfa.add_transfer_rule($from_state, $input, $to_state);)*
        nfa
    }};
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn macro_test() {
        let mut nfa = nfa!(
            initial_states: "X";
            finite_states: "Y";
            transfer_rules: "X" => "ɛ" => "5",
                            "5" => "a" => "5",
                            "5" => "b" => "5",
                            "5" => "ɛ" => "1",
                            "1" => "a" => "3",
                            "3" => "a" => "2",
                            "1" => "b" => "4",
                            "4" => "b" => "2",
                            "2" => "ɛ" => "6",
                            "6" => "a" => "6",
                            "6" => "b" => "6",
                            "6" => "ɛ" => "Y");
        println!("{:#?}", nfa);
        nfa.calc_epsilon_closure_matrix();
        let dfa = nfa.to_dfa();
        println!("{:?}", dfa);
    }
}