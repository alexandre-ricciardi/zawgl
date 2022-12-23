# zawgl
Open Source Timelined Graph Database  
zawgl is in WIP stage, though it can already expose a few Cypher features.

## Status
* Pattern Matching with a VF2 graph sub-graph isomorphism algorithm.
* Property Graph storage with a B+Tree for indexes and fixed size Records files for Nodes, Relationships and Properties.
* A fixed sized Pager implementation.

## Roadmap
* Study VF3 version of sub-graph isomorphism algorithm.
* Keep in mind that graph structures may be timelined in order to be able to retrieve past graph states.
* Javascript procedures.
* Improve open-cypher and gremlin support.

## Memory Model
* [Memory model](model.md)


