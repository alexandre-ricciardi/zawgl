use one_graph_core::{model::*, graph_engine::GraphEngine};

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

pub fn handle_query_steps<'a>(steps: &Vec<QueryStep>, graph_engine: &mut GraphEngine) -> Vec<PropertyGraph> {
    let mut results = Vec::<PropertyGraph>::new();
    for step in steps {
        match step.step_type {
            StepType::MATCH => {
                results = Vec::<PropertyGraph>::new();
                for pattern in &step.patterns {
                    let mut matched = graph_engine.match_pattern(pattern);
                    if let Some(res) = &mut matched {
                        results.append(res);
                    }
                }
            },
            StepType::CREATE => {
                results = Vec::<PropertyGraph>::new();
                for pattern in &step.patterns {
                    let mut created = graph_engine.match_pattern_and_create(pattern);
                    if let Some(res) = &mut created {
                        results.append(res);
                    }
                }
            },
            StepType::DELETE => todo!(),
        }
    }
    results
}