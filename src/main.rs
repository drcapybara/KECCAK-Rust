use std::ops::BitXor;

/**
    A port of the C version of tinySha3 by Markku-Juhani O. Saarinen found at 
    https://github.com/mjosaarinen/tiny_sha3/blob/master/sha3.c
    
    Dustin Ray - Summer 2020
 */

fn main() {

    let KECCAKF_ROUNDS: usize;
    KECCAKF_ROUNDS = 24;

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

    
    let keccakf_rotc: [u64;24] = 
        [1,  3,  6,  10, 15, 21, 28, 36, 45, 55, 2,  14,
        27, 41, 56, 8,  25, 43, 62, 18, 39, 61, 20, 44];

    //set to type usize since these will be used for iterating
    let keccakf_piln: [usize; 24] = 
        [10, 7,  11, 17, 18, 3, 5,  16, 8,  21, 24, 4,
        15, 23, 19, 13, 12, 2, 20, 14, 22, 9,  6,  1];

    //variables for iteration; set to usize
    let mut i: usize;
    let mut j: usize;
    let mut r: usize;

    let mut t: u64; 
    //state array of 25, 64 bit words
    let mut st: [u64; 25] = [0; 25];
    
    let mut bc: [u64; 5] = [0; 5];

    r = 0;
    
    //main loop will run exactly ROUNDS number of times, := 24 in this example. 
    for r in 0..KECCAKF_ROUNDS {

        /* Theta Step */
        for i in 0..5 {
            //bc[i] = st[i] ^ st[i + 5] ^ st[i + 10] ^ st[i + 15] ^ st[i + 20];
            bc[i] = bitxor64(bitxor64(bitxor64(bitxor64(st[i], st[i + 5]), st[i + 10]), st[i + 15]), st[i + 20]);
        }
        for i in 0..5 {
            //t = bc[(i + 4) % 5] ^ ROTL64(bc[(i + 1) % 5], 1);   
            t = bitxor64(bc[(i + 4) % 5], ROTL64(bc[(i + 1) % 5], 1));
            
            j = 0;
            while j < 25 {
            st[j + i] ^= t;
            j = j + 5;
            }
        }


        /* Rho Pi step */
        t = st[1];
        i = 0;
        for i in 0..24 {
            j = keccakf_piln[i];
            bc[0] = st[j];
            st[j] = ROTL64(t, keccakf_rotc[i]);
            t = bc[0];
        }


        /* Chi Step */
        j = 0;
        
        while j < 25 {
            i = 0;
            for i in 0..5 {
                bc[i] = st[j + i];        
            } i = 0;
            for i in 0..5 {
                // "!" in rust is the same as "~" in C
                st[j + i] ^= (!bc[(i + 1) % 5]) & bc[(i + 2) % 5];
            }
            j = j + 5;
        }

        /* Iota Step */
        st[0] ^= keccakf_rndc[r];

    }

}


/**
    u64 XOR function, https://doc.rust-lang.org/std/primitive.u64.html
    param this is 64 bit unsigned word
    param other is 64 bit unsigned word
    return value is 64 bit unsigned result of this XORed with other
*/
fn bitxor64(this: u64, other: u64) -> <u64 as BitXor<u64>>::Output {
    return this ^ other;
}

/** 
    ROTL64 function from KECCAK specifications. 
*/
fn ROTL64(x: u64, y: u64) -> u64 {
   return ((x) << (y)) | ((x) >> (64 - (y)));
}
