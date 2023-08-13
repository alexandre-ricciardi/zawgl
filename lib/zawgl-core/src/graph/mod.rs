// MIT License
// Copyright (c) 2022 Alexandre RICCIARDI
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

pub mod traits;
pub mod container;

use self::traits::*;

#[derive(PartialEq, Eq, Copy, Clone, Debug, Hash)]
pub struct EdgeIndex {
    index: usize,
}

impl EdgeIndex {
    pub fn new(index: usize) -> Self {
        EdgeIndex {index}
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
        NodeIndex {index}
    }
}

impl traits::MemGraphId for NodeIndex {
    fn get_index(&self) -> usize {
        self.index
    }
}

#[derive(Clone)]
pub struct VertexData<EID: MemGraphId, N> {
    pub first_outbound_edge: Option<EID>,
    pub first_inbound_edge: Option<EID>,
    pub node: N,
}

impl <EID: MemGraphId + Copy, N> VertexData<EID, N> {
    pub fn new(n: N) -> Self {
        VertexData{first_outbound_edge: None, first_inbound_edge: None, node: n}
    }
    pub fn get_first_outbound_edge(&self) -> Option<EID> {
        self.first_outbound_edge
    }
    pub fn get_first_inbound_edge(&self) -> Option<EID> {
        self.first_inbound_edge
    }
}

#[derive(Debug, Clone)]
pub struct EdgeData<NID: MemGraphId, EID: MemGraphId, R: Clone> {
    pub id: EID,
    pub source: NID,
    pub target: NID,
    pub next_outbound_edge: Option<EID>,
    pub next_inbound_edge: Option<EID>,
    pub relationship: R,
}

impl <NID: MemGraphId + Copy, EID: MemGraphId + Copy, R: Clone> EdgeData<NID, EID, R> {
    pub fn get_source(&self) -> NID {
        self.source
    }
    pub fn get_target(&self) -> NID {
        self.target
    }

    pub fn get_next_outbound_edge(&self) -> Option<EID> {
        self.next_outbound_edge
    }
    pub fn get_next_inbound_edge(&self) ->  Option<EID> {
        self.next_inbound_edge
    }
}

pub struct Graph<N: Clone, R: Clone> {
    vertices: Vec<VertexData<EdgeIndex, N>>,
    edges: Vec<EdgeData<NodeIndex, EdgeIndex, R>>,
}

impl <N: Clone, R: Clone> Clone for Graph<N, R> {
    fn clone(&self) -> Self {
        Graph::new_clone(self.vertices.clone(), self.edges.clone())
    }
}

pub struct OutEdges<'a, R: Clone> {
    edges: &'a Vec<EdgeData<NodeIndex, EdgeIndex, R>>,
    current_edge_index: Option<EdgeIndex>,
}

impl <R: Clone> Iterator for OutEdges<'_, R> {
    type Item = EdgeIndex;

    fn next(&mut self) -> Option<EdgeIndex> {
        match self.current_edge_index {
            None => None,
            Some(edge_index) => {
                let edge = &self.edges[edge_index.get_index()];
                let curr_edge_index = self.current_edge_index;
                self.current_edge_index = edge.next_outbound_edge;
                curr_edge_index
            }
        }
    }
}


pub struct InEdges<'a, R: Clone> {
    edges: &'a Vec<EdgeData<NodeIndex, EdgeIndex, R>>,
    current_edge_index: Option<EdgeIndex>,
}

impl <'a, R: Clone> Iterator for InEdges<'a, R> {
    type Item = EdgeIndex;

    fn next(&mut self) -> Option<Self::Item> {
        match self.current_edge_index {
            None => None,
            Some(edge_index) => {
                let edge = &self.edges[edge_index.get_index()];
                let curr_edge_index = self.current_edge_index;
                self.current_edge_index = edge.next_inbound_edge;
                curr_edge_index
            }
        }
    }
}

impl <N: Clone, R: Clone> Graph<N, R> {
    fn out_edges(&self, source: &NodeIndex) -> OutEdges<'_, R> {
        let first_outbound_edge = self.vertices[source.get_index()].first_outbound_edge;
        OutEdges{ edges: &self.edges, current_edge_index: first_outbound_edge }
    }

    fn in_edges(&self, target: &NodeIndex) -> InEdges<'_, R> {
        let first_inbound_edge = self.vertices[target.get_index()].first_inbound_edge;
        InEdges{ edges: &self.edges, current_edge_index: first_inbound_edge }
    }
    fn in_degree(&self, node: &NodeIndex) -> usize {
        self.in_edges(node).count()
    }
    fn out_degree(&self, node: &NodeIndex) -> usize {
        self.out_edges(node).count()
    }
}

impl <N: Clone, R: Clone> GraphTrait<NodeIndex, EdgeIndex> for Graph<N, R> {
    fn get_source_index(&self, edge_index: &EdgeIndex) -> NodeIndex {
        self.edges[edge_index.get_index()].source
    }
    fn get_target_index(&self, edge_index: &EdgeIndex) -> NodeIndex {
        self.edges[edge_index.get_index()].target
    }

    fn nodes_len(&self) -> usize {
        self.vertices.len()
    }

    fn edges_len(&self) -> usize {
        self.edges.len()
    }

    fn get_nodes_ids(&self) -> Vec<NodeIndex> {
        (0..self.nodes_len()).map(NodeIndex::new).collect()
    }
    
}
impl<N: Clone, R: Clone> Default for Graph<N, R> {
    fn default() -> Self {
        Self::new()
    }
}
impl <N: Clone, R: Clone> Graph<N, R> {
    pub fn new() -> Self {
        Graph{ vertices: Vec::new(), edges: Vec::new() }
    }

    fn new_clone(nodes: Vec<VertexData<EdgeIndex, N>>, edges: Vec<EdgeData<NodeIndex, EdgeIndex, R>>) -> Self {
        Graph{ vertices: nodes, edges }
    }

    pub fn add_vertex(&mut self, node: N) -> NodeIndex {
        let index = self.vertices.len();
        self.vertices.push(VertexData::<EdgeIndex, N>{first_outbound_edge: None, first_inbound_edge: None, node});
        NodeIndex::new(index)
    }

    pub fn get_vertex(&self, id: NodeIndex) -> &VertexData<EdgeIndex, N> {
        &self.vertices[id.get_index()]
    }
    pub fn get_edge_data(&self, id: EdgeIndex) -> EdgeData<NodeIndex, EdgeIndex, R> {
        self.edges[id.get_index()].clone()
    }

    pub fn add_edge(&mut self, rel: R, source: NodeIndex, target: NodeIndex) -> EdgeIndex {
        let index = self.edges.len();
        {
            let source_data = &self.vertices[source.get_index()];
            let target_data = &self.vertices[target.get_index()];
            self.edges.push(EdgeData{id: EdgeIndex::new(index),
                source, target,
                next_inbound_edge: target_data.first_inbound_edge, 
                next_outbound_edge: source_data.first_outbound_edge,
                relationship: rel,
            });
        }
        
        let ms = &mut self.vertices[source.get_index()];
        ms.first_outbound_edge = Some(EdgeIndex::new(index));
        let mt = &mut self.vertices[target.get_index()];
        mt.first_inbound_edge = Some(EdgeIndex::new(index));
        EdgeIndex::new(index)
    }
}

#[cfg(test)]
mod test_graph {
    use super::*;
    #[test]
    fn test_small_graph_it() {
        let mut graph = Graph::new();
        let n0 = graph.add_vertex(1);
        let n1 = graph.add_vertex(2);
        let n2 = graph.add_vertex(3);

        let e0 = graph.add_edge(4, n0, n1);
        let _e1 = graph.add_edge(4, n1, n2);
        let e2 = graph.add_edge(5, n0, n2);

        let ed0 = graph.get_edge_data(e0);
        assert_eq!(ed0.source, n0);
        assert_eq!(ed0.target, n1);
        assert_eq!(ed0.next_outbound_edge, None);
        
        let nd0 = graph.get_vertex(n0);
        assert_eq!(nd0.first_outbound_edge, Some(e2));

        let ed2 = graph.get_edge_data(e2);
        assert_eq!(ed2.source, n0);
        assert_eq!(ed2.target, n2);
        assert_eq!(ed2.next_outbound_edge, Some(e0));

    }
}