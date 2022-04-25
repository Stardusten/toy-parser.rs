use std::borrow::Borrow;
use std::collections::{BTreeMap, BTreeSet, HashMap, VecDeque};
use std::fmt::{Debug, Formatter, write};
use std::hash::Hash;
use std::iter::once;
use std::ptr::NonNull;
use bimap::BiBTreeMap;
use crate::automaton::FiniteAutomaton;
use crate::edge::Edge;
use crate::input::Input;
use crate::nfa::NFA;
use crate::result::{Error, IResult};
use crate::state::State;

pub struct DFA {
    /// 唯一初态
    pub initial_state: Option<State>,
    /// 终态集，可空
    pub finite_states: BTreeSet<State>,
    /// 合法输入字符集
    pub feasible_inputs: BTreeSet<Input>,
    /// 邻接矩阵，用于存储状态转换图中的所有弧
    pub adjacency_matrix: BTreeMap<State, BTreeMap<State, Edge>>
}

impl<'a> FiniteAutomaton<'a> for DFA {

    fn new() -> Self {
        todo!()
    }

    fn add_initial_states<I>(&mut self, initial_states: I) -> IResult<()>
        where I: Iterator<Item = &'a str> {
        if self.initial_state.is_some() {
            return Err(Error::UnsupportedOperation("Initial state is already specified. A DFA has at most one initial state."));
        }
        let vec = initial_states.collect::<Vec<&str>>();
        if vec.len() == 1 {
            self.initial_state.replace( unsafe { State::new(*vec.get_unchecked(0)) });
            return Ok(());
        } else {
            return Err(Error::IllegalArgument("Unexpected number of initial states. A DFA has at most one state.")); // initial_states 为空
        }
    }

    fn add_finite_states<I>(&mut self, finite_states: I) -> IResult<()>
        where I: Iterator<Item = &'a str> {
        finite_states.for_each(|s| {
            self.finite_states.insert(State::new(s));
        });
        Ok(())
    }

    fn add_transfer_rule(&mut self, from_state_id: &str, input_str: &str, to_state_id: &str) -> IResult<()> {
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