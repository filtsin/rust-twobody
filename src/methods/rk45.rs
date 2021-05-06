use crate::soe::Soe;
use std::iter::Iterator;
use std::ops::{Add, Mul};

pub struct Rk45<T, S> {
    init: T,
    soe: S,
    h: f64,
    e: f64,
}

impl<T, S> Rk45<T, S> {
    pub fn new(init: T, soe: S, h: f64, e: f64) -> Self {
        Self { init, soe, h, e }
    }
}

impl<T, S> Iterator for Rk45<T, S> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
    }
}
