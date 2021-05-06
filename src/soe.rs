use crate::vector::Vector;
use std::marker::PhantomData;

/// Trait for system of equations
/// something that we can call and get output
pub trait Soe {
    type Args;

    fn call(&mut self, args: &Self::Args) -> Self::Args;
}

/// System of two equations for main problem
///
/// `F1` and `F2` - two functions
///
/// T - inherit type of vectors
///
/// N1 - length of functions arguments
///
/// N2 - length of functions outputs
///
/// Should be created from Soe2Builder
pub struct Soe2<F1, F2, T, const N1: usize, const N2: usize> {
    f1: F1,
    f2: F2,
    marker: PhantomData<(Vector<T, N1>, Vector<T, N2>)>,
}

/// Builder for Soe2
pub struct Soe2Builder<T, const ArgLen: usize, const OutLen: usize> {
    marker: PhantomData<(Vector<T, ArgLen>, Vector<T, OutLen>)>,
}

impl<T, const N1: usize, const N2: usize> Soe2Builder<T, N1, N2> {
    pub fn new() -> Self {
        Self {
            marker: PhantomData,
        }
    }
    pub fn build<F1, F2>(self, f1: F1, f2: F2) -> Soe2<F1, F2, T, N1, N2> {
        Soe2 {
            f1,
            f2,
            marker: PhantomData,
        }
    }
}

/// One equation (just for example)
pub struct SimpleSoe<F1, T, const N1: usize, const N2: usize> {
    f1: F1,
    marker: PhantomData<(Vector<T, N1>, Vector<T, N2>)>,
}

pub struct SimpleSoeBuilder<T, const ArgLen: usize, const OutLen: usize> {
    marker: PhantomData<(Vector<T, ArgLen>, Vector<T, OutLen>)>,
}

impl<T, const N1: usize, const N2: usize> SimpleSoeBuilder<T, N1, N2> {
    pub fn new() -> Self {
        Self {
            marker: PhantomData,
        }
    }
    pub fn build<F1>(self, f1: F1) -> SimpleSoe<F1, T, N1, N2> {
        SimpleSoe {
            f1,
            marker: PhantomData,
        }
    }
}

impl<F1, F2, T, const ArgLen: usize, const OutLen: usize> Soe for Soe2<F1, F2, T, ArgLen, OutLen>
where
    F1: FnMut(&Vector<T, ArgLen>) -> Vector<T, OutLen>,
    F2: FnMut(&Vector<T, ArgLen>) -> Vector<T, OutLen>,
    T: Default + Copy,
{
    type Args = Vector<T, ArgLen>;

    fn call(&mut self, args: &Self::Args) -> Self::Args {
        let f1result = (self.f1)(args);
        let f2result = (self.f2)(args);

        Self::Args::construct_from_two(&f1result, &f2result)
    }
}

impl<F1, T, const ArgLen: usize, const OutLen: usize> Soe for SimpleSoe<F1, T, ArgLen, OutLen>
where
    F1: FnMut(&Vector<T, ArgLen>) -> Vector<T, OutLen>,
    T: Default + Copy,
{
    type Args = Vector<T, ArgLen>;

    fn call(&mut self, args: &Self::Args) -> Self::Args {
        let f1result = (self.f1)(args);

        Self::Args::construct_from_two(&f1result, &Vector::<T, 0>::new())
    }
}
