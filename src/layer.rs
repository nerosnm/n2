//! Layers of units in a network.

use std::default::Default;

use crate::unit::Unit;

/// A layer in a neural network, containing a positive, nonzero number of units
/// `N`.
#[derive(Debug)]
pub struct Layer<const N: usize> {
    units: [Unit; N],
}

impl<const N: usize> Layer<N> {
    /// Create a layer containing `units` units.
    pub fn new() -> Self {
        assert!(N > 0, "each layer must contain at least 1 unit");

        Self { units: [Unit; N] }
    }
}

impl<const N: usize> Default for Layer<N> {
    fn default() -> Self {
        Self::new()
    }
}
