use crate::SA;

pub trait Multi {
    fn mul(& self, a: u128, b: u128) -> u128;
}

impl Multi for SA {
    fn mul(& self, a: u128, b: u128) -> u128 {
        a * b
    }
}

pub fn mul(a: u128, b: u128) -> u128 {
    a * b
}