use crate::soe::Soe;

use std::{
    iter::Iterator,
    ops::{Add, Div, Mul, Sub},
};

use super::call_soe;

pub struct Am2<T, S> {
    init1: T,
    init2: T,
    soe: S,
    h: f64,
}

impl<T, S> Am2<T, S> {
    pub fn new(init1: T, init2: T, soe: S, h: f64) -> Self {
        Self {
            init1,
            init2,
            soe,
            h,
        }
    }
}

impl<T, S> Iterator for Am2<T, S>
where
    S: Soe<Args = T>,
    T: Default
        + Copy
        + AsMut<[f64]>
        + AsRef<[f64]>
        + Add<T, Output = T>
        + Mul<f64, Output = T>
        + Div<f64, Output = T>
        + Sub<T, Output = T>,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let result2 = call_soe(&mut self.soe, &self.init2) * self.h;
        let result1 = call_soe(&mut self.soe, &self.init1) * self.h;

        let tmp = self.init2 + result2 * 3.0 / 2.0 - result1 / 2.0;

        let value = call_soe(&mut self.soe, &tmp) * self.h;

        self.init1 = self.init2;
        self.init2 = self.init1 + value * 5.0 / 12.0 + result2 * 2.0 / 3.0 - result1 / 12.0;
        self.init2.as_mut()[0] = self.init1.as_ref()[0] + self.h;

        Some(self.init2)
    }
}
