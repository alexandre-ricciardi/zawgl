pub mod traits;
pub mod container;

use self::traits::*;

#[derive(PartialEq, Eq, Copy, Clone, Debug, Hash)]
pub struct EdgeIndex {
    index: usize,
}

impl EdgeIndex {
    pub fn new(index: usize) -> Self {
        EdgeIndex {index: index}
    }
}

impl traits::MemGraphId for EdgeIndex {
    fn get_index(&self) -> usize {
        self.index
    }
}

#[derive(PartialEq, Eq, Copy, Clone, Debug, Hash)]
pub struct NodeIndex {
    index: usize,
}

impl NodeIndex {
    pub fn new(index: usize) -> Self {
        NodeIndex {index: index}
    }
}

impl traits::MemGraphId for NodeIndex {
    fn get_index(&self) -> usize {
        self.index
    }
}

pub struct NodeData<EID: MemGraphId> {
    first_outbound_edge: Option<EID>,
    first_inbound_edge: Option<EID>,
}

pub struct EdgeData<NID: MemGraphId, EID: MemGraphId> {
    pub source: NID,
    pub target: NID,
    pub next_outbound_edge: Option<EID>,
    pub next_inbound_edge: Option<EID>,
}

pub struct Graph {
    nodes: Vec<NodeData<EdgeIndex>>,
    edges: Vec<EdgeData<NodeIndex, EdgeIndex>>,
}

pub struct Successors<'g> {
    graph: &'g Graph,
    current_edge_index: Option<EdgeIndex>,
}

pub struct Ancestors<'g> {
    graph: &'g Graph,
    current_edge_index: Option<EdgeIndex>,
}

impl <'graph> Iterator for Successors<'graph> {
    type Item = NodeIndex;

    fn next(&mut self) -> Option<NodeIndex> {
        match self.current_edge_index {
            None => None,
            Some(edge_index) => {
                let edge = &self.graph.edges[edge_index.get_index()];
                self.current_edge_index = edge.next_outbound_edge;
                Some(edge.target)
            }
        }
    }
}

impl <'graph> Iterator for Ancestors<'graph> {
    type Item = NodeIndex;

    fn next(&mut self) -> Option<NodeIndex> {
        match self.current_edge_index {
            None => None,
            Some(edge_index) => {
                let edge = &self.graph.edges[edge_index.get_index()];
                self.current_edge_index = edge.next_inbound_edge;
                Some(edge.source)
            }
        }
    }
}

pub struct OutEdges<'g> {
    graph: &'g Graph,
    current_edge_index: Option<EdgeIndex>,
}

impl <'g> Iterator for OutEdges<'g> {
    type Item = EdgeIndex;

    fn next(&mut self) -> Option<EdgeIndex> {
        match self.current_edge_index {
            None => None,
            Some(edge_index) => {
                let edge = &self.graph.edges[edge_index.get_index()];
                let curr_edge_index = self.current_edge_index;
                self.current_edge_index = edge.next_outbound_edge;
                curr_edge_index
            }
        }
    }
}


pub struct InEdges<'g> {
    graph: &'g Graph,
    current_edge_index: Option<EdgeIndex>,
}

impl <'graph> Iterator for InEdges<'graph> {
    type Item = EdgeIndex;

    fn next(&mut self) -> Option<Self::Item> {
        match self.current_edge_index {
            None => None,
            Some(edge_index) => {
                let edge = &self.graph.edges[edge_index.get_index()];
                let curr_edge_index = self.current_edge_index;
                self.current_edge_index = edge.next_inbound_edge;
                curr_edge_index
            }
        }
    }
}

impl <'g> GraphTrait<'g, NodeIndex, EdgeIndex> for Graph {
    type OutIt = OutEdges<'g>;
    type InIt = InEdges<'g>;
    fn out_edges(&self, source: &NodeIndex) -> OutEdges {
        let first_outbound_edge = self.nodes[source.get_index()].first_outbound_edge;
        OutEdges{ graph: self, current_edge_index: first_outbound_edge }
    }

    fn in_edges(&self, target: &NodeIndex) -> InEdges {
        let first_inbound_edge = self.nodes[target.get_index()].first_inbound_edge;
        InEdges{ graph: self, current_edge_index: first_inbound_edge }
    }

    fn get_source_index(&self, edge_index: &EdgeIndex) -> &NodeIndex {
        &self.edges[edge_index.get_index()].source
    }
    fn get_target_index(&self, edge_index: &EdgeIndex) -> &NodeIndex {
        &self.edges[edge_index.get_index()].target
    }

    fn get_nodes_len(&self) -> usize {
        self.nodes.len()
    }
}
impl Graph {
    pub fn new() -> Self {
        Graph{ nodes: Vec::new(), edges: Vec::new() }
    }

    pub fn add_node(&mut self) -> NodeIndex {
        let index = self.nodes.len();
        self.nodes.push(NodeData::<EdgeIndex>{first_outbound_edge: None, first_inbound_edge: None});
        NodeIndex::new(index)
    }

    pub fn get_node(&self, id: NodeIndex) -> &NodeData<EdgeIndex> {
        &self.nodes[id.get_index()]
    }
    pub fn get_edge(&self, id: EdgeIndex) -> &EdgeData<NodeIndex, EdgeIndex> {
        &self.edges[id.get_index()]
    }

    pub fn add_edge(&mut self, source: NodeIndex, target: NodeIndex) -> EdgeIndex {
        let index = self.edges.len();
        {
            let source_data = &self.nodes[source.get_index()];
            let target_data = &self.nodes[target.get_index()];
            self.edges.push(EdgeData{source: source, target: target,
                 next_inbound_edge: target_data.first_inbound_edge, 
                 next_outbound_edge: source_data.first_outbound_edge});
        }
        
        let ms = &mut self.nodes[source.get_index()];
        ms.first_outbound_edge = Some(EdgeIndex::new(index));
        let mt = &mut self.nodes[target.get_index()];
        mt.first_inbound_edge = Some(EdgeIndex::new(index));
        EdgeIndex::new(index)
    }

    pub fn successors(&self, source: &NodeIndex) -> Successors {
        let first_outbound_edge = self.nodes[source.get_index()].first_outbound_edge;
        Successors{ graph: self, current_edge_index: first_outbound_edge }
    }
    
    pub fn ancestors(&self, target: &NodeIndex) -> Ancestors {
        let first_inbound_edge = self.nodes[target.get_index()].first_inbound_edge;
        Ancestors{ graph: self, current_edge_index: first_inbound_edge }
    }
    
    pub fn get_nodes(&self) -> &Vec<NodeData<EdgeIndex>> {
        &self.nodes
    }
    pub fn get_edges(&self) -> &Vec<EdgeData<NodeIndex, EdgeIndex>> {
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

        let targets = graph.successors(&n0).collect::<Vec<NodeIndex>>();
        assert_eq!(targets[0], n2);
        assert_eq!(targets[1], n1);
        assert_eq!(targets.len(), 2);

        let sources = graph.ancestors(&n2).collect::<Vec<NodeIndex>>();
        assert_eq!(sources.len(), 2);

    }
}