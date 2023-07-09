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

pub trait MemGraphId {
    fn get_index(&self) -> usize;
}

pub trait GraphTrait<NodeId: MemGraphId, EdgeId: MemGraphId> {
    fn get_source_index(&self, edge_index: &EdgeId) -> NodeId;
    fn get_target_index(&self, edge_index: &EdgeId) -> NodeId;
    fn nodes_len(&self) -> usize;
    fn edges_len(&self) -> usize;
    fn get_nodes_ids(&self) -> Vec<NodeId>;
}


pub trait GraphContainerTrait<NID: MemGraphId, EID: MemGraphId, NODE, RELATIONSHIP>: GraphTrait<NID, EID> {
    fn get_node_mut(&mut self, id: &NID) -> &mut NODE;
    fn get_relationship_mut(&mut self, id: &EID) -> &mut RELATIONSHIP;
    fn get_node_ref(&self, id: &NID) -> &NODE;
    fn get_relationship_ref(&self, id: &EID) -> &RELATIONSHIP;
}

pub trait GrowableGraphTrait<NodeId: MemGraphId, EdgeId: MemGraphId, > {
    fn get_source_index(&self, edge_index: &EdgeId) -> NodeId;
    fn get_target_index(&self, edge_index: &EdgeId) -> NodeId;
    fn nodes_len(&self) -> usize;
    fn edges_len(&self) -> usize;
    fn get_nodes_ids(&self) -> Vec<NodeId>;
}



pub trait GrowableGraphContainerTrait<NID: MemGraphId, EID: MemGraphId, NODE, RELATIONSHIP>: GrowableGraphTrait<NID, EID> {
    fn get_node_ref(&mut self, id: &NID) -> Option<&NODE>;
    fn get_relationship_ref(&mut self, id: &EID) -> Option<&RELATIONSHIP>;
}
