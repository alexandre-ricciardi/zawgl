mod base_state;
mod state;

use std::collections::HashMap;
use self::state::State;
use super::super::graph::traits::*;

pub struct Matcher<'g0, 'g1, NID0, NID1, EID0, EID1, N0, R0, N1, R1, VCOMP, ECOMP, Graph0, Graph1, CALLBACK>
    where NID0: std::hash::Hash + Eq + MemGraphId + Copy, NID1: std::hash::Hash + Eq + MemGraphId + Copy,
    EID0: std::hash::Hash + Eq + MemGraphId + Copy, EID1: std::hash::Hash + Eq + MemGraphId + Copy, 
    N0: std::hash::Hash + Eq, R0: std::hash::Hash + Eq, 
    N1: std::hash::Hash + Eq, R1: std::hash::Hash + Eq, 
    Graph0: GraphContainerTrait<'g0, NID0, EID0, N0, R0>,
    Graph1: GraphContainerTrait<'g1, NID1, EID1, N1, R1>,
    VCOMP: Fn(&N0, &N1) -> bool, ECOMP: Fn(&R0, &R1) -> bool,
    CALLBACK: Fn(&HashMap<NID0, NID1>, &HashMap<NID1, NID0>) -> bool  {
        graph_0: &'g0 Graph0,
        graph_1: &'g1 Graph1,
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
    Graph0: GraphContainerTrait<'g0, NID0, EID0, N0, R0>,
    Graph1: GraphContainerTrait<'g1, NID1, EID1, N1, R1>,
    VCOMP: Fn(&N0, &N1) -> bool, ECOMP: Fn(&R0, &R1) -> bool,
    CALLBACK: Fn(&HashMap<NID0, NID1>, &HashMap<NID1, NID0>) -> bool {

        fn new(graph_0: &'g0 Graph0, graph_1: &'g1 Graph1, vcomp: VCOMP, ecomp: ECOMP, callback: CALLBACK) -> Self {
            Matcher {
                graph_0: graph_0,
                graph_1: graph_1,
                state: State::new(graph_0, graph_1, vcomp, ecomp),
                found_match: false,
                graph_0_ids: sort_nodes(graph_0),
                graph_1_ids: graph_1.get_nodes_ids(),
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
            loop {
                if self.state.success() {
                    if !self.state.call_back(&self.callback) {
                        return true;
                    } else {
                        self.found_match = true;
                        if self.match_continuation.is_empty() {
                            return self.found_match;
                        } else {
                            self.back_track();
                        }
                    }
                }
                if !self.state.valid() {
                    if self.match_continuation.is_empty() {
                        return self.found_match;
                    } else {
                        self.back_track();
                    }
                }

                if let Some(nid) = self.graph_0_ids.iter().find(|nid| self.state.possible_candidate_0(nid)) {
                    self.first_candidate_0 = Some(*nid);
                }

                if self.match_continuation.is_empty() {
                    return self.found_match;
                } else {
                    self.back_track();
                }
            }
        }

    }

    fn sort_nodes<'g, NID, EID, N, R, Graph>(graph: &'g Graph) -> Vec<NID> 
    where NID: std::hash::Hash + Eq + MemGraphId + Copy,
    EID: std::hash::Hash + Eq + MemGraphId + Copy,
    N: std::hash::Hash + Eq, R: std::hash::Hash + Eq,
    Graph: GraphContainerTrait<'g, NID, EID, N, R> {
        let mut res = graph.get_nodes_ids();
        res.sort_by(|a, b| (graph.in_degree(b) + graph.out_degree(b)).cmp(&(graph.in_degree(a) + graph.out_degree(a))));
        res
    }

    pub fn sub_graph_isomorphism<'g0, 'g1, NID0, NID1, EID0, EID1, N0, R0, N1, R1, VCOMP, ECOMP, Graph0, Graph1, CALLBACK>
    (graph_0: &'g0 Graph0, graph_1: &'g1 Graph1, vcomp: VCOMP, ecomp: ECOMP, callback: CALLBACK) -> bool
    where NID0: std::hash::Hash + Eq + MemGraphId + Copy, NID1: std::hash::Hash + Eq + MemGraphId + Copy,
    EID0: std::hash::Hash + Eq + MemGraphId + Copy, EID1: std::hash::Hash + Eq + MemGraphId + Copy, 
    N0: std::hash::Hash + Eq, R0: std::hash::Hash + Eq, 
    N1: std::hash::Hash + Eq, R1: std::hash::Hash + Eq, 
    Graph0: GraphContainerTrait<'g0, NID0, EID0, N0, R0>,
    Graph1: GraphContainerTrait<'g1, NID1, EID1, N1, R1>,
    VCOMP: Fn(&N0, &N1) -> bool, ECOMP: Fn(&R0, &R1) -> bool,
    CALLBACK: Fn(&HashMap<NID0, NID1>, &HashMap<NID1, NID0>) -> bool  {
        if graph_0.nodes_len() > graph_1.nodes_len() {
            false
        } else if graph_0.edges_len() > graph_1.edges_len() {
            false
        } else {
            let mut matcher = Matcher::new(graph_0, graph_1, vcomp, ecomp, callback);
            matcher.process()
        }
    }