// MIT License
// Copyright (c) 2024 Alexandre RICCIARDI
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


use log::LevelFilter;
use zawgl_core::{model::{init::InitContext, Node, Property, Relationship}, repository::graph_repository::GraphRepository};
use simple_logger::SimpleLogger;

#[test]
fn demo() {
    SimpleLogger::new().with_level(LevelFilter::Info).init().unwrap();
    let ctx: InitContext = InitContext::new("zawgl-db").expect("can't create database context");
    let mut gr = GraphRepository::new(ctx);
    let mut alice = Node::new();
    alice.set_labels(vec!["Person".to_string()]);
    alice.set_properties(vec![Property::new_integer("age", 42)]);
    let alice_with_id = gr.create_node(&alice).unwrap();
    let mut bob = Node::new();
    bob.set_labels(vec!["Person".to_string()]);
    bob.set_properties(vec![Property::new_integer("age", 36)]);
    let bob_with_id = gr.create_node(&bob).unwrap();
    let mut is_friend_rel = Relationship::new();
    is_friend_rel.set_labels(vec!["IS_FRIEND_OF".to_string()]);
    gr.create_relationship(&is_friend_rel, alice_with_id.get_id().unwrap(), bob_with_id.get_id().unwrap());
    gr.sync();

}