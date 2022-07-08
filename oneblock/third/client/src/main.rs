use std::env;
use std::net::TcpStream;
use std::io::{
    Read,
    Write,
};

use server::message::{
    ResponseQuestion,
    ResponseResult,
    SubmittedAnswer,
    server_to_client_msg,
    client_to_server_msg,
};

fn request_question(_stream: &mut TcpStream) {
    let msg_type = client_to_server_msg::REQUEST_QUESTION.to_be_bytes();
    _stream.write(&msg_type).unwrap();
}

fn submit_answer(_stream: &mut TcpStream, n: u32) {
    let msg_type = client_to_server_msg::SUBMIT_ANSWER.to_be_bytes();
    let answer = SubmittedAnswer{answer: n};
    let answer_str = serde_json::to_string(&answer).unwrap();
    let mut v = Vec::<u8>::new();
    v.push(msg_type[0]);
    for i in answer_str.as_bytes() {
        v.push(*i);
    }
    _stream.write(&v.as_slice()).unwrap();
}

#[allow(dead_code)]
fn main() {
    let args: Vec<String> = env::args().collect();
    let mut _stream = TcpStream::connect("localhost:3000").unwrap();

    match args[1].as_str() {
        "-s" => request_question(&mut _stream),
        "-a" => {
            let n: u32 = args[2].parse::<u32>().unwrap();
            submit_answer(&mut _stream, n)
        },
        _ => panic!("Wrong parameter"),
    }

    let mut buffer = [0; 1024];
    let len = _stream.read(&mut buffer).unwrap();
    
    let msg_type = buffer[0];
    let data = &buffer[1..len];
    match msg_type {
        server_to_client_msg::RESPOND_QUESTION => {
            let question: ResponseQuestion = serde_json::from_slice(data).unwrap();
            println!("The question is{:?}", question);
        },
        server_to_client_msg::RESPOND_RESULT => {
            let question: ResponseResult = serde_json::from_slice(data).unwrap();
            println!("The result is{:?}", question);
        },
        _ => println!("Message type error"),
    }
}
