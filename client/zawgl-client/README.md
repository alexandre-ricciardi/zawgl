# zawgl-client
Zawgl graph database rust client

## Usage
Zawgl query language is Cypher.

Sample usage:
```
let client = Client::new("ws://localhost:8182").await;
let mut params = Parameters::new();
params.insert("pid".to_string(), PropertyValue::Integer(12));
let r = client.execute_cypher_request_with_parameters("create (n:Person) where id(n) = $pid return n", params).await;
```