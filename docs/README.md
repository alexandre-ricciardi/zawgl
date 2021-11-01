# one-graph
Open Source Timelined Graph Database  
OneGraph is in WIP stage, though it can already expose a few Gremlin features.

## Status
* Pattern Matching with a VF2 graph sub-graph isomorphism algorithm.
* Property Graph storage with a B+Tree for indexes and fixed size Records files for Nodes, Relationships and Properties.
* A fixed sized Pager implementation.

## Roadmap
* Improve backend with LLAMA cache and Bw-Tree for indexes.
* Begin frontend implementation to publish a OpenAPI interface.
* Study VF3 version of sub-graph isomorphism algorithm.
* Keep in mind that graph structures may be timelined in order to be able to retrieve past graph states.
* Improve Gremlin support.


## Documentation
* [Cargo doc](http://docs.one-graph.io/og)
