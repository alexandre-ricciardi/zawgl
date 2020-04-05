pub type NodeIndex = usize;
pub type EdgeIndex = usize;

pub struct NodeData {
    first_outbound_edge: Option<EdgeIndex>,
    first_inbound_edge: Option<EdgeIndex>,
}

pub struct EdgeData {
    pub source: NodeIndex,
    pub target: NodeIndex,
    pub next_outbound_edge: Option<EdgeIndex>,
    pub next_inbound_edge: Option<EdgeIndex>,
}

pub struct Graph {
    nodes: Vec<NodeData>,
    edges: Vec<EdgeData>,
}

pub struct Successors<'graph> {
    graph: &'graph Graph,
    current_edge_index: Option<EdgeIndex>,
}

pub struct Ancestors<'graph> {
    graph: &'graph Graph,
    current_edge_index: Option<EdgeIndex>,
}

impl <'graph> Iterator for Successors<'graph> {
    type Item = usize;

    fn next(&mut self) -> Option<NodeIndex> {
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

    fn next(&mut self) -> Option<NodeIndex> {
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

    pub fn add_node(&mut self) -> NodeIndex {
        let index = self.nodes.len();
        self.nodes.push(NodeData{first_outbound_edge: None, first_inbound_edge: None});
        index
    }

    pub fn get_node(&self, id: NodeIndex) -> &NodeData {
        &self.nodes[id]
    }
    pub fn get_edge(&self, id: EdgeIndex) -> &EdgeData {
        &self.edges[id]
    }

    pub fn add_edge(&mut self, source: NodeIndex, target: NodeIndex) -> EdgeIndex {
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

    pub fn successors(&self, source: NodeIndex) -> Successors {
        let first_outbound_edge = self.nodes[source].first_outbound_edge;
        Successors{ graph: self, current_edge_index: first_outbound_edge }
    }
    
    pub fn ancestors(&self, target: NodeIndex) -> Ancestors {
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

        let ed0 = graph.get_edge(e0);
        assert_eq!(ed0.source, n0);
        assert_eq!(ed0.target, n1);
        assert_eq!(ed0.next_outbound_edge, None);
        
        let nd0 = graph.get_node(n0);
        assert_eq!(nd0.first_outbound_edge, Some(e2));

        let ed2 = graph.get_edge(e2);
        assert_eq!(ed2.source, n0);
        assert_eq!(ed2.target, n2);
        assert_eq!(ed2.next_outbound_edge, Some(e0));

        let targets: Vec<usize> = graph.successors(n0).collect();
        assert_eq!(targets[0], n2);
        assert_eq!(targets[1], n1);
        assert_eq!(targets.len(), 2);

        let sources: Vec<usize> = graph.ancestors(n2).collect();
        assert_eq!(sources.len(), 2);

    }
}