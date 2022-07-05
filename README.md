# zawgl
Open Source Timelined Graph Database (Work In Progress)

## Status
* At the moment Zawgl Database supports a few gremlin and cypher queries.
* Pattern Matching with a VF2 sub-graph isomorphism algorithm.
* Property Graph storage
* With a B+Tree for indexes
* Fixed size Records files for Nodes, Relationships and Properties.
* A fixed sized Pager implementation.

## Test
A docker package is available for testing purpose:  
```
docker run -p8182:8182 --rm -it ghcr.io/alexandre-ricciardi/alexandre-ricciardi/zawgl:latest
```

This will expose an endpoint on 8182 port.

## Roadmap
* Study VF3 version of sub-graph isomorphism algorithm.
* Keep in mind that graph structures may be timelined in order to be able to retrieve past graph states.

