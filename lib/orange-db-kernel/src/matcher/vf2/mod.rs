mod base_state;
mod state;

use std::collections::HashMap;
use self::state::State;
use super::super::graph::traits::*;

pub struct Matcher<'g0: 'g1, 'g1, NID0, NID1, EID0, EID1, N0, R0, N1, R1, VCOMP, ECOMP, Graph0, Graph1, CALLBACK>
    where NID0: std::hash::Hash + Eq + MemGraphId + Copy, NID1: std::hash::Hash + Eq + MemGraphId + Copy,
    EID0: std::hash::Hash + Eq + MemGraphId + Copy, EID1: std::hash::Hash + Eq + MemGraphId + Copy, 
    N0: std::hash::Hash + Eq, R0: std::hash::Hash + Eq, 
    N1: std::hash::Hash + Eq, R1: std::hash::Hash + Eq, 
    Graph0: GraphContainerTrait<NID0, EID0, N0, R0> + GraphIteratorTrait<NID0, EID0>,
    Graph1: GraphContainerTrait<NID1, EID1, N1, R1> + GrowableGraph<NID1, EID1> + GraphIteratorTrait<NID1, EID1>,
    VCOMP: Fn(&N0, &N1) -> bool, ECOMP: Fn(&R0, &R1) -> bool,
    CALLBACK: FnMut(&HashMap<NID0, NID1>, &HashMap<NID1, NID0>, &Graph0, &Graph1) -> bool  {
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
    N0: std::hash::Hash + Eq, R0: std::hash::Hash + Eq, 
    N1: std::hash::Hash + Eq, R1: std::hash::Hash + Eq, 
    Graph0: GraphContainerTrait<NID0, EID0, N0, R0> + GraphIteratorTrait<NID0, EID0>,
    Graph1: GraphContainerTrait<NID1, EID1, N1, R1> + GrowableGraph<NID1, EID1> + GraphIteratorTrait<NID1, EID1>,
    VCOMP: Fn(&N0, &N1) -> bool, ECOMP: Fn(&R0, &R1) -> bool,
    CALLBACK: FnMut(&HashMap<NID0, NID1>, &HashMap<NID1, NID0>, &Graph0, &Graph1) -> bool {

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
            if let Some(back) = self.match_continuation.pop() {
                self.state.pop(&back.0, &back.1);
                self.graph_1_loop();
            }
        }

        fn graph_1_loop(&mut self) {
            if let Some(id0) = self.first_candidate_0 {
                for next_candidate_1_id in self.curr_candidate_1_index..self.graph_1_ids.len() {
                    let id1 = self.graph_1_ids[next_candidate_1_id];
                    self.curr_candidate_1_index += 1;
                    if self.state.possible_candidate_1(&id1) && self.state.feasible(&id0, &id1) {
                        self.match_continuation.push((id0, id1));
                        self.state.push(&id0, &id1);
                        self.process();
                    }
                }
            }
        }

        pub fn process(&mut self) -> bool {
            let mut backtrack = false;
            loop {
                if self.state.success() {
                    if !self.state.call_back(&mut self.callback) {
                        return true;
                    } else {
                        self.found_match = true;
                        backtrack = true;
                    }
                }
                if !backtrack && !self.state.valid() {
                    backtrack = true;
                }
                if !backtrack {
                    if let Some(nid) = self.graph_0_ids.iter().find(|nid| self.state.possible_candidate_0(nid)) {
                        self.first_candidate_0 = Some(*nid);
                    }
                    self.curr_candidate_1_index = 0;
                    self.graph_1_loop();
                }
                if self.match_continuation.is_empty() {
                    return self.found_match;
                }
                self.back_track();
            }
        }

    }

fn sort_nodes<'g, NID, EID, N, R, Graph>(graph: &'g Graph) -> Vec<NID> 
where NID: std::hash::Hash + Eq + MemGraphId + Copy,
EID: std::hash::Hash + Eq + MemGraphId + Copy,
N: std::hash::Hash + Eq, R: std::hash::Hash + Eq,
Graph: GraphContainerTrait<NID, EID, N, R> {
    let mut res = graph.get_nodes_ids();
    res.sort_by(|a, b| (graph.in_degree(b) + graph.out_degree(b)).cmp(&(graph.in_degree(a) + graph.out_degree(a))));
    res
}

pub fn sub_graph_isomorphism<'g0: 'g1, 'g1, NID0: 'g1, NID1: 'g1, EID0: 'g1, EID1: 'g1, N0: 'g1, R0: 'g1, N1: 'g1, R1: 'g1, VCOMP, ECOMP, Graph0, Graph1, CALLBACK>
(graph_0: &'g0 Graph0, graph_1: &'g1 mut Graph1, vcomp: VCOMP, ecomp: ECOMP, callback: CALLBACK) -> bool
where NID0: std::hash::Hash + Eq + MemGraphId + Copy, NID1: std::hash::Hash + Eq + MemGraphId + Copy,
EID0: std::hash::Hash + Eq + MemGraphId + Copy, EID1: std::hash::Hash + Eq + MemGraphId + Copy, 
N0: std::hash::Hash + Eq, R0: std::hash::Hash + Eq, 
N1: std::hash::Hash + Eq, R1: std::hash::Hash + Eq, 
Graph0: GraphContainerTrait<NID0, EID0, N0, R0>,
Graph0: GraphIteratorTrait<NID0, EID0>,
Graph1: GraphContainerTrait<NID1, EID1, N1, R1> + GrowableGraph<NID1, EID1>,
Graph1: GraphIteratorTrait<NID1, EID1>,
VCOMP: Fn(&N0, &N1) -> bool, ECOMP: Fn(&R0, &R1) -> bool,
CALLBACK: FnMut(&HashMap<NID0, NID1>, &HashMap<NID1, NID0>, &Graph0, &Graph1)-> bool  {

    let mut matcher = Matcher::new(graph_0, graph_1, vcomp, ecomp, callback);
    matcher.process()
}