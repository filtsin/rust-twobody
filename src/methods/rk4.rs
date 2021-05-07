use crate::soe::Soe;

use std::{
    fmt::Debug,
    ops::{Div, Index, IndexMut, Mul},
};
use std::{iter::Iterator, ops::Add};

use super::call_soe;

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
    T: Default
        + Copy
        + AsMut<[f64]>
        + Add<T, Output = T>
        + Mul<f64, Output = T>
        + Div<f64, Output = T>,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let k1 = call_soe(&mut self.soe, &self.init);

        let next_params = self.init + k1 * self.h / 2.0;

        let k2 = call_soe(&mut self.soe, &next_params);

        let next_params = self.init + k2 * self.h / 2.0;

        let k3 = call_soe(&mut self.soe, &next_params);

        let next_params = self.init + k3 * self.h;

        let k4 = call_soe(&mut self.soe, &next_params);

        let next_step = self.init + (k1 + k2 * 2.0 + k3 * 2.0 + k4) * self.h / 6.0;

        self.init = next_step;

        Some(next_step)
    }
}
