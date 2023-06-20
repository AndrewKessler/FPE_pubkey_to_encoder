extern crate aes_gcm;

use aes_gcm::{
aead::{Aead, AeadCore, KeyInit, OsRng},
Aes256Gcm, Nonce, Key // Or `Aes128Gcm`
};

#[derive(Debug)]
struct Group {

    element: u8,
    weight: String

}

fn main() {

    let mut list: Vec<Group> = Vec::new();
    let key = Aes256Gcm::generate_key(OsRng);
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng); // 96-bits; unique per message
    let cipher = Aes256Gcm::new(&key);
    //calculate a weight using AES for randomizing a position in a list f
    //you could use big num and sort list numerically as cannonical but sorting a Vec of strings seems lazy but just as good?

    for i in 0..255 {
        let mut ciphertext = cipher.encrypt(&nonce, i.to_string().as_ref()).unwrap();
        let mut a_string = String::from("");
        //weirdness abounds
        for j in 1..(ciphertext.len()-1) {

            a_string.push(ciphertext[j] as char);
        }
        let mut entry = Group { element: i , weight: a_string };
        list.push(entry);

        }

        //println!("key: {:#?}", key);
    list.sort_by_key(|d| d.element);
    list.sort_by(|d1, d2| d1.weight.cmp(&d2.weight));

    // The encryption key can be generated randomly but in reality must come from an EC25519 pub key.

         // println!("list: {:#?}", list);
    let mut permutation: Vec<u8> = Vec::new();
    for k in 0..255 {
        permutation.push(list[k].element);
    }
    println!("list: {:#?}", permutation);
}
