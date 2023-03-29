use sha3::{Digest, Keccak256};
use secp256k1::{Secp256k1, Message, ecdsa, PublicKey};
// use secp256k1::rand::rngs::OsRng;
// use as_slice::AsSlice;

pub fn calc_hash() {
//     let mut hasher = Keccak256::new();
//     hasher.update(b"asdf");

//     let result = hasher.finalize();

//     println!("{:?}", hex::encode(result.to_vec()));
//     // println!("{:?}", decode("1234"));

    let h = hex::decode("041c0ae2fe60e7b9e91b3690626318c8759147c6daf96147d886d37b4df8dd8829db901b1a4bbb9374b35322660503495597332b3944e49985fa2e827797634799");
    let public_key = PublicKey::from_slice(h.unwrap().as_ref()).expect("public keys must be 33 or 65 bytes, serialized according to SEC 2");
println!("{:?}", public_key.serialize());
//     let secp = Secp256k1::new();
//     let message = Message::from_slice(hex::decode("4c8f18581c0167eb90a761b4a304e009b924f03b619a0c0e8ea3adfce20aee64").unwrap().as_ref())
//     .expect("messages must be 32 bytes and are expected to be hashes");

//     let sig = ecdsa::Signature::from_compact(&[
//         168,  65, 123, 229, 161, 228,  17, 181,  75,  75, 184,
//          40, 110,  21,  95,  22, 213, 249, 247,  56, 228,  90,
//          30, 171,  45, 245, 101,  78, 225, 211, 214, 179, 113,
//          87, 216, 140,  73, 131, 243, 205,  66,  89, 224, 221,
//          75, 190, 161,  61,  14, 159, 194,  59, 124,   2, 183,
//         126,  85, 127,  20, 116,   6,  46, 116,  66
//       ]).expect("compact signatures are 64 bytes; DER signatures are 68-72 bytes");
//       println!("{:?}", message.as_ref());
//       println!("{:?}", sig);
//     assert!(secp.verify_ecdsa(&message, &sig, &public_key).is_ok());

    // let secp = Secp256k1::new();
    // let (secret_key, public_key) = secp.generate_keypair(&mut OsRng);
    // println!("{:?}", public_key);
    // let (secret_key2, public_key2) = secp.generate_keypair(&mut OsRng);
    // println!("{:?}", public_key2);

}