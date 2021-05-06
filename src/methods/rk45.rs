use crate::soe::Soe;
use std::iter::Iterator;
use std::ops::{Add, Mul, Div, Sub};
use super::{abs, call_soe};

pub struct Rk45<T, S> {
    init: T,
    soe: S,
    h: f64,
    e: f64,
    max: f64
}

impl<T, S> Rk45<T, S> {
    pub fn new(init: T, soe: S, h: f64, e: f64, max: f64) -> Self {
        Self { init, soe, h, e, max }
    }
}

impl<T, S> Iterator for Rk45<T, S>
where
    S: Soe<Args = T>,
    T: Default + Copy + AsMut<[f64]> + AsRef<[f64]> + Add<T, Output=T> + Mul<f64, Output=T>
       + Div<f64, Output=T> + Sub<T, Output=T>
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.init.as_ref()[0] > self.max { return None; }

        let k1 = call_soe(&mut self.soe, &self.init) * self.h;

        let next_params = self.init + k1 / 4.0;

        let k2 = call_soe(&mut self.soe, &next_params) * self.h;

        let mut next_params = self.init + k1 * 3.0 / 32.0
                                        + k2 * 9.0 / 32.0;

        next_params.as_mut()[0] = self.init.as_ref()[0] + self.h * 3.0 / 8.0;

        let k3 = call_soe(&mut self.soe, &next_params) * self.h;

        let mut next_params = self.init + (k1 * 1932.0 / 2197.0)
                                        - (k2 * 7200.0 / 2197.0)
                                        + (k3 * 7296.0 / 2197.0);

        next_params.as_mut()[0] = self.init.as_ref()[0] + self.h * 12.0 / 13.0;

        let k4 = call_soe(&mut self.soe, &next_params) * self.h;

        let mut next_params = self.init + (k1 * 439.0 / 216.0)
                                        - (k2 * 8.0)
                                        + (k3 * 3680.0 / 513.0)
                                        - (k4 * 845.0 / 4104.0);

        next_params.as_mut()[0] = self.init.as_ref()[0] + self.h;

        let k5 = call_soe(&mut self.soe, &next_params) * self.h;

        let mut next_params = self.init - (k1 * 8.0 / 27.0)
                                        + (k2 * 2.0)
                                        - (k3 * 3544.0 / 2565.0)
                                        + (k4 * 1859.0 / 4104.0)
                                        - (k5 * 11.0 / 40.0);

        next_params.as_mut()[0] = self.init.as_ref()[0] + self.h / 2.0;

        let k6 = call_soe(&mut self.soe, &next_params) * self.h;

        let mut next  = self.init + k1 * 25.0 / 216.0 + k3 * 1408.0 / 2565.0
                              + k4 * 2197.0 / 4104.0 - k5 / 5.0;

        next.as_mut()[0] = 0.0;

        let mut next_cap  = self.init + k1 * 16.0 / 135.0 + k3 * 6656.0 / 12825.0
                                  + k4 * 28561.0 / 56430.0 - k5  * 9.0 / 50.0
                                  + k6 * 2.0 / 55.0;

        next_cap.as_mut()[0] = 0.0;
        
        let r = abs(&(next_cap - next)) / self.h;

        let sigma = (self.e / r).powf(0.25) * 0.84;

        if r <= self.e {
            next.as_mut()[0] = self.init.as_ref()[0] + self.h;
            self.init = next;
            self.h *= sigma;
            Some(next)
        } else {
            self.h *= sigma;
            self.next()
        }
    }
}
