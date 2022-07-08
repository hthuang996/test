use std::net::{TcpListener, TcpStream};
use std::io::{
    Read,
    Write
};
// use tokio::time;
use std::vec::Vec;

/*
This is a race to be the first to answer a question
message:
    msg_type: 1-get current question, 2-answer question
server to client
    msg_type: 1-question, 2-result
 */

pub mod message;
use message::server_to_client_msg;
use message::client_to_server_msg;
use message::{
    ResponseQuestion,
    ResponseResult,
    SubmittedAnswer,
};

// const UPDATE_INTERVAL: u64 = 1;

struct Server {
    question: [u32; 4],
}

impl Server {
    pub fn new() -> Self {
        Self {
            question: [2,49,22,5]
        }
    }

    // pub async fn question_update(&mut self) {
    //     println!("question update");
    // }

    pub fn handle_question(&mut self, stream: &mut TcpStream) {
        println!("handle_question");
        let msg_type = server_to_client_msg::RESPOND_QUESTION.to_be_bytes();
        let question = ResponseQuestion{question: self.question};
        let question_str = serde_json::to_string(&question).unwrap();
        let mut v = Vec::<u8>::new();
        v.push(msg_type[0]);
        for i in question_str.as_bytes() {
            v.push(*i);
        }
        stream.write(&v.as_slice()).unwrap();
    }
    
    pub fn handle_answer(&mut self, stream: &mut TcpStream, answer: &SubmittedAnswer) {
        println!("handle_answer, {:?}", answer);
        let msg_type = server_to_client_msg::RESPOND_RESULT.to_be_bytes();
        let mut result = ResponseResult{result: false};
        if answer.answer == 78 {
            result.result = true;
        }
        let result_str = serde_json::to_string(&result).unwrap();
        let mut v = Vec::<u8>::new();
        v.push(msg_type[0]);
        for i in result_str.as_bytes() {
            v.push(*i);
        }
        stream.write(&v.as_slice()).unwrap();
    }
}

// async fn time_interval(s: &'static Server) {
//     let mut interval = time::interval(time::Duration::from_secs(UPDATE_INTERVAL));
//     loop {
//         interval.tick().await;
//         s.question_update();
//     }
// }

#[allow(dead_code)]
#[tokio::main]
async fn main() {
    let mut s: Server = Server::new();

    let listener = TcpListener::bind("127.0.0.1:3000").unwrap();
    println!("Running on port 3000...");

    for stream in listener.incoming() {
        println!("Connection established");

        if stream.is_err() {
            println!("Stream error");
            continue;
        }

        let mut buffer = [0; 1024];
        let mut _stream = stream.unwrap();
        let len = _stream.read(&mut buffer).unwrap();

        if len == 0 {
            continue;
        }
        
        let msg_type = buffer[0];
        let data = &buffer[1..len];

        match msg_type {
            client_to_server_msg::REQUEST_QUESTION => s.handle_question(&mut _stream),
            client_to_server_msg::SUBMIT_ANSWER => {
                let answer: SubmittedAnswer = serde_json::from_slice(data).unwrap();
                s.handle_answer(&mut _stream, &answer)
            },
            _ => println!("Message type error"),
        }
    }
}
