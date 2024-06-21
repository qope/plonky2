#[cfg(not(feature = "std"))]
use alloc::boxed::Box;
use core::fmt::Debug;

use plonky2_field::extension::Extendable;

use crate::hash::hash_types::RichField;
use crate::plonk::circuit_builder::CircuitBuilder;

/// `BuilderHook` can impose the constraints specified by `constrain`
/// at the beginning of the circuit build.
pub trait BuilderHook<F: RichField + Extendable<D>, const D: usize>:
    'static + Send + Sync + Debug
{
    fn constrain(&self, builder: &mut CircuitBuilder<F, D>);
}

#[derive(Debug)]
pub struct BuilderHookRef<F: RichField + Extendable<D>, const D: usize>(
    pub Box<dyn BuilderHook<F, D>>,
);

impl<F: RichField + Extendable<D>, const D: usize> BuilderHookRef<F, D> {
    pub fn new<H: BuilderHook<F, D>>(hook: H) -> BuilderHookRef<F, D> {
        BuilderHookRef(Box::new(hook))
    }
}
