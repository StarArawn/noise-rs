//! A procedural noise generation library for Rust.
//!
//! # Example
//!
//! ```rust
//! //use noise::{NoiseFn, Perlin};
//!
//! //let perlin = Perlin::new();
//! //let val = perlin.get([42.4, 37.7, 2.8]);
//! ```

#![deny(missing_copy_implementations)]
#![allow(dead_code)]

pub use crate::noise_fns::*;

mod gradient;
mod math;
mod noise_fns;
pub mod noisefield;
mod permutationtable;
pub mod utils;
