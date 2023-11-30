use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct LevelSchema {
    pub name: String,
    pub version: u32,
}