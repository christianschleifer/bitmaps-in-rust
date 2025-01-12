use crate::Bitmap;
use std::fmt::{Debug, Formatter};
use std::ops::BitOr;
use std::{cmp, iter};

/// Non-optimized implementation of a [Bitmap].
#[derive(Clone)]
pub struct SimpleBitmap {
    bits: Vec<u32>,
}

impl SimpleBitmap {
    pub fn new() -> Self {
        Self { bits: Vec::new() }
    }
}

impl Bitmap for SimpleBitmap {
    fn set(&mut self, index: u32) {
        let u32_index_in_bits_vec = (index / 32) as usize;
        let bit_index_in_u32 = index & 0b11111;

        // if there is too little u32s in the bits vec, it has to be extended
        if u32_index_in_bits_vec >= self.bits.len() {
            self.bits.resize(u32_index_in_bits_vec + 1, 0);
        }

        let stored_u32 = self.bits[u32_index_in_bits_vec];

        let modified_u32 = stored_u32 | (0b1 << bit_index_in_u32);

        self.bits[u32_index_in_bits_vec] = modified_u32;
    }

    fn get(&self, index: u32) -> bool {
        let u32_index_in_bits_vec = (index / 32) as usize;

        if let Some(bucket) = self.bits.get(u32_index_in_bits_vec) {
            let bit_index_in_u32 = index & 0b11111;

            ((bucket >> bit_index_in_u32) & 0b1) == 1
        } else {
            false
        }
    }
}

impl BitOr for SimpleBitmap {
    type Output = SimpleBitmap;

    fn bitor(self, rhs: Self) -> Self::Output {
        // allocate enough capacity for the larger of both vecs
        let mut union = Vec::with_capacity(cmp::max(self.bits.len(), rhs.bits.len()));

        let mut left_iter = self.bits.iter();
        let mut right_iter = rhs.bits.iter();

        // iterate over both iterators and perform the bitwise or operation as long as both iters yield u32s and
        // add the result to the union vector
        for (left, right) in iter::zip(&mut left_iter, &mut right_iter) {
            union.push(left | right);
        }

        // if there is u32s remaining in left, add the u32s to the union vector
        for left in left_iter {
            union.push(*left);
        }

        // if there is u32s remaining in right, add the u32s to the union vector
        for right in right_iter {
            union.push(*right);
        }

        SimpleBitmap { bits: union }
    }
}

impl Debug for SimpleBitmap {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for num in &self.bits {
            writeln!(f, "{:032b}", num)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn get_bits_vector_index() {
        assert_eq!(31 / 32, 0);
        assert_eq!(32 / 32, 1);
    }

    #[test]
    #[allow(clippy::identity_op)]
    fn get_bit_index_in_u32_using_modulo() {
        assert_eq!(31 % 32, 31);
        assert_eq!(32 % 32, 0);
    }

    #[test]
    fn get_bit_index_in_u32_using_bitwise_and() {
        assert_eq!(31 & 31, 31);
        assert_eq!(32 & 31, 0);
    }

    #[test]
    fn it_sets_and_gets_bits() {
        // given
        let mut bm = SimpleBitmap::new();

        // when
        bm.set(31);
        bm.set(32);
        println!("{:?}", bm);

        // then
        assert!(!bm.get(0));
        assert!(bm.get(31));
        assert!(bm.get(32));
    }

    #[test]
    fn it_builds_bit_unions() {
        // given
        // Speyside    --> [0, 1, 0, 0, 0, 0, 1, 0, 0, 0]
        let mut speyside_bm = SimpleBitmap::new();
        speyside_bm.set(1);
        speyside_bm.set(6);
        println!("{:?}", speyside_bm);

        // Highlands   --> [0, 0, 1, 1, 0, 0, 0, 0, 0, 1]
        let mut highlands_bm = SimpleBitmap::new();
        highlands_bm.set(2);
        highlands_bm.set(3);
        highlands_bm.set(9);
        println!("{:?}", highlands_bm);

        // when
        let speyside_or_highlands = speyside_bm | highlands_bm;
        println!("{:?}", speyside_or_highlands);

        // then
        // Union       --> [0, 1, 1, 1, 0, 0, 1, 0, 0, 1]
        assert!(!speyside_or_highlands.get(0));
        assert!(speyside_or_highlands.get(1));
        assert!(speyside_or_highlands.get(2));
        assert!(speyside_or_highlands.get(3));
        assert!(!speyside_or_highlands.get(4));
        assert!(!speyside_or_highlands.get(5));
        assert!(speyside_or_highlands.get(6));
        assert!(!speyside_or_highlands.get(7));
        assert!(!speyside_or_highlands.get(8));
        assert!(speyside_or_highlands.get(9));
        assert!(!speyside_or_highlands.get(10));
    }
}
