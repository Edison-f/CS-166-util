pub(crate) fn add_points(p1: (i32, i32), p2: (i32, i32), a: i32, modulus: i32) -> (i32, i32) {
    // println!("{:?}", p1.1 - p2.1);
    // println!("{:?}", p2);
    let m;
    if p1 != p2 {
        m = special_modulus((p2.1 - p1.1) * modular_inverse(p2.0 - p1.0, modulus), modulus);
    } else {
        m = special_modulus((3 * i32::pow(p1.0, 2) + a) * modular_inverse(2 * p1.1, modulus), modulus);
    }
    let x3 = (i32::pow(m, 2) - p1.0 - p2.0) % modulus;
    let y3 = special_modulus((m * (p1.0 - x3)) - p1.1, modulus);
    println!("m = {}, P3 = ({}, {})", m, special_modulus(x3, modulus), special_modulus(y3, modulus));
    (special_modulus(x3, modulus), special_modulus(y3, modulus))
}


///
///
/// # Arguments
///
/// * `n`:
/// * `modulus`:
///
/// returns: n^-1 (mod modulus)
///
///
pub fn modular_inverse(n: i32, modulus: i32) -> i32 {
    let mut count = 1;
    while ((n * count) % modulus) != 1 {
        count += 1;
    }
    count
}


///
///
/// # Arguments
///
/// * `n`:
/// * `modulus`:
///
/// returns: n % modulus, using modular arithmetic logic (e.g. -2 mod 5 = 3)
///
///
pub fn special_modulus(n: i32, modulus: i32) -> i32 {
    if n >= 0 {
        n % modulus
    } else {
        modulus + (n % modulus)
    }
}