use std::vec::Vec;

fn sum(v: &Vec<u8>) -> Option<u8> {
    let mut ret = 0_u8;
    for i in v {
        let m = ret.checked_add(*i);
        if m.is_none() {
            return None;
        }
        ret = m.unwrap();
    }
    return Some(ret);
}

fn main() {
    let mut v = vec![222,1,22];
    let mut ret = sum(&v);
    println!("Result is {:?}", ret);
    v = vec![222,1,22,44];
    ret = sum(&v);
    println!("Result is {:?}", ret);
}
