use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct GetResponse<T> {
    pub found: bool,
    pub _id: String,
    pub _source: Option<T>,
}
