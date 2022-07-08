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

## Screenshots
- Request question
- 
![图片](https://user-images.githubusercontent.com/83948501/178004796-502a1eea-d8eb-4297-b8ba-e4c9b89726d8.png)

-- Submit question

![图片](https://user-images.githubusercontent.com/83948501/178004996-f6834175-15aa-4f8c-8ea2-db885e4b862e.png)

- Server
- 
![图片](https://user-images.githubusercontent.com/83948501/178005059-3b23b0b0-38e9-4e13-a0a3-1efba7ee966e.png)
