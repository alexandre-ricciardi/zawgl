use zawgl_core::{model::*, graph_engine::GraphEngine};

mod pattern_builder;

use pattern_builder::{build_pattern, merge_patterns};

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


fn make_cartesian_product(pools: &Vec<Vec<PropertyGraph>>) -> Vec<Vec<&PropertyGraph>> {
    let mut res = vec![];
 
    let mut list_iter = pools.iter();
    if let Some(first_list) = list_iter.next() {
        for i in first_list {
            res.push(vec![i]);
        }
    }
    for l in list_iter {
        let mut tmp = vec![];
        for r in res {
            for el in l {
                let mut tmp_el = r.clone();
                tmp_el.push(el);
                tmp.push(tmp_el);
            }
        }
        res = tmp;
    }
    res
}

pub fn handle_query_steps<'a>(steps: &Vec<QueryStep>, graph_engine: &mut GraphEngine) -> Vec<PropertyGraph> {
    let mut results = Vec::<Vec<PropertyGraph>>::new();
    for step in steps {
        match step.step_type {
            StepType::MATCH => {
                if results.is_empty() {
                    for pattern in &step.patterns {
                        let matched = graph_engine.match_pattern(pattern);
                        if let Some(res) = matched {
                            results.push(res);
                        }
                    }
                } else {
                    let mut new_res = Vec::new();
                    for pattern in &step.patterns {
                        let products = make_cartesian_product(&results);
                        for product in &products {
                            let merge_sources = merge_patterns(product);
                            let merge = build_pattern(&merge_sources, pattern);
                            let matched = graph_engine.match_pattern(&merge);
                            if let Some(c) = matched {
                                new_res.push(c);
                            }
                        }
                    }
                    results = new_res;
                }
            },
            StepType::CREATE => {
                if results.is_empty() {
                    for pattern in &step.patterns {
                        let created = graph_engine.match_pattern_and_create(pattern);
                        if let Some(res) = created {
                            results.push(res);
                        }
                    }
                } else {
                    let mut new_res = Vec::new();
                    for pattern in &step.patterns {
                        let products = make_cartesian_product(&results);
                        for product in &products {
                            let merge_sources = merge_patterns(product);
                            let merge = build_pattern(&merge_sources, pattern);
                            let created = graph_engine.match_pattern_and_create(&merge);
                            if let Some(c) = created {
                                new_res.push(c);
                            }
                        }
                    }
                    results = new_res;
                }
            },
            StepType::DELETE => todo!(),
        }
    }
    let mut result = Vec::new();
    for res in &mut results {
        result.append(res);
    }
    result
}