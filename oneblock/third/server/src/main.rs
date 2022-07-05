use std::net::{TcpListener, TcpStream};
use std::io::Read;

/*
This is a race to be the first to answer a question
message:
    msg_type: 1-get current question, 2-answer question
server to client
    msg_type: 1-question, 2-result
 */

pub mod message;
use message::ServerToClientMsg;
use message::ClientToServerMsg;

fn handle_question() {

}

fn handle_answer() {

}

fn main() {
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
        _stream.read(&mut buffer).unwrap();
        
        let msg_type = buffer[0];
        let data = &buffer[1..];

        match msg_type {
            ClientToServerMsg::RespondQuestion => handle_question(),
            ClientToServerMsg::RespondResult => handle_answer(),
            _ => println!("Message type error"),
        }
    }
}
