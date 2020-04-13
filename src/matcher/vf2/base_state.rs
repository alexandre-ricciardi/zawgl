use std::collections::HashMap;

pub struct BaseState<'g0, 'g1, V0: Eq + std::hash::Hash, V1: Eq + std::hash::Hash> {
    term_in_count: usize,
    term_out_count: usize,
    term_both_count: usize,
    core_count: usize,
    core_map: HashMap<&'g0 V0, &'g1 V1>,
    in_map: HashMap<&'g0 V0, usize>,
    out_map: HashMap<&'g0 V0, usize>,
}

impl <'g0, 'g1, V0: Eq + std::hash::Hash, V1: Eq + std::hash::Hash> BaseState<'g0, 'g1, V0, V1> {
    pub fn push(&mut self, v0: &'g0 V0, v1: &'g1 V1) {
        self.core_count += 1;
        self.core_map.insert(v0, v1);
        if !self.in_map.contains_key(v0) {
            self.in_map.insert(v0, self.core_count);
            self.term_in_count += 1;
            if self.out_map.contains_key(v0) {
                self.term_both_count += 1;
            }
        }
        if !self.out_map.contains_key(v0) {
            self.out_map.insert(v0, self.core_count);
            self.term_out_count += 1;
            if self.in_map.contains_key(v0) {
                self.term_both_count += 1;
            }
        }
    }
}