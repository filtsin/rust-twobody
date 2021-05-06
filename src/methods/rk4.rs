use crate::soe::Soe;
use ndarray::{
    iter::IndexedIterMut, ArrayView1, Dimension, IntoDimension, IntoNdProducer, Ix1, NdProducer,
    Zip,
};
use std::{iter::Iterator, ops::Add};
use std::{
    fmt::Debug,
    ops::{Index, IndexMut, Mul, Div},
};

pub struct Rk4<T, S> {
    init: T,
    soe: S,
    h: f64,
}

impl<T, S> Rk4<T, S> {
    pub fn new(init: T, soe: S, h: f64) -> Self {
        Self { init, soe, h }
    }
}

impl<T, S> Iterator for Rk4<T, S>
where
    S: Soe<Args = T>,
    T: Default + Copy + AsMut<[f64]> + Add<T, Output=T> + Mul<f64, Output=T>
       + Div<f64, Output=T>
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let mut k1 = self.soe.call(&self.init);
        k1.as_mut()[0] = 1.0f64;

        let next_params = self.init + k1 * self.h / 2.0;

        let mut k2 = self.soe.call(&next_params);
        k2.as_mut()[0] = 1.0f64;

        let next_params = self.init + k2 * self.h / 2.0;

        let mut k3 = self.soe.call(&next_params);
        k3.as_mut()[0] = 1.0f64;

        let next_params = self.init + k3 * self.h;

        let mut k4 = self.soe.call(&next_params);
        k4.as_mut()[0] = 1.0f64;

        let next_step = self.init + (k1 + k2 * 2.0 + k3 * 2.0 + k4) * self.h / 6.0;

        self.init = next_step;

        Some(next_step)
    }
}
