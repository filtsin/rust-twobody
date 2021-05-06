use ndarray::Zip;
use std::{fmt::Debug, marker::PhantomData, ops::Deref};
use std::fmt::Display;

use crate::{
    methods::{rk4::Rk4, euler::Euler},
    soe::{Soe, Soe2, Soe2Builder},
    vector::{Vector, Vector2, Vector3, Vector5, Vector7},
};

type VType = f64;

/// One of the body with initial parameters
#[derive(Debug, Clone, Copy)]
pub struct Body<const N: usize> {
    /// Mass
    pub m: VType,
    /// Position
    pub pos: Vector<VType, N>,
    /// Initial speed
    pub velocity: Vector<VType, N>,
}

pub type Body2d = Body<2>;
pub type Body3d = Body<3>;

#[derive(Debug, Clone, Copy)]
pub struct TwoBodySystem<const N: usize> {
    body1: Body<N>,
    body2: Body<N>,
    /// Gravity constant
    g: VType,
}

pub type TwoBodySystem2d = TwoBodySystem<2>;
pub type TwoBodySystem3d = TwoBodySystem<3>;

#[derive(Debug, Clone, Copy)]
pub struct TwoBodyReader<const N: usize> {
    a: Vector<VType, N>,
    b: Vector<VType, N>,
    m1: VType,
    m2: VType,
}

pub type TwoBodyReader2d = TwoBodyReader<2>;
pub type TwoBodyReader3d = TwoBodyReader<3>;

#[derive(Debug, Clone, Copy)]
pub struct Position<const N: usize> {
    pub body1: Vector<VType, N>,
    pub body2: Vector<VType, N>,
}

impl<const N: usize> Display for Position<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{},{}", self.body1, self.body2)
    }
}

impl TwoBodyReader<2> {
    pub fn get(&self, data: Vector<VType, 5>) -> Position<2> {
        let mut body1 = Vector::<VType, 2>::new();
        Zip::from(&mut body1)
            .and(&self.a)
            .and(&self.b)
            .and(&data[1..3])
            .for_each(|res, &a, &b, &r| {
                *res = ((a * data[0] + b) - (self.m2 * r)) / (self.m1 + self.m2)
            });

        let mut body2 = Vector::<VType, 2>::new();
        Zip::from(&mut body2)
            .and(&self.a)
            .and(&self.b)
            .and(&data[1..3])
            .for_each(|res, &a, &b, &r| {
                *res = ((a * data[0] + b) + (self.m2 * r)) / (self.m1 + self.m2)
            });

        Position { body1, body2 }
    }
}

impl TwoBodyReader<3> {
    pub fn get(&self, data: Vector<VType, 7>) -> Position<3> {
        let mut body1 = Vector::<VType, 3>::new();
        Zip::from(&mut body1)
            .and(&self.a)
            .and(&self.b)
            .and(&data[1..4])
            .for_each(|res, &a, &b, &r| {
                *res = ((a * data[0] + b) - (self.m2 * r)) / (self.m1 + self.m2)
            });

        let mut body2 = Vector::<VType, 3>::new();
        Zip::from(&mut body2)
            .and(&self.a)
            .and(&self.b)
            .and(&data[1..4])
            .for_each(|res, &a, &b, &r| {
                *res = ((a * data[0] + b) + (self.m2 * r)) / (self.m1 + self.m2)
            });

        Position { body1, body2 }
    }
}

impl<const N: usize> TwoBodySystem<N> {
    pub fn new(body1: Body<N>, body2: Body<N>, g: VType) -> Self {
        Self { body1, body2, g }
    }
}

impl TwoBodySystem<2> {
    /// Generate system of equations
    pub fn generate_soe(self) -> impl Soe<Args = Vector<VType, 5>> {
        let f1 = |args: &Vector<VType, 5>| Vector::<VType, 2> {
            data: [args[3], args[4]],
        };

        let f2 = move |args: &Vector<VType, 5>| {
            let mut result = Vector::<VType, 2>::new();
            let r = &args[1..3];

            let sum_sq = r[0].powi(2) + r[1].powi(2);
            let len_inpow3 = sum_sq * sum_sq.sqrt();

            Zip::from(&mut result).and(r).for_each(|res, &r| {
                *res = -self.g * (self.body1.m + self.body2.m) * r / len_inpow3;
            });
            result
        };

        Soe2Builder::<f64, 5, 2>::new().build(f1, f2)
    }

    /// Get init vector
    /// (t0, r0x, r0y, v0x, v0y)
    /// where
    ///
    /// t0 - start of time
    ///
    /// (r0x, r0y) - initial position of vector between `body1` and `body2`
    ///
    /// (v0x, v0y) - initial speed of vector between `body` and `body2`
    pub fn get_init(&self) -> Vector<VType, 5> {
        Vector::<VType, 5>::construct_from_two(
            &(self.body2.pos - self.body1.pos),
            &(self.body2.velocity - self.body1.velocity),
        )
    }

    /// Calculation of center of mass movement
    /// R(t) = At + B
    ///
    /// # Returns
    ///
    /// ( (a1, a2), (b1, b2)) vector
    pub fn calc_center(&self) -> (Vector<VType, 2>, Vector<VType, 2>) {
        (
            ((self.body1.velocity * self.body1.m + self.body2.velocity * self.body2.m)
                / (self.body1.m + self.body2.m)),
            ((self.body1.pos * self.body1.m + self.body2.pos * self.body2.m)
                / (self.body1.m + self.body2.m)),
        )
    }

    pub fn build_reader(&self) -> TwoBodyReader<2> {
        let center = self.calc_center();
        TwoBodyReader {
            a: center.0,
            b: center.1,
            m1: self.body1.m,
            m2: self.body2.m,
        }
    }

    /// Construct rk4 solver with `h` step
    pub fn construct_rk4(&self, h: VType) -> impl Iterator<Item = Vector<VType, 5>> {
        Rk4::new(self.get_init(), self.generate_soe(), h)
    }

    pub fn construct_euler(&self, h: VType) -> impl Iterator<Item = Vector<VType, 5>> {
        Euler::new(self.get_init(), self.generate_soe(), h)
    }
}

impl TwoBodySystem<3> {
    // Generate system of equations
    pub fn generate_soe(self) -> impl Soe<Args = Vector<VType, 7>> {
        let f1 = |args: &Vector<VType, 7>| Vector::<VType, 3> {
            data: [args[4], args[5], args[6]],
        };

        let f2 = move |args: &Vector<VType, 7>| {
            let mut result = Vector::<VType, 3>::new();
            let r = &args[1..4];

            let sum_sq = r[0].powi(2) + r[1].powi(2) + r[2].powi(3);
            let len_inpow3 = sum_sq * sum_sq.sqrt();

            Zip::from(&mut result).and(r).for_each(|res, &r| {
                *res = -self.g * (self.body1.m + self.body2.m) * r / len_inpow3;
            });
            result
        };

        Soe2Builder::<VType, 7, 3>::new().build(f1, f2)
    }

    /// Get init vector
    /// (t0, r0x, r0y, r0z, v0x, v0y, v0z)
    /// where
    ///
    /// t0 - start of time
    ///
    /// (r0x, r0y, r0z) - initial position of vector between `body1` and `body2`
    ///
    /// (v0x, v0y, v0z) - initial speed of vector between `body` and `body2`
    pub fn get_init(&self) -> Vector<VType, 7> {
        Vector::<VType, 7>::construct_from_two(
            &(self.body2.pos - self.body1.pos),
            &(self.body2.velocity - self.body1.velocity),
        )
    }

    /// Calculation of center of mass movement
    /// R(t) = At + B
    ///
    /// # Returns
    ///
    /// ( (a1, a2, a3), (b1, b2, b3)) vector
    pub fn calc_center(&self) -> (Vector<VType, 3>, Vector<VType, 3>) {
        (
            ((self.body1.velocity * self.body1.m + self.body2.velocity * self.body2.m)
                / (self.body1.m + self.body2.m)),
            ((self.body1.pos * self.body1.m + self.body2.pos * self.body2.m)
                / (self.body1.m + self.body2.m)),
        )
    }

    pub fn build_reader(&self) -> TwoBodyReader<3> {
        let center = self.calc_center();
        TwoBodyReader {
            a: center.0,
            b: center.1,
            m1: self.body1.m,
            m2: self.body2.m,
        }
    }
    /// Construct rk4 solver with `h` step
    pub fn construct_rk4(&self, h: VType) -> impl Iterator<Item = Vector<VType, 7>> {
        Rk4::new(self.get_init(), self.generate_soe(), h)
    }

    pub fn construct_euler(&self, h: VType) -> impl Iterator<Item = Vector<VType, 7>> {
        Euler::new(self.get_init(), self.generate_soe(), h)
    }
}
