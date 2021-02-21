//! Specification of the structure of neural networks.

pub mod builder;

pub use builder::NetworkBuilder;

use crate::link::Link;

/// A network, containing at least two layers, where the input layer has `I`
/// units and the output layer has `O` units.
#[derive(Debug)]
pub struct Network<const I: usize, const O: usize> {
    layers: Box<dyn Link<I, O>>,
}
