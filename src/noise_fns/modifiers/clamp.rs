use crate::noise_fns::NoiseFn;

/// Noise function that clamps the output value from the source function to a
/// range of values.
pub struct Clamp<'a, T, const DIM: usize> {
    /// Outputs a value.
    pub source: &'a dyn NoiseFn<T, DIM>,

    /// Bound of the clamping range. Default is -1.0 to 1.0.
    pub bounds: (f64, f64),
}

impl<'a, T, const DIM: usize> Clamp<'a, T, DIM> {
    pub fn new(source: &'a dyn NoiseFn<T, DIM>) -> Self {
        Self {
            source,
            bounds: (-1.0, 1.0),
        }
    }

    pub fn set_lower_bound(self, lower_bound: f64) -> Self {
        Self {
            bounds: (lower_bound, self.bounds.1),
            ..self
        }
    }

    pub fn set_upper_bound(self, upper_bound: f64) -> Self {
        Self {
            bounds: (self.bounds.0, upper_bound),
            ..self
        }
    }

    pub fn set_bounds(self, lower_bound: f64, upper_bound: f64) -> Self {
        Self {
            bounds: (lower_bound, upper_bound),
            ..self
        }
    }
}

impl<'a, T, const DIM: usize> NoiseFn<T, DIM> for Clamp<'a, T, DIM> {
    fn get(&self, point: [T; DIM]) -> f64 {
        let value = self.source.get(point);

        value.clamp(self.bounds.0, self.bounds.1)
    }
}
