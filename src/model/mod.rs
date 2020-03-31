use super::graph::*;
pub mod init;

#[derive(Debug, PartialEq, Clone)]
pub enum PropertyValue {
    PString(String),
    PInteger(i64),
    PFloat(f64),
    PBool(bool),
}

pub enum Directive {
    CREATE,
    MATCH,
    DELETE
}

pub struct Property {
    pub id: Option<u64>,
    pub name: Option<String>,
    pub value: Option<PropertyValue>,
}

impl Property {
    pub fn new() -> Self {
        Property { name: None, value: None, id: None }
    } 
}


pub struct Node {
    pub id: Option<u64>,
    pub var: Option<String>,
    pub properties: Vec<Property>,
    pub labels: Vec<String>
}


impl Node {
    pub fn new() -> Self {
        Node {var: None, properties: Vec::new(), labels: Vec::new(), id:None}
    }
}

pub struct Relationship {
    pub id: Option<u64>,
    pub var: Option<String>,
    pub properties: Vec<Property>,
    pub labels: Vec<String>,
}

impl Relationship {
    pub fn new() -> Self {
        Relationship {var: None, properties: Vec::new(), labels: Vec::new(), id: None}
    }
}

pub struct PropertyGraph {
    pub nodes: Vec<Node>,
    pub relationships: Vec<Relationship>,
    pub graph: Graph,
}


pub struct NodeIterator<'graph> {
    graph: &'graph PropertyGraph,
    current_node_index: Option<usize>,
}

impl <'graph> Iterator for NodeIterator<'graph> {
    type Item = (NodeData, Node);

    fn next(&mut self) -> Option<(NodeData, Node)> {
        match self.current_node_index {
            None => None,
            Some(node_index) => {
                let node = &self.graph.nodes[node_index];
                let node_data = &self.graph.graph[node_index];
                self.current_node_index = edge.next_outbound_edge;
                Some(edge.target)
            }
        }
    }
}

impl PropertyGraph {
    pub fn new() -> Self {
        PropertyGraph {nodes: Vec::new(), relationships: Vec::new(), graph: Graph::new()}
    }

    pub fn add_node(&mut self) -> usize {
        let node = Node::new();
        self.nodes.push(node);
        self.graph.add_node()
    }

    pub fn add_relationship(&mut self, source: usize, target: usize) -> usize {
        let rel = Relationship::new();
        self.relationships.push(rel);
        self.graph.add_edge(source, target)
    }

    pub fn get_node_mut(&mut self, id: usize) -> &mut Node {
        &mut self.nodes[id]
    }

    pub fn get_relationship_mut(&mut self, id: usize) -> &mut Relationship {
        &mut self.relationships[id]
    }

    pub fn get_node_ref(&self, id: usize) -> &Node {
        &self.nodes[id]
    }

    pub fn get_relationship_ref(&self, id: usize) -> &Relationship {
        &self.relationships[id]
    }

    pub fn successors(&self, source: usize) -> Successors {
        self.graph.successors(source)
    }
    
    pub fn ancestors(&self, target: usize) -> Ancestors {
        self.graph.ancestors(target)
    }
}

pub struct Request {
    pub pattern: PropertyGraph,
    pub directive: Directive,
}

impl Request {
    pub fn new(directive: Directive) -> Self {
        Request {pattern: PropertyGraph::new(), directive: directive}
    }
}