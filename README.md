# zawgl
Open Source Timelined Graph Database (Work In Progress)


## About Graph Timeline
Timelining feature aims to version graph states, changes made to the graph shall persist in order to allow to retrieve past graph states by passing the graph version ID in the request.



## Status
* At the moment Zawgl Database supports some cypher queries.
* Pattern Matching with a VF2 sub-graph isomorphism algorithm.
* Property Graph storage
* With a B+Tree for indexes
* Fixed size Records files for Nodes, Relationships and Properties.
* A fixed sized Pager implementation.

## Testing
A docker package is available for testing purpose:  
```shell
docker run -p8182:8182 --rm -it ghcr.io/alexandre-ricciardi/alexandre-ricciardi/zawgl:latest
```

This will expose an endpoint on 8182 port.

## Interface
Zawgl exposes a WebSocket on the configured port that transports [Bson](https://crates.io/crates/bson) documents.

Zawgl replies contain a list of graphs representing all the matching instance of the query. 

## Usage
To launch zawgl simply run
```shell
zawgl
```
It will generate its default configuration into .zawgl/Settings.toml file in the current execution directory.

Rust Zawgl client is available on crates.io [zawgl-client](https://crates.io/crates/zawgl-client)

A Deno TypeScript client is also available [ZawglClient](client/zawgl-deno-ts-client/zawgl_client.ts)

Example request:
```rust
use zawgl_client::Client;
use zawgl_client::parameters::{Parameters, Value};

let client = Client::new("ws://localhost:8182").await;
let mut params = Parameters::new();
params.insert("pid".to_string(), Value::Integer(12));
let r = client.execute_cypher_request_with_parameters("match (n:Person) where id(n) = $pid return n", params).await;
```


## Roadmap
* Study VF3 version of sub-graph isomorphism algorithm.
* Keep in mind that graph structures may be timelined in order to be able to retrieve past graph states.

### Short Time Roadmap
* Improve Cypher support
* Implement transactions tests
* Implement client transactions API
* Procedures
* Documentation
* Benchmarks

