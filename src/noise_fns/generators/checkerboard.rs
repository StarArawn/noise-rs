use crate::{core::checkerboard::checkerboard, noise_fns::NoiseFn};

/// Noise function that outputs a checkerboard pattern.
///
/// This noise function can take one input, size, and outputs 2<sup>size</sup>-sized
/// blocks of alternating values. The values of these blocks alternate between
/// -1.0 and 1.0.
///
/// This noise function is not very useful by itself, but it can be used for
/// debugging purposes.
#[derive(Clone, Copy, Debug)]
pub struct Checkerboard {
    /// Controls the size of the block in 2^(size).
    size: u32,
}

impl Checkerboard {
    /// Controls the size of the block in 2^(size) units.
    pub fn new(size: u32) -> Self {
        Self { size }
    }

    pub fn set_size(self, size: u32) -> Self {
        Self { size }
    }

    pub fn size(self) -> u32 {
        self.size
    }
}

impl Default for Checkerboard {
    fn default() -> Self {
        Self { size: 1 }
    }
}

impl<const DIM: usize> NoiseFn<f64, DIM> for Checkerboard {
    fn get(&self, point: [f64; DIM]) -> f64 {
        checkerboard(point, self.size as u32)
    }
}
