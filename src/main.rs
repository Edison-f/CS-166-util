fn main() {
    println!("Hello, world!");
    let k: Vec<u8> = vec![0x01, 0x02, 0x03, 0x04, 0x50, 0x06, 0x07];
    let k2 = vec![0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08];
    encrypt(k, String::from("llllllllllllllllllllllll"));
}
// 0x0102030405060708 works
// for i = 0 to 255
// S[i] = i
// K[i] = key[i (modN)]
// next i
// j = 0
// for i = 0 to 255
// j = (j + S[i] + K[i]) (mod 256)
// Swap(S[i], S[j])
// next i
// i = j = 0

// i = (i + 1) (mod 256)
// j = (j + S[i]) (mod 256)
// swap(S[i], S[j])
// t = (S[i] + S[j]) (mod 256)
// keystreamByte = S[t]

fn encrypt(key: Vec<u8>, plaintext: String) -> Vec<u8> {
    // Init
    let plaintext_bytes = plaintext.as_bytes();
    let mut s = Vec::new();
    let mut k = Vec::new();
    // let key = key.to_be_bytes();
    println!("{}", key.len());
    for i in 0..256 {
        s.push(i as u8);
        k.push(key[i % key.len()]);
    }
    let mut j: u16 = 0;
    for i in 0..255 {
        j = (j + s[i] as u16 + k[i] as u16) % 256;
        s.swap(i, j as usize);
    }
    let mut i = 0;
    let mut j: u16 = 0;
    // for byte in &s.clone() {
    //     print!("{:#04x}, ", byte);
    // }
    // Init done
    let mut cipher_bytes = vec![];
    for chunk in plaintext_bytes {
        i = (i + 1) % 256;
        j = (j + s[i] as u16) % 256; // maybe need % 256
        s.swap(i, j as usize);
        let t = (s[i] as u16 + s[j as usize] as u16) % 256;
        let key_byte = s[t as usize];
        print!("{:#04x}, ", key_byte.clone());
        cipher_bytes.push(key_byte ^ chunk)
    }
    cipher_bytes
}

fn decrypt(key: String, ciphertext: String) -> String {
    return String::from("lol")

}
