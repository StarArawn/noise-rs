use crate::noise_fns::NoiseFn;

/// Noise function that outputs the sum of the two output values from two source
/// functions.
pub struct Add<'a, T> {
    /// Outputs a value.
    pub source1: &'a dyn NoiseFn<T>,

    /// Outputs a value.
    pub source2: &'a dyn NoiseFn<T>,
}

impl<'a, T> Add<'a, T> {
    pub fn new(source1: &'a dyn NoiseFn<T>, source2: &'a dyn NoiseFn<T>) -> Self {
        Self { source1, source2 }
    }
}

impl<'a, T> NoiseFn<T> for Add<'a, T>
where
    T: Copy + Send + Sync,
{
    fn get(&self, point: T) -> f64 {
        self.source1.get(point) + self.source2.get(point)
    }
}
