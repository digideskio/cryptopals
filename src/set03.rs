use set01::{xor,aes128_ecb_encrypt};
use set02::CBCOracle;
use utils::{random_bytes,store64};

fn recover_cbc_byte(oracle: &CBCOracle, block: &[u8], iv: &[u8], c: &[u8], i: usize, o: usize, t: bool) -> Vec<Vec<u8>> {
    let mut blocks = vec![];
    let mut x = iv.to_vec();
    let mut y = match t { true => c[0..o].to_vec(), false => c[0..16].to_vec(), };
    for j in 0..256 {
        if t {
            // modify ciphertext
            for k in 0..i {
                y[o - 16 - (k + 1)] = c[o - 16 - (k + 1)] ^ block[16 - (k + 1)] ^ ((i + 1) as u8);
            }
            y[o - 16 - (i + 1)] = c[o - 16 - (i + 1)] ^ (j as u8) ^ ((i + 1) as u8);
        } else {
            // modify IV
            for k in 0..i {
                x[o - 16 - (k + 1)] = iv[o - 16 - (k + 1)] ^ block[16 - (k + 1)] ^ ((i + 1) as u8);
            }
            x[o - 16 - (i + 1)] = iv[o - 16 - (i + 1)] ^ (j as u8) ^ ((i + 1) as u8);
        }
        if None != oracle.decrypt(&x, &y) {
            let mut b = block.to_vec();
            b[16 - i - 1] = j as u8;
            blocks.push(b);
        }
    }
    blocks
}

pub fn recover_cbc_plaintext(oracle: &CBCOracle, iv: &[u8], c: &[u8]) -> Vec<u8> {
    let mut p = vec![];
    // prepare block offset list
    let mut v = (0..(c.len()/16-1)).map(|i| (c.len() - 16*i, true) ).collect::<Vec<(usize,bool)>>();
    v.push((32, false)); // in the last step we have to modify the IV instead of the ciphertext
    for (o,t) in v {
        let mut blocks = vec![];
        for i in 0..16 {
            blocks = match i {
                0 => recover_cbc_byte(&oracle, &vec![0x00;16], &iv, &c, i, o, t),
                _ => {
                    let mut buffer = vec![];
                    for b in blocks {
                        buffer.extend(recover_cbc_byte(&oracle, &b, &iv, &c, i, o, t));
                    }
                    buffer
                }
            }
        }
        // there was not a single case where more than one block was left at this point
        p.insert(0,blocks[0].to_vec());
    }
    p.concat()
}

pub fn aes128_ctr(n: &[u8], k: &[u8], m: &[u8]) -> Vec<u8> {
    let mut i = 0;
    let nonce = n.to_vec();
    let mut c = vec![];
    for x in m.chunks(16) {
        let y = vec![nonce.clone(),store64(i as u64)].concat();
        c.extend(xor(x,&aes128_ecb_encrypt(k,&y)[0..x.len()]));
        i += 1;
    }
    c
}

pub struct CTROracle{ key: Vec<u8> }

impl CTROracle {
    pub fn new() -> CTROracle {
        CTROracle { key: random_bytes(16) }
    }
    pub fn encrypt(&self, n: &[u8], m: &[u8]) -> Vec<u8> {
        aes128_ctr(&n, &self.key, &m)
    }
}

// returns elements of x contained in f
pub fn filter_elements(x: &[u8], f: &[u8]) -> Vec<u8> {
    x.iter().map(|y| y.clone()).filter(|&y| f.contains(&y)).collect::<Vec<u8>>()
}
