use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Rust2DartResponse<T>
where
    T: Serialize,
{
    pub data: T,
}

impl<T> Rust2DartResponse<T>
where
    T: Serialize,
{
    pub fn to_json(self) -> String {
        serde_json::to_string(&self).unwrap_or("".to_owned())
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum IpcMessage {
    DartMessage(String),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PinWindowItem {
    pub title: String,
    pub checked: bool,
    pub id: i32,
}
