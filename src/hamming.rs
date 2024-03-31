pub(crate) fn hamming_runner() {
    let _alice: u64 =   0xBE439AD598EF5147;
    let _bob: u64 =          0x9C8B7A1425369584;
    let _charlie: u64 =      0x885522336699CCBB;
    let _u: u64 =            0xC975A2132E89CEAF;
    let _v: u64 =            0xDB9A8675342FEC15;
    let _w: u64 =            0xA6039AD5F8CFD965;
    let _x: u64 =            0x1DCA7A54273497CC;
    let _y: u64 =            0xAF8B6C7D5E3F0F9A;
    let list = vec![_u, _v, _w, _x, _y];
    // hamming_dist(_bob, _charlie);
    hamming_dist_vec(_alice, list);
}

fn hamming_dist(var1: u64, var2: u64){
    let and = var1 & var2;
    let mut test: u64 = 1;
    let mut result = 0;
    for _i in 0..64 {
        if (and & test) == test {
            result += 1;
        }
        test <<= 1;
    }
    println!("{}", result)
}

fn hamming_dist_vec(base: u64, compare: Vec<u64>) {
    for val in compare {
        hamming_dist(base, val)
    }
}