use serde::{Deserialize, Serialize};

pub mod client_to_server_msg {
    pub const REQUEST_QUESTION: u8 = 1;
    pub const SUBMIT_ANSWER: u8 = 2;
}

pub mod server_to_client_msg {
    pub const RESPOND_QUESTION: u8 = 1;
    pub const RESPOND_RESULT: u8 = 2;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubmittedAnswer {
    pub answer: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseQuestion {
    pub question: [u32; 4],
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseResult {
    pub result: bool,
}