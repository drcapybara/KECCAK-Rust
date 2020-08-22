use std::ops::BitXor;



fn main() {


    /*Initialize constants */

    let keccakf_rndc: [u64; 24] = 
        [0x0000000000000001, 0x0000000000008082, 0x800000000000808a,
        0x8000000080008000, 0x000000000000808b, 0x0000000080000001,
        0x8000000080008081, 0x8000000000008009, 0x000000000000008a,
        0x0000000000000088, 0x0000000080008009, 0x000000008000000a,
        0x000000008000808b, 0x800000000000008b, 0x8000000000008089,
        0x8000000000008003, 0x8000000000008002, 0x8000000000000080,
        0x000000000000800a, 0x800000008000000a, 0x8000000080008081,
        0x8000000000008080, 0x0000000080000001, 0x8000000080008008];

    

    let keccakf_rotc: [u32;24] = 
        [1,  3,  6,  10, 15, 21, 28, 36, 45, 55, 2,  14,
        27, 41, 56, 8,  25, 43, 62, 18, 39, 61, 20, 44];

    let keccakf_piln: [u32; 24] = 
        [10, 7,  11, 17, 18, 3, 5,  16, 8,  21, 24, 4,
        15, 23, 19, 13, 12, 2, 20, 14, 22, 9,  6,  1];


    let i: u32;
    let j: u32;
    let r: u32;

    type Output = <u64 as BitXor<u64>>::Output;

    let mut t: u64; 

    let mut st: [u64; 25] = [0; 25];
    let mut bc: [u64; 5] = [0; 5];

    /* Theta Step */
    for i in 0..5 {
        bc[i] = bitxor64(bitxor64(bitxor64(bitxor64(st[i], st[i + 5]), st[i + 10]), st[i + 15]), st[i + 20]);
    }




    for i in 0..5 {
        t = bitxor64(bc[(i + 4) % 5], ROTL64(bc[(i + 1) % 5], 1));
    }

}

///u64 XOR function, https://doc.rust-lang.org/std/primitive.u64.html
fn bitxor64(this: u64, other: u64) -> <u64 as BitXor<u64>>::Output {
    return this ^ other;
}

fn ROTL64(x: u64, y: u64) -> u64 {
   return ((x) << (y)) | ((x) >> (64 - (y)));
}
