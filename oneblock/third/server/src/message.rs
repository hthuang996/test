use serde::{Deserialize, Serialize};
use serde_json::Result;

pub mod ServerToClientMsg {
    pub const RequestQuestion: u8 = 1;
    pub const SubmitAnswer: u8 = 2;
}

pub mod ClientToServerMsg {
    pub const RespondQuestion: u8 = 1;
    pub const RespondResult: u8 = 2;
}

#[derive(Serialize, Deserialize)]
pub struct SubmittedAnswer {
    answer: u32,
}

#[derive(Serialize, Deserialize)]
pub struct ResponseQuestion {
    question: [u32; 4],
}

#[derive(Serialize, Deserialize)]
pub struct ResponseResult {
    result: bool,
}