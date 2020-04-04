pub struct NodeData {
    first_outbound_edge: Option<usize>,
    first_inbound_edge: Option<usize>,
}

pub struct EdgeData {
    pub source: usize,
    pub target: usize,
    pub next_outbound_edge: Option<usize>,
    pub next_inbound_edge: Option<usize>,
}

pub struct Graph {
    nodes: Vec<NodeData>,
    edges: Vec<EdgeData>,
}

pub struct Successors<'graph> {
    graph: &'graph Graph,
    current_edge_index: Option<usize>,
}

pub struct Ancestors<'graph> {
    graph: &'graph Graph,
    current_edge_index: Option<usize>,
}

impl <'graph> Iterator for Successors<'graph> {
    type Item = usize;

    fn next(&mut self) -> Option<usize> {
        match self.current_edge_index {
            None => None,
            Some(edge_index) => {
                let edge = &self.graph.edges[edge_index];
                self.current_edge_index = edge.next_outbound_edge;
                Some(edge.target)
            }
        }
    }
}

impl <'graph> Iterator for Ancestors<'graph> {
    type Item = usize;

    fn next(&mut self) -> Option<usize> {
        match self.current_edge_index {
            None => None,
            Some(edge_index) => {
                let edge = &self.graph.edges[edge_index];
                self.current_edge_index = edge.next_inbound_edge;
                Some(edge.source)
            }
        }
    }
}

impl Graph {
    pub fn new() -> Self {
        Graph{ nodes: Vec::new(), edges: Vec::new() }
    }

    pub fn add_node(&mut self) -> usize {
        let index = self.nodes.len();
        self.nodes.push(NodeData{first_outbound_edge: None, first_inbound_edge: None});
        index
    }

    pub fn get_node(&self, id: usize) -> &NodeData {
        &self.nodes[id]
    }
    pub fn get_edge(&self, id: usize) -> &EdgeData {
        &self.edges[id]
    }

    pub fn add_edge(&mut self, source: usize, target: usize) -> usize {
        let index = self.edges.len();
        {
            let source_data = &self.nodes[source];
            let target_data = &self.nodes[target];
            self.edges.push(EdgeData{source: source, target: target,
                 next_inbound_edge: target_data.first_inbound_edge, 
                 next_outbound_edge: source_data.first_outbound_edge});
        }
        
        let ms = &mut self.nodes[source];
        ms.first_outbound_edge = Some(index);
        let mt = &mut self.nodes[target];
        mt.first_inbound_edge = Some(index);
        index
    }

    pub fn successors(&self, source: usize) -> Successors {
        let first_outbound_edge = self.nodes[source].first_outbound_edge;
        Successors{ graph: self, current_edge_index: first_outbound_edge }
    }
    
    pub fn ancestors(&self, target: usize) -> Ancestors {
        let first_inbound_edge = self.nodes[target].first_inbound_edge;
        Ancestors{ graph: self, current_edge_index: first_inbound_edge }
    }

    pub fn get_nodes(&self) -> &Vec<NodeData> {
        &self.nodes
    }
    pub fn get_edges(&self) -> &Vec<EdgeData> {
        &self.edges
    }
}

#[cfg(test)]
mod test_graph {
    use super::*;
    #[test]
    fn test_small_graph_it() {
        let mut graph = Graph::new();
        let n0 = graph.add_node();
        let n1 = graph.add_node();
        let n2 = graph.add_node();

        let e0 = graph.add_edge(n0, n1);
        let e1 = graph.add_edge(n1, n2);
        let e2 = graph.add_edge(n0, n2);

        let mut count = 0;
        for e in graph.successors(n0) {
            count += 1;
        }

        assert_eq!(count, 2);

        count = 0;
        for e in graph.ancestors(n2) {
            count += 1;
        }

        assert_eq!(count, 2);
    }
}