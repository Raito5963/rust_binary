// not
pub fn not(x: u8)->(u8){
    !x & 1
}

// and
pub fn and(x: u8, y: u8)->(u8){
    x & y
}

// or
pub fn or(x: u8, y: u8)->(u8){
    x | y
}

// xor
pub fn xor(x: u8, y: u8)->(u8){
    x ^ y
}

// half adder
pub fn half_adder(x: u8, y: u8)->(u8, u8){
    let c:u8 = and(x, y);
    let s:u8 = xor(x, y);
    (c, s)
}

// full adder
pub fn full_adder(x: u8, y: u8, cin: u8)->(u8, u8){
    let (c1, s1) = half_adder(x, y);
    let (c2, s2) = half_adder(s1, cin);
    let cout = c1 | c2;
    (cout, s2)
}

pub fn left_shift(x: u32, i: u32)->(u32){
    x << i
}

pub fn right_shift(x: u32, i: u32)->(u32){
    x >> i
}

pub fn adder(x: u32, y: u32)->(u32){
    let mut carry: u8 = 0;
    let mut res: u32 = 0;

    for i in 0..32{
        let bit_x = (right_shift(x, i) & 1) as u8;
        let bit_y = (right_shift(y, i) & 1) as u8;

        let (cout, sum) = full_adder(bit_x, bit_y, carry);

        res = or(res, left_shift(sum as u32, i as u32);

        carry = cout;
    }
    res
}