use crate::soe::Soe;
use ndarray::Zip;
use std::{iter::Iterator, ops::{Add, Mul, Div}};

pub struct Euler<T, S> {
    init: T,
    soe: S,
    h: f64
}

impl<T, S> Euler<T, S> {
    pub fn new(init: T, soe: S, h: f64) -> Self {
        Self { init, soe, h }
    }
}

impl<T, S> Iterator for Euler<T, S>
where
    S: Soe<Args=T>,
    T: Default + Copy + AsMut<[f64]> + Add<T, Output=T> + Mul<f64, Output=T>
       + Div<f64, Output=T>
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let mut result = self.soe.call(&self.init);
        result.as_mut()[0] = 1.0f64;

        let tmp = self.init + result * self.h;

        let mut prediction = self.soe.call(&tmp);
        prediction.as_mut()[0] = 1.0f64;

        let tmp = self.init + (result + prediction) * self.h / 2.0;

        self.init = tmp;

        Some(tmp)
    }

}
