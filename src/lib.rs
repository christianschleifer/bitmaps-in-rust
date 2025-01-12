#![allow(clippy::new_without_default)]

pub use simple_bitmap::SimpleBitmap;
use std::ops::BitOr;

mod simple_bitmap;

/// Describes the presence or absence of values.
pub trait Bitmap: Sized + BitOr {
    /// Sets the presence of a value at the given index.
    fn set(&mut self, index: u32);

    /// Gets the presence or absence of a value at the given index.
    fn get(&self, index: u32) -> bool;
}
