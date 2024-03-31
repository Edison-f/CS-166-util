#![allow(arithmetic_overflow)]


mod elliptic;
mod rc4;
mod md5;
mod pwcase4;
mod hamming;

fn main() {
    // let res = pwcase4::probability(1f64/4f64, 30, 256);
    // println!("{}", res)
    hamming::hamming_runner();
}
