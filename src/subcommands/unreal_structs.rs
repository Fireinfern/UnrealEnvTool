use cli_table::Table;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Table)]
pub struct UnrealVersion{
    pub version: String,
    pub path: String
}

#[derive(Serialize, Deserialize, Clone)]
pub struct CollectionOfVersions {
    pub versions: Vec<UnrealVersion>
}