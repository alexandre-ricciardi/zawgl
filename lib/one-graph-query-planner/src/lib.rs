use one_graph_core::{model::{*, init::InitContext}, graph_engine::GraphEngine};

pub enum StepType {
    MATCH, CREATE, DELETE
}

pub struct QueryStep {
    pub patterns: Vec<PropertyGraph>,
    pub step_type: StepType,
}

impl QueryStep {
    pub fn new(step_type: StepType) -> Self {
        QueryStep {step_type: step_type, patterns: Vec::new()}
    }
}

pub struct QueryResult {
    pub patterns: Vec<PropertyGraph>,
}

pub fn handle_query_steps<'a>(conf: &InitContext<'a>, steps: &Vec<QueryStep>) -> Vec<PropertyGraph> {
    let mut graph_engine = GraphEngine::new(conf);
    
}