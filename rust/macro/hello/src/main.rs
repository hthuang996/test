use hello_macro::HelloMacro;
use hello_macro_derive::HelloMacro;

#[derive(HelloMacro)]
struct Pancakes;

// impl HelloMacro for Pancakes {
//     fn hello_macro() {
//         println!("Hello, Macro!");
//     }
// }

fn main() {
    // Pancakes::hello_macro();
    let v = vec![1,2,3];
    println!("{:?}", v);
}
