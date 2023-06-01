# zawgl-client
Zawgl graph database rust client

## Usage
Zawgl query language is Cypher.

Sample usage:
```rust
let client = Client::new("ws://localhost:8182").await;
let mut params = Parameters::new();
params.insert("pid".to_string(), Value::Integer(12));
let r = client.execute_cypher_request_with_parameters("match (n:Person) where id(n) = $pid return n", params).await;
```
The response is a Bson document, see example below:
```json
{
  "request_id": "969f462c-ec71-41ab-bed8-0b46314f5965",
  "result": {
    "graphs": [
      {
        "nodes": [
          {
            "name": "x",
            "id": 113,
            "properties": [],
            "labels": [
              "Person"
            ]
          }
        ],
        "relationships": [
          {
            "id": 78,
            "source_id": 113,
            "target_id": 113,
            "properties": [],
            "labels": [
              "FRIEND_OF"
            ],
            "name": "f"
          }
        ]
      }
    ]
  }
}
```