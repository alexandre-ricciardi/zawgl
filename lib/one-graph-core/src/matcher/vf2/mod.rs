mod base_state;
mod state;

use std::collections::{HashMap, HashSet};
use self::state::State;
use super::super::graph::traits::*;

enum IterationStates {
    Process,
    Validate,
    LookForCandidates,
    InitGraph1Loop,
    Graph1Loop,
    Backtrack,
}

pub struct Matcher<'g0: 'g1, 'g1, NID0, NID1, EID0, EID1, N0, R0, N1, R1, VCOMP, ECOMP, Graph0, Graph1, CALLBACK>
    where NID0: std::hash::Hash + Eq + MemGraphId + Copy, NID1: std::hash::Hash + Eq + MemGraphId + Copy,
    EID0: std::hash::Hash + Eq + MemGraphId + Copy, EID1: std::hash::Hash + Eq + MemGraphId + Copy, 
    Graph0: GraphContainerTrait<NID0, EID0, N0, R0> + GraphIteratorTrait<NID0, EID0>,
    Graph1: GrowableGraphContainerTrait<NID1, EID1, N1, R1> + GrowableGraphIteratorTrait<NID1, EID1>,
    VCOMP: Fn(&N0, &N1) -> bool, ECOMP: Fn(&R0, &R1) -> bool,
    CALLBACK: FnMut(&HashMap<NID0, NID1>, &HashMap<NID1, NID0>, &Graph0, &mut Graph1) -> Option<bool>  {
        state: State<'g0, 'g1, NID0, NID1, EID0, EID1, N0, R0, N1, R1, VCOMP, ECOMP, Graph0, Graph1>,
        found_match: bool,
        match_continuation: Vec<(NID0, NID1)>,
        first_candidate_0: Option<NID0>,
        callback: CALLBACK,
}

impl <'g0, 'g1, NID0, NID1, EID0, EID1, N0, R0, N1, R1, VCOMP, ECOMP, Graph0, Graph1, CALLBACK> Matcher <'g0, 'g1, NID0, NID1, EID0, EID1, N0, R0, N1, R1, VCOMP, ECOMP, Graph0, Graph1, CALLBACK>
    where NID0: std::hash::Hash + Eq + MemGraphId + Copy, NID1: std::hash::Hash + Eq + MemGraphId + Copy,
    EID0: std::hash::Hash + Eq + MemGraphId + Copy, EID1: std::hash::Hash + Eq + MemGraphId + Copy,
    Graph0: GraphContainerTrait<NID0, EID0, N0, R0> + GraphIteratorTrait<NID0, EID0>,
    Graph1: GrowableGraphContainerTrait<NID1, EID1, N1, R1> + GrowableGraphIteratorTrait<NID1, EID1>,
    VCOMP: Fn(&N0, &N1) -> bool, ECOMP: Fn(&R0, &R1) -> bool,
    CALLBACK: FnMut(&HashMap<NID0, NID1>, &HashMap<NID1, NID0>, &Graph0, &mut Graph1) -> Option<bool> {

        pub fn new(graph_0: &'g0 Graph0, graph_1: &'g1 mut Graph1, vcomp: VCOMP, ecomp: ECOMP, callback: CALLBACK) -> Self {
            Matcher {
                state: State::new(graph_0, graph_1, vcomp, ecomp),
                found_match: false,
                match_continuation: Vec::new(),
                first_candidate_0: None,
                callback: callback,
            }
        }

        fn back_track(&mut self) {
            let last =  self.match_continuation.pop();
            if let Some(back) = last {
                self.state.pop(&back.0, &back.1);
            }
        }

        pub fn process(&mut self, ids0: Vec<NID0>, ids1: Vec<NID1>) -> Option<bool> {
            let mut it0;
            let mut it1 = ids1.iter();
            let mut state = IterationStates::Process;
            let mut init_set = HashSet::<NID0>::new();
            loop {
                match state {
                    IterationStates::Process => {
                        if self.state.success() {
                            self.found_match = true;
                            if !self.state.call_back(&mut self.callback)? {
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
                        it0 = ids0.iter();
                        while let Some(id0) = it0.next() {
                            if self.state.possible_candidate_0(id0) && (!init_set.contains(id0) || !self.match_continuation.is_empty()) {
                                if self.match_continuation.is_empty() {
                                    init_set.insert(*id0);
                                }
                                self.first_candidate_0 = Some(*id0);
                                break;
                            }
                        }
                        state = IterationStates::InitGraph1Loop;
                    },
                    IterationStates::InitGraph1Loop => {
                        it1 = ids1.iter();
                        state = IterationStates::Graph1Loop;
                    },
                    IterationStates::Graph1Loop => {
                        let mut backtrack = true;
                        if let Some(id0) = &self.first_candidate_0 {
                            while let Some(id1) = it1.next() {
                                if self.state.possible_candidate_1(&id1) && self.state.feasible(id0, id1)? {
                                    self.match_continuation.push((*id0, *id1));
                                    self.state.push(id0, id1);
                                    backtrack = false;
                                    break;
                                }
                            }
                        }
                        if !backtrack {
                            state = IterationStates::Process;
                        } else {
                            state = IterationStates::Backtrack;
                        }
                    },
                    IterationStates::Backtrack => {
                        if self.match_continuation.is_empty() {
                            if init_set.len() == ids0.len() {
                                return Some(self.found_match);
                            } else {
                                state = IterationStates::LookForCandidates;
                            }
                        } else {
                            self.back_track();
                            state = IterationStates::Graph1Loop;
                        }
                    }
                }
            }
        }
    }

fn sort_nodes<'g, NID, EID, Graph>(graph: &'g Graph) -> Vec<NID> 
where NID: std::hash::Hash + Eq + MemGraphId + Copy,
EID: std::hash::Hash + Eq + MemGraphId + Copy,
Graph: GraphIteratorTrait<NID, EID> + GraphTrait<NID, EID> {
    let mut res = graph.get_nodes_ids();
    res.sort_by(|a, b| (graph.in_degree(b) + graph.out_degree(b)).cmp(&(graph.in_degree(a) + graph.out_degree(a))));
    res
}

pub fn sub_graph_isomorphism<'g0: 'g1, 'g1, NID0: 'g1, NID1: 'g1, EID0: 'g1, EID1: 'g1, N0: 'g1, R0: 'g1, N1: 'g1, R1: 'g1, VCOMP, ECOMP, Graph0, Graph1, CALLBACK>
(graph_0: &'g0 Graph0, graph_1: &'g1 mut Graph1, vcomp: VCOMP, ecomp: ECOMP, callback: CALLBACK) -> Option<bool>
where NID0: std::hash::Hash + Eq + MemGraphId + Copy, NID1: std::hash::Hash + Eq + MemGraphId + Copy,
EID0: std::hash::Hash + Eq + MemGraphId + Copy, EID1: std::hash::Hash + Eq + MemGraphId + Copy,
Graph0: GraphContainerTrait<NID0, EID0, N0, R0>,
Graph0: GraphIteratorTrait<NID0, EID0>,
Graph1: GrowableGraphContainerTrait<NID1, EID1, N1, R1>,
Graph1: GrowableGraphIteratorTrait<NID1, EID1>,
VCOMP: Fn(&N0, &N1) -> bool, ECOMP: Fn(&R0, &R1) -> bool,
CALLBACK: FnMut(&HashMap<NID0, NID1>, &HashMap<NID1, NID0>, &Graph0, &mut Graph1)-> Option<bool>  {

    let id0 = sort_nodes(graph_0);
    let id1 = graph_1.get_nodes_ids();
    let mut matcher = Matcher::new(graph_0, graph_1, vcomp, ecomp, callback);
    
    matcher.process(id0, id1)
}