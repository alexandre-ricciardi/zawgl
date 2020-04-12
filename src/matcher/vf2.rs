pub struct BaseState {
    term_in_count: usize,
    term_out_count: usize,
    term_both_count: usize,
    core_count: usize,
}

impl BaseState {
    pub fn push(&mut self, vertex_this: usize, vertex_other: usize) {
        self.core_count += 1;
    }
}