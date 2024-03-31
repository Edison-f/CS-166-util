pub(crate) fn generate_keystream() {
    // let k = vec![0x01, 0x02, 0x03, 0x04, 0x50, 0x06, 0x07];
    // let k2 = vec![0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08];
    // let _k3 = vec![0x01, 0x02, 0x03, 0x04, 0x05];
    // let _k4 = vec![0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a];
    let key = [0x1A, 0x2B, 0x3C, 0x4D, 0x5E, 0x6F, 0x77]; // Key from book
    let mut log = vec![];
    // Init
    let mut s = Vec::new();
    let mut k = Vec::new();
    let mut i = 0;
    while i < 256 {
        s.push(i as u8);
        k.push(key[i % key.len()]);
        i += 1;
    }
    let mut j = 0;
    let mut i = 0;
    while i < 256 {
        j = (j + s[i] as u16 + k[i] as u16) % 256;//k[i] as u16) % 256;
        s.swap(i, j as usize);
        i += 1;
    }
    log.push((format!("Init finish: i = {}, j = {}", i, j), s.clone()));
    // Init done
    let mut i = 0;
    let mut j = 0;

    // Keystream start
    let mut count = 0;
    for _ in 0..=1050 {
        i = (i + 1) % 256;
        j = (j + s[i % 256] as u16) % 256;
        s.swap(i, j as usize);
        // let t = (Wrapping(s[i % 256]) + Wrapping(s[j as usize])).0;
        // let key_byte = s[t as usize]; // Don't need to generate key_byte
        // print!("{:#04x} ", key_byte.clone());
        count += 1;
        if count == 100 || count == 1000 {
            log.push((format!("Count = {}: i = {}, j = {}", count, i, j), s.clone()));
        }
    }
    for s in log {
        println!("{}", s.0);
        for k in 0..16 {
            for l in 0..16 {
                print!("{:#04x} ", s.1[l + k * 16]);
            }
            println!();
        }
    }
}
