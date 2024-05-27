use mod_exp::mod_exp;
use rayon::prelude::*;

// const P: u128 = u128::MAX - 158; // https://t5k.org/lists/2small/100bit.html 2^128 - 159 is prime, u128::MAX is 2^128 -1 therefore P is prime
const P: u128 = (1u128 << 127) - 1; // https://t5k.org/lists/2small/100bit.html 2^127 - 1 is prime

pub struct Pixel {
    r: u8,
    g: u8,
    b: u8,
}

impl Pixel {
    fn hash_pixel(&self, pixel_idx: usize) -> u128 {
        let r_idx = (pixel_idx * 3) as u128;
        let r_hash = mod_exp(self.r as u128, r_idx, P);
        let g_hash = mod_exp(self.g as u128, r_idx + 1, P);
        let b_hash = mod_exp(self.b as u128, r_idx + 2, P);
        r_hash + g_hash + b_hash
    }
}

pub fn hash(
    pixels: Vec<Vec<Pixel>>,
    full_dim: (usize, usize),
    crop_start: (usize, usize),
    residual_hash: u128,
) -> u128 {
    let crop_hash = pixels
        .par_iter()
        .enumerate()
        .map(|(row_n, row)| {
            row.iter()
                .enumerate()
                .map(|(col_n, pxl)| {
                    let full_row_n = crop_start.0 + row_n;
                    let full_col_n = crop_start.1 + col_n;
                    let idx = full_row_n * full_dim.0 + full_col_n;
                    pxl.hash_pixel(idx)
                })
                .reduce(mod_add)
        })
        .reduce(mod_id, par_mod_add)
        .unwrap();
    (crop_hash + residual_hash) % P
}

fn mod_add(acc: u128, e: u128) -> u128 {
    assert!(acc < P);
    let tmp = acc + e;
    tmp % P
}

fn par_mod_add(acc: Option<u128>, e: Option<u128>) -> Option<u128> {
    let acc = acc.unwrap_or(0);
    let e = e.unwrap_or(0);
    mod_add(acc, e).into()
}

fn mod_id() -> Option<u128> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;
}
