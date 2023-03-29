use std::{fmt::Write, num::ParseIntError};
use std::net::{TcpListener, TcpStream};
use std::io::Read;
use tokio::time;
use num::Num;
use std::collections::HashMap;
use parity_scale_codec::{Encode, Decode};
// use parity_scale_codec_derive::{Encode, Decode};

mod crypto;

const CROSS_CONTRACT_ADDRESS: &str = "X";
const SEND_MESSAGE_SELECTOR: [u8; 4] = [0; 4];

trait A {
    fn a(&self) { println!("a!"); }
}

trait B {
    fn a(&self) { println!("b!"); }
}

#[derive(Default, Debug, Clone)]
pub struct SA {
    value: u8,
    b: SB,
    head: Link,
}

#[derive(Default, Debug, Clone)]
struct SB {
    value: String,
    b: bool,
}

mod C {
    #[derive(Default, Debug, Clone)]
    pub struct SC {
        pub value: String,
        pub c: bool,
    }
}

#[derive(Debug, Clone)]
enum Link {
    Empty,
    More,
}

impl Default for Link {
    fn default() -> Link {
        Self::Empty
    }
}

impl SA {
    fn new() -> Self {
        let b = SB::default();
        Self {
            value: 8,
            b: b.clone(),
            head: Link::default(),
        }
    }

    fn b(&self) { println!("SA!"); }
}

impl A for SA {
    fn a(&self) { println!("surprise! a"); }
}

mod MA {
    pub fn get_struct() {
        let c = super::C::SC::default();
        println!("{:?}", c.value);
        // b
    }
}

mod mul;
use mul::Multi;

fn get_result(a: SA) -> Result<String, u8> {
    let a: Option<String> = None;
    let b = a.ok_or(8)?;
    Ok(b)
}

pub fn decode_hex(s: &str) -> Result<Vec<u8>, ParseIntError> {
    let mut f = 0;
    if &s[0..2] == "0x" {
        f = 2;
    }
    (f..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16))
        .collect()
}

// pub fn encode_hex(bytes: &[u8]) -> String {
//     let mut s = String::with_capacity(bytes.len() * 2);
//     for &b in bytes {
//         write!(&mut s, "{:02x}", b).unwrap();
//     }
//     s
// }

fn convert_address(s: &str) -> [u8; 32] {
    let mut begin = 0;
    if &s[0..2] == "0x" {
        begin = 2;
    }

    let mut v: [u8; 32] = [0; 32];
    let mut index = 0;
    for i in begin/2..s.len()/2 {
        v[index] = u8::from_str_radix(&s[i * 2..i * 2 + 2], 16).unwrap();
        index = index + 1;
    }
    
    v
}

#[derive(Default, Debug, PartialEq, Decode, Encode)]
struct TokenOpcode {
	op: u8,
	data: Vec<u8>
}

#[derive(Debug, PartialEq, Encode, Decode)]
enum EnumType {
	#[codec(index = 15)]
	A,
	B(u32, u64),
	C {
		a: u32,
		b: u64,
	},
}

#[derive(Debug)]
struct TupleS([u8; 16]);

// #[tokio::main]
pub fn main() -> Result<(), ()> {
    // b"node-template::storage::"
    //                 .iter()
    //                 .chain(encoded_bn)
    //                 .copied()
    //                 .collect::<Vec<u8>>()
    // let mut c = 0;
    // let mut a = vec![1,2,3,4];
    // let b = vec![2,3,4];
    // for i in b.iter() {
    //     // println!("{:?}", i);
    //     c = *i;
    // }
    // println!("{:?}", 0_u32 as Num);
    crypto::calc_hash();
    // let a = TupleS([1; 16]);
    // println!("{:?}", a.0);
    // let a = TokenOpcode::default();
    // let mut b = a.encode();
    // let c = TokenOpcode::decode(&mut b.as_slice());
    // println!("{:?}", c);
    Ok(())
}

#[cfg(test)]
mod tests1;