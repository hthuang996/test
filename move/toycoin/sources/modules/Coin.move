address 0x2 {
    module Coin {
        struct Coin has drop {
            value: u64,
        }

        public fun mint(value: u64): Coin {
            Coin { value }
        }
    }
}