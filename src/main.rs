#![allow(arithmetic_overflow)]

use crate::md5::md5_runner;

mod elliptic;
mod rc4;
mod md5;

fn main() {
    md5_runner();
}