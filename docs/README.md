# one-graph
Open Source Timelined Graph Database

## Status
* Currently the database core lib enables OpenCypher query language parsing.
* Pattern Matching with a VF2 graph sub-graph isomorphism algorithm.
* Property Graph storage with a B+Tree for indexes and fixed size Records files for Nodes, Relationships and Properties.
* A fixed sized Pager implementation.

## Roadmap
* Improve backend with LLAMA cache and Bw-Tree for indexes.
* Continue where clause implementation, optimize boolean expressions in order to inject constraints in the pattern matching logics.
* Begin frontend implementation to publish a OpenAPI interface.
* Study VF3 version of sub-graph isomorphism algorithm.
* Keep in mind that graph structures may be timelined in order to be able to retrieve past graph states.
* Gremlin support.


## Documentation
* [Cargo doc](http://docs.one-graph.io/og/index.html)