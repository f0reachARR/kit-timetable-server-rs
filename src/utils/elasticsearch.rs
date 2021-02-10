use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct GetResponse<T> {
    pub found: bool,
    pub _id: String,
    pub _source: T,
}