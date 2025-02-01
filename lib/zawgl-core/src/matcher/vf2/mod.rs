// MIT License
// Copyright (c) 2022 Alexandre RICCIARDI
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

mod base_state;
mod state;

use std::collections::HashMap;
use log::trace;

use crate::graph_engine::model::{ProxyNodeId, GraphProxy};
use crate::model::{PropertyGraph, Relationship, Node};

use self::state::State;
use super::super::graph::traits::*;
use super::super::graph::*;

enum IterationStates {
    Process,
    Validate,
    LookForCandidates,
    InitGraph1Loop,
    Graph1Loop,
    Backtrack,
}


struct VecIterator<T> {
    index: usize,
    end: bool,
    vector: Vec<T>,
}

impl <T> VecIterator<T> {
    fn new(v: Vec<T>) -> Self {
        let end = v.is_empty();
        VecIterator { index: 0, end, vector: v }
    }

    fn end(&self) -> bool {
        self.end
    }

    fn index(&self) -> usize {
        self.index
    }

    fn value(&self) -> &T {
        &self.vector[self.index]
    }

    fn set_index(&mut self, index: usize) {
        if index < self.vector.len() {
            self.index = index;
            self.end = false;
        } else {
            self.end = true;
        }
    }

    fn reset(&mut self) {
        self.end = self.vector.is_empty();
        self.index = 0;
    }

    fn inc(&mut self) {
        if self.index < self.vector.len() - 1 {
            self.index += 1;
        } else {
            self.end = true;
        }
    }
}

pub struct Matcher<'g0, VCOMP, ECOMP, CALLBACK>
    where VCOMP: Fn(&Node, &Node) -> bool, ECOMP: Fn(&Relationship, &Relationship) -> bool,
    CALLBACK: FnMut(&HashMap<NodeIndex, ProxyNodeId>, &HashMap<ProxyNodeId, NodeIndex>, &PropertyGraph, &mut GraphProxy<'_>) -> Option<bool> {
        state: State<'g0, VCOMP, ECOMP>,
        callback: CALLBACK,
}

impl <'g0, VCOMP, ECOMP, CALLBACK> Matcher <'g0, VCOMP, ECOMP, CALLBACK>
    where VCOMP: Fn(&Node, &Node) -> bool, ECOMP: Fn(&Relationship, &Relationship) -> bool,
    CALLBACK: FnMut(&HashMap<NodeIndex, ProxyNodeId>, &HashMap<ProxyNodeId, NodeIndex>, &PropertyGraph, &mut GraphProxy<'_>) -> Option<bool> {

        pub fn new(graph_0: &'g0 PropertyGraph, vcomp: VCOMP, ecomp: ECOMP, callback: CALLBACK) -> Self {
            Matcher {
                state: State::new(graph_0, vcomp, ecomp),
                callback,
            }
        }

        pub fn process(&mut self, ids0: Vec<NodeIndex>, ids1: Vec<ProxyNodeId>, graph_1: &mut GraphProxy<'_>) -> Option<bool> {
            let mut index0 = VecIterator::new(ids0);
            let mut index1 = VecIterator::new(ids1);
            let mut state = IterationStates::Process;
            let mut found_match = false;
            let mut match_continuation = Vec::new();
            loop {
                match state {
                    IterationStates::Process => {
                        if self.state.success() {
                            found_match = true;
                            if !self.state.call_back(&mut self.callback, graph_1)? {
                                return Some(true);
                            } else {
                                state = IterationStates::Backtrack;
                            }
                        } else {
                            state = IterationStates::Validate;
                        }
                    },
                    IterationStates::Validate => {
                        if !self.state.valid() {
                            state = IterationStates::Backtrack;
                        } else {
                            state = IterationStates::LookForCandidates;
                        }
                    },
                    IterationStates::LookForCandidates => {
                        index0.reset();
                        while !index0.end() && !self.state.possible_candidate_0(index0.value()) {
                            trace!("possible candidate for pattern {:?}", index0.value());
                            index0.inc();
                        }
                        state = IterationStates::InitGraph1Loop;
                    },
                    IterationStates::InitGraph1Loop => {
                        index1.reset();
                        state = IterationStates::Graph1Loop;
                    },
                    IterationStates::Graph1Loop => {
                        let mut backtrack = true;
                        while !index1.end() {
                            if self.state.possible_candidate_1(index1.value()) && self.state.feasible(index0.value(), index1.value(), graph_1)? {
                                trace!("feasible dn node {:?}", index1.value());
                                match_continuation.push((index0.index(), index1.index()));
                                self.state.push(index0.value(), index1.value(), graph_1);
                                backtrack = false;
                                break;
                            }
                            index1.inc();
                        }
                        if !backtrack {
                            state = IterationStates::Process;
                        } else {
                            state = IterationStates::Backtrack;
                        }
                    },
                    IterationStates::Backtrack => {
                        if let Some(back) = match_continuation.pop() {
                            index0.set_index(back.0);
                            index1.set_index(back.1);
                            self.state.pop(index0.value(), index1.value(), graph_1)?;
                            index1.inc();
                            state = IterationStates::Graph1Loop;
                        } else {
                            return Some(found_match);
                        }
                    }
                }
            }
        }
    }

fn sort_nodes(graph: &'_ PropertyGraph) -> Vec<NodeIndex> {
    let mut res = graph.get_nodes_ids();
    res.sort_by_key(|b| std::cmp::Reverse(graph.in_degree(b) + graph.out_degree(b)));
    res
}

pub fn sub_graph_isomorphism<'g0: 'g1, 'g1: 'a, 'a, VCOMP, ECOMP, CALLBACK>
(graph_0: &'g0 PropertyGraph, graph_1: &'g1 mut GraphProxy<'a>, vcomp: VCOMP, ecomp: ECOMP, callback: CALLBACK) -> Option<bool>
where VCOMP: Fn(&Node, &Node) -> bool, ECOMP: Fn(&Relationship, &Relationship) -> bool,
CALLBACK: FnMut(&HashMap<NodeIndex, ProxyNodeId>, &HashMap<ProxyNodeId, NodeIndex>, &PropertyGraph, &mut GraphProxy<'_>)-> Option<bool>  {

    let id0 = sort_nodes(graph_0);
    let id1 = graph_1.get_nodes_ids();
    let mut matcher = Matcher::new(graph_0, vcomp, ecomp, callback);
    matcher.process(id0, id1, graph_1)
}