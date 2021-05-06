use crate::soe::Soe;
use ndarray::Zip;
use std::{
    iter::Iterator,
    ops::{Add, Div, Mul},
};

use super::call_soe;

pub struct Euler<T, S> {
    init: T,
    soe: S,
    h: f64,
}

impl<T, S> Euler<T, S> {
    pub fn new(init: T, soe: S, h: f64) -> Self {
        Self { init, soe, h }
    }
}

impl<T, S> Iterator for Euler<T, S>
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
        let result = call_soe(&mut self.soe, &self.init);

        let tmp = self.init + result * self.h;

        let prediction = call_soe(&mut self.soe, &tmp);

        self.init = self.init + (result + prediction) * self.h / 2.0;

        Some(self.init)
    }
}
