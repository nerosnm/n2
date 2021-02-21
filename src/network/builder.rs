//! Tools for building [`Network`]s.

// TODO: Remove this once this module's implementation is finished.
#![allow(unused_imports, dead_code)]

use crate::link::{EndLink, FullLink, Link};

use super::Network;

/// A builder for [`Network`]s.
///
/// This works internally by building up a backwards linked list of layers as
/// each builder method is called, and then reversing it when
/// [`build()`] is called.
///
/// It's not possible to defer creating the layers themselves until the end of
/// the process, because then there would be no way to store the values of the
/// const generic parameters defining the size of each hidden layer (of which
/// there are a variable amount).
///
/// [`build()`]: InputHiddenOutput::build()
pub struct NetworkBuilder;

impl NetworkBuilder {
    /// Create a network builder.
    pub const fn new() -> Self {
        Self
    }

    /// Add an input layer to the network, containing `I` units.
    ///
    /// Every network must have an input layer, so this must be called before
    /// adding any other layers.
    pub fn input_layer<const I: usize>(self) -> Input<I> {
        Input::<I>::new()
    }
}

/// A partially specified network, with the input layer specified but nothing
/// else.
pub struct Input<const IN: usize> {
    link: Box<StartLink<IN>>,
}

impl<const IN: usize> Input<IN> {
    /// Create a new input layer specification, containing a layer with `IN`
    /// units in it.
    fn new() -> Self {
        let layer = BuilderLayer::<IN>;
        let link = Box::new(StartLink { layer });

        Self { link }
    }

    /// Add a hidden layer, containing `H` units, to this network.
    pub fn hidden_layer<const H: usize>(self) -> InputHidden<H, IN> {
        InputHidden::<H, IN>::new(self)
    }

    /// Add an output layer, containing `O` units, to this network, skipping
    /// adding any hidden layers.
    pub fn output_layer<const O: usize>(self) -> InputHiddenOutput<O, IN> {
        InputHidden::<0, IN>::new(self).output_layer::<O>()
    }
}

/// A partially specified network, with the input layer and zero or more hidden
/// layers specified.
pub struct InputHidden<const HEAD: usize, const IN: usize> {
    link: Box<dyn ReverseLink<HEAD, IN>>,
}

impl<const HEAD: usize, const IN: usize> InputHidden<HEAD, IN> {
    /// Create a new hidden layer specification, containing one hidden layer
    /// with `HEAD` units in it.
    fn new(input: Input<IN>) -> InputHidden<HEAD, IN> {
        // The head of the layer list will now be the new hidden layer we're
        // creating:
        let first_hidden = BuilderLayer::<HEAD>;

        // The next (and last) element in the list will be the input layer:
        let input = input.link;

        let link = Box::new(BuilderLink {
            layer: first_hidden,
            link: input,
        });

        Self { link }
    }

    /// Add another hidden layer, containing `H` units, to this network.
    pub fn hidden_layer<const H: usize>(self) -> InputHidden<H, IN> {
        // THe head of the layer list will now be this, the most recent hidden
        // layer, that we're creating right now.
        let next_hidden = BuilderLayer::<H>;

        // The next element in the list will be the last hidden layer that was
        // added before this.
        let last_hidden = self.link;

        let link = Box::new(BuilderLink::<H, HEAD, IN> {
            layer: next_hidden,
            link: last_hidden,
        });

        InputHidden::<H, IN> { link }
    }

    /// Add an output layer, containing `O` units, to this network.
    ///
    /// Each network must have an output layer, so this must be called before
    /// building the network.
    pub fn output_layer<const O: usize>(self) -> InputHiddenOutput<O, IN> {
        InputHiddenOutput::<O, IN>::new(self)
    }
}

/// A fully specified but not built network, with the input layer, zero or more
/// hidden layers and the output layer specified.
pub struct InputHiddenOutput<const O: usize, const I: usize> {
    link: Box<dyn ReverseLink<O, I>>,
}

impl<const OUT: usize, const IN: usize> InputHiddenOutput<OUT, IN> {
    fn new<const PREV: usize>(input_hidden: InputHidden<PREV, IN>) -> InputHiddenOutput<OUT, IN> {
        // The head of the layer list will now be this, last, output layer that
        // we're creating right now.
        let output = BuilderLayer::<OUT>;

        // The next element in the list will be the last layer we added before
        // this (if there are any hidden layers, this is the most recently added
        // one, otherwise this is the input layer).
        let last_hidden = input_hidden.link;

        let link = Box::new(BuilderLink {
            layer: output,
            link: last_hidden,
        });

        InputHiddenOutput::<OUT, IN> { link }
    }

    pub fn build(self) -> Network<IN, OUT> {
        todo!()
    }
}

/// A fake layer, only used while specifying a network.
///
/// Used to record the parameter `N` specifying the number of units that should
/// be in this layer once the network is built.
struct BuilderLayer<const N: usize>;

/// A reverse link, only used while specifying a network.
///
/// These links point backwards through the network; in other words, if you
/// traverse from start to end, you will begin on the output layer and end on
/// the input layer.
trait ReverseLink<const FROM: usize, const TO: usize> {}

/// A link between two layers, only used while specifying a network.
struct BuilderLink<const FROM: usize, const NEXT: usize, const END: usize> {
    layer: BuilderLayer<FROM>,
    link: Box<dyn ReverseLink<NEXT, END>>,
}

impl<const FROM: usize, const NEXT: usize, const END: usize> ReverseLink<FROM, END>
    for BuilderLink<FROM, NEXT, END>
{
}

/// A fake link containing a fake layer, intended to be used to wrap the input
/// layer at the end of a chain of [`ReverseLink`]s when specifying a network.
///
/// This type claims to link two layers both of `N` units in its
/// [`ReverseLink<N, N>`] implementation, although in reality they are the same
/// layer. When attempting to traverse that link, `None` will be returned.
struct StartLink<const N: usize> {
    layer: BuilderLayer<N>,
}

impl<const N: usize> ReverseLink<N, N> for StartLink<N> {}
