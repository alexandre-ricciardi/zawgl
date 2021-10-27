mod base_state;
mod state;

use std::collections::HashMap;
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
        graph_0_ids: Vec<NID0>,
        graph_1_ids: Vec<NID1>,
        match_continuation: Vec<(NID0, NID1)>,
        first_candidate_0: Option<NID0>,
        curr_candidate_1_index: usize,
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
            let graph1_ids = graph_1.get_nodes_ids();
            Matcher {
                state: State::new(graph_0, graph_1, vcomp, ecomp),
                found_match: false,
                graph_0_ids: sort_nodes(graph_0),
                graph_1_ids: graph1_ids,
                match_continuation: Vec::new(),
                first_candidate_0: None,
                curr_candidate_1_index: 0,
                callback: callback,
            }
        }

        fn back_track(&mut self) {
            let last =  self.match_continuation.pop();
            if let Some(back) = last {
                self.state.pop(&back.0, &back.1);
            }
        }

        fn graph_1_loop(&mut self) -> Option<bool> {
            if let Some(id0) = self.first_candidate_0 {
                for next_candidate_1_id in self.curr_candidate_1_index..self.graph_1_ids.len() {
                    let id1 = self.graph_1_ids[next_candidate_1_id];
                    if self.state.possible_candidate_1(&id1) && self.state.feasible(&id0, &id1)? {
                        self.match_continuation.push((id0, id1));
                        self.state.push(&id0, &id1);
                        return Some(true);
                    }
                    self.curr_candidate_1_index += 1;
                }
            }
            Some(false)
        }

        pub fn process(&mut self) -> Option<bool> {
            let mut state = IterationStates::Process;
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
                        if let Some(nid) = self.graph_0_ids.iter().find(|nid| self.state.possible_candidate_0(nid)) {
                            self.first_candidate_0 = Some(*nid);
                        }
                        state = IterationStates::InitGraph1Loop;
                    },
                    IterationStates::InitGraph1Loop => {
                        self.curr_candidate_1_index = 0;
                        state = IterationStates::Graph1Loop;
                    },
                    IterationStates::Graph1Loop => {
                        let goto_process = self.graph_1_loop()?;
                        if goto_process {
                            state = IterationStates::Process;
                        } else {
                            state = IterationStates::Backtrack;
                        }
                    },
                    IterationStates::Backtrack => {
                        if self.match_continuation.is_empty() {
                            return Some(self.found_match);
                        }
                        self.back_track();
                        self.curr_candidate_1_index += 1;
                        state = IterationStates::Graph1Loop;
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

    let mut matcher = Matcher::new(graph_0, graph_1, vcomp, ecomp, callback);
    matcher.process()
}