# Rust TCP Server
This is a simple tcp client and tcp server.

## Intruduction
The client can get a question from the server, here is a list of numbers.  
And then the client submits the sum of the numbers as a answer to the server.

## Usage
### Server
```
cargo run
```

### Client
- Request a question
```
cd server
cargo run -- -s
```

- Submit the answer
```
cd client
cargo run -- -a <ANSWER>
```
`<ANSWER>` is a number.