//! Links between layers in a network.

use std::fmt::Debug;

use crate::layer::Layer;

/// A link between two layers in a network, where the first layer contains `F`
/// units ("from"), and the second contains `T` units ("to").
pub trait Link<const F: usize, const T: usize>: Debug {}

/// A link between two layers where each unit in the second layer is connected
/// to all of the units in the first layer.
#[derive(Debug)]
pub struct FullLink<const F: usize, const N: usize, const T: usize> {
    layer: Layer<F>,
    next: Box<dyn Link<N, T>>,
}

impl<const F: usize, const N: usize, const T: usize> Link<F, T> for FullLink<F, N, T> {}

/// A fake link intended to be used to wrap the output layer in a network.
///
/// This type claims to link two layers both of `N` units in its [`Link<N, N>`]
/// implementation, although in reality they are the same layer. When attempting
/// to traverse that link, `None` will be returned.
#[derive(Debug)]
pub struct EndLink<const N: usize> {
    layer: Layer<N>,
}

impl<const N: usize> EndLink<N> {
    pub fn with_layer(layer: Layer<N>) -> Self {
        Self { layer }
    }
}

impl<const N: usize> Link<N, N> for EndLink<N> {}
