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

use super::*;
use super::traits::*;

pub struct GraphContainer<NODE: Clone, RELATIONSHIP: Clone> {
    graph: Graph<NODE, RELATIONSHIP>,
}

impl <N: Clone, R: Clone> Clone for GraphContainer<N, R> {
    fn clone(&self) -> Self {
        Self { graph: self.graph.clone() }
    }
}

impl <NODE: Clone, RELATIONSHIP: Clone> GraphContainer<NODE, RELATIONSHIP> {

    pub fn get_node_mut(&mut self, id: &NodeIndex) -> &mut NODE {
        &mut self.graph.vertices[id.get_index()].node
    }

    pub fn get_relationship_mut(&mut self, id: &EdgeIndex) -> &mut RELATIONSHIP {
        &mut self.graph.edges[id.get_index()].relationship
    }

    pub fn get_node_ref(&self, id: &NodeIndex) -> &NODE {
        &self.graph.vertices[id.get_index()].node
    }

    pub fn get_relationship_ref(&self, id: &EdgeIndex) -> &RELATIONSHIP {
        &self.graph.edges[id.get_index()].relationship
    }
}

impl <NODE: Clone, RELATIONSHIP: Clone> GraphContainer<NODE, RELATIONSHIP> {
    
    pub fn out_edges(&self, source: &NodeIndex) -> OutEdges<'_, RELATIONSHIP> {
        self.get_inner_graph().out_edges(source)
    }

    pub fn in_edges(&self, target: &NodeIndex) -> InEdges<'_, RELATIONSHIP> {
        self.get_inner_graph().in_edges(target)
    }
    pub fn in_degree(&self, node: &NodeIndex) -> usize {
        self.in_edges(node).count()
    }
    pub fn out_degree(&self, node: &NodeIndex) -> usize {
        self.out_edges(node).count()
    }
}

impl <NODE: Clone, RELATIONSHIP: Clone> GraphContainer<NODE, RELATIONSHIP> {

    pub fn get_source_index(&self, edge_index: &EdgeIndex) -> NodeIndex {
        self.get_inner_graph().get_source_index(edge_index)
    }
    pub fn get_target_index(&self, edge_index: &EdgeIndex) -> NodeIndex {
        self.get_inner_graph().get_target_index(edge_index)
    }
    pub fn nodes_len(&self) -> usize {
        self.graph.vertices.len()
    }
    pub fn edges_len(&self) -> usize {
        self.graph.edges_len()
    }
    pub fn get_nodes_ids(&self) -> Vec<NodeIndex> {
        (0..self.nodes_len()).map(NodeIndex::new).collect()
    }
}
impl<NODE: Clone, RELATIONSHIP: Clone> Default for GraphContainer<NODE, RELATIONSHIP> {
    fn default() -> Self {
        Self::new()
    }
}
impl <NODE: Clone, RELATIONSHIP: Clone> GraphContainer<NODE, RELATIONSHIP> {
    pub fn new() -> Self {
        GraphContainer {graph: Graph::new()}
    }
    pub fn add_node(&mut self, node: NODE) -> NodeIndex {
        self.graph.add_vertex(node)
    }

    pub fn add_relationship(&mut self, rel: RELATIONSHIP, source: NodeIndex, target: NodeIndex) -> EdgeIndex {
        self.graph.add_edge(rel, source, target)
    }
    
    pub fn insert_node(&mut self, node: NODE, edge_index: &EdgeIndex) -> (EdgeIndex, EdgeIndex) {
        self.graph.insert_vertex(node, edge_index)
    }

    pub fn get_inner_graph(&self) -> &Graph<NODE, RELATIONSHIP> {
        &self.graph
    }

    pub fn get_relationships_and_edges(&self) -> Vec<&EdgeData<NodeIndex, EdgeIndex, RELATIONSHIP>> {
        self.get_edges()
    }

    pub fn get_nodes_with_ids(&self) -> Vec<(&NODE, NodeIndex)> {
        self.graph.vertices.iter().map(|v| &v.node).zip(self.graph.get_nodes_ids()).collect()
    }

    pub fn get_relationships(&self) -> Vec<&RELATIONSHIP> {
        self.graph.edges.iter().map(|e| &e.relationship).collect()
    }

    pub fn get_relationships_mut(&mut self) -> Vec<&mut RELATIONSHIP> {
        self.graph.edges.iter_mut().map(|e| &mut e.relationship).collect()
    }

    pub fn get_edges(&self) -> Vec<&EdgeData<NodeIndex, EdgeIndex, RELATIONSHIP>> {
        self.graph.edges.iter().filter(|e| !e.discard).collect::<Vec<&EdgeData<NodeIndex, EdgeIndex, RELATIONSHIP>>>()
    }

    pub fn get_edges_mut(&mut self) -> &mut Vec<EdgeData<NodeIndex, EdgeIndex, RELATIONSHIP>> {
        &mut self.graph.edges
    }
    pub fn get_nodes(&self) -> Vec<&NODE> {
        self.graph.vertices.iter().map(|v| &v.node).collect()
    }

    pub fn get_nodes_mut(&mut self) -> Vec<&mut NODE> {
        self.graph.vertices.iter_mut().map(|v| &mut v.node).collect()
    }
}
