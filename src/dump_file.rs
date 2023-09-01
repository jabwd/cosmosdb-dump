use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use serde_json::Value;

#[derive(Debug, Deserialize, Serialize)]
pub struct DumpFile {
    pub databases: Vec<Database>
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Database {
    pub name: String,
    pub collections: Vec<Collection>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Collection {
    pub name: String,
    pub documents: Vec<HashMap<String, Value>>,
}
