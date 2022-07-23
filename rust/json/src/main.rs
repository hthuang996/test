use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    match args[1].as_str() {
        "-s" => println!("s"),
        "-a" => println!("a"),
        _ => println!("n"),
    }
    println!("{:?}", args);
}
