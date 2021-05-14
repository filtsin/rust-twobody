use std::fmt::Debug;
use std::fmt::Display;

use crate::{
    methods::{ab2::Ab2, am2::Am2, euler::Euler, rk4::Rk4, rk45::Rk45},
    soe::{Soe, Soe2, Soe2Builder},
    vector::{Vector, Vector2, Vector3, Vector5, Vector7},
    kepler::Kepler
};

pub type VType = f64;

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
        let r: Vector<VType, 2> = [data[1], data[2]].into();

        let body1 = ((self.a * data[0] + self.b) - r * self.m2) / (self.m1 + self.m2);

        let body2 = ((self.a * data[0] + self.b) + r * self.m1) / (self.m1 + self.m2);

        Position { body1, body2 }
    }
}

impl TwoBodyReader<3> {
    pub fn get(&self, data: Vector<VType, 7>) -> Position<3> {
        let r: Vector<VType, 3> = [data[1], data[2], data[3]].into();

        let body1 = ((self.a * data[0] + self.b) - r * self.m2) / (self.m1 + self.m2);

        let body2 = ((self.a * data[0] + self.b) + r * self.m1) / (self.m1 + self.m2);

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
            let _result = Vector::<VType, 2>::new();
            let r: Vector<VType, 2> = [args[1], args[2]].into();

            let sum_sq = r[0].powi(2) + r[1].powi(2);
            let len_inpow3 = sum_sq * sum_sq.sqrt();

            r * (self.body1.m + self.body2.m) * -self.g / len_inpow3
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

    pub fn construct_rk45(
        &self,
        h: VType,
        e: VType,
        max: VType,
    ) -> impl Iterator<Item = Vector<VType, 5>> {
        Rk45::new(self.get_init(), self.generate_soe(), h, e, max)
    }

    pub fn construct_ab2(
        &self,
        h: VType,
        init2: Vector<VType, 5>,
    ) -> impl Iterator<Item = Vector<VType, 5>> {
        Ab2::new(self.get_init(), init2, self.generate_soe(), h)
    }

    pub fn construct_am2(
        &self,
        h: VType,
        init2: Vector<VType, 5>,
    ) -> impl Iterator<Item = Vector<VType, 5>> {
        Am2::new(self.get_init(), init2, self.generate_soe(), h)
    }

    pub fn construct_kepler(
        &self,
        h: VType
    ) -> impl Iterator<Item = Vector<VType, 5>> {
        let init = self.get_init();
        Kepler::new([init[1], init[2], 0.0].into(), [init[3], init[4], 0.0].into(), self.g * self.body1.m * 10.0, h)
    }
}

impl TwoBodySystem<3> {
    // Generate system of equations
    pub fn generate_soe(self) -> impl Soe<Args = Vector<VType, 7>> {
        let f1 = |args: &Vector<VType, 7>| Vector::<VType, 3> {
            data: [args[4], args[5], args[6]],
        };

        let f2 = move |args: &Vector<VType, 7>| {
            let _result = Vector::<VType, 3>::new();
            let r: Vector<VType, 3> = [args[1], args[2], args[3]].into();

            let sum_sq = r[0].powi(2) + r[1].powi(2) + r[2].powi(3);
            let len_inpow3 = sum_sq * sum_sq.sqrt();

            r * (self.body1.m + self.body2.m) * -self.g / len_inpow3
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

    pub fn construct_rk45(
        &self,
        h: VType,
        e: VType,
        max: VType,
    ) -> impl Iterator<Item = Vector<VType, 7>> {
        Rk45::new(self.get_init(), self.generate_soe(), h, e, max)
    }

    pub fn construct_ab2(
        &self,
        h: VType,
        init2: Vector<VType, 7>,
    ) -> impl Iterator<Item = Vector<VType, 7>> {
        Ab2::new(self.get_init(), init2, self.generate_soe(), h)
    }

    pub fn construct_am2(
        &self,
        h: VType,
        init2: Vector<VType, 7>,
    ) -> impl Iterator<Item = Vector<VType, 7>> {
        Am2::new(self.get_init(), init2, self.generate_soe(), h)
    }
}
