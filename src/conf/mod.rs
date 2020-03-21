use toml;
use serde;

#[derive(Deserialize)]
pub struct Conf {
    db_dir: String,
    node_store: String,
    relationship_store: String,
    properties_store: String,
}