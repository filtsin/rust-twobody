use crate::twobody::TwoBodySystem2d;
use crate::vector::{Vector3, Vector5};
use std::f64::consts::{self, PI};

#[derive(Debug)]
pub struct Kepler {
    // Kepler's parameters
    
    // Semi-major axis
    a: f64,
    // Eccentricity
    e: f64,
    // Argument of periapsis
    w: f64,
    // Longitute of ascending node (LAN)
    omega: f64,
    // Inclination
    i: f64,
    // Mean anomaly
    M: f64,
    // std gravitational parameter
    mu: f64,

    // Other parameters for iterator
   
    // Init time
    t0: f64,
    // Current time
    t: f64,
    // Time step
    step: f64
}

fn vec_len(vec: Vector3) -> f64 {
    (vec[0].powi(2) + vec[1].powi(2) + vec[2].powi(2)).sqrt()
}

fn scalar_mul(v1: Vector3, v2: Vector3) -> f64 {
    v1[0] * v2[0] + v1[1] * v2[1] + v1[2] * v2[2]
}

fn angle_between(v1: Vector3, v2: Vector3) -> f64 {
    scalar_mul(v1, v2) / (vec_len(v1) * vec_len(v2))
}

impl Kepler {
    pub fn new(r: Vector3, v: Vector3, mu: f64, step: f64) -> Self {

        dbg!(r, v);
        // 1. a) Calculate orbital momentum vector h
        let h = r * v;

        dbg!(h);

        // 1. b) Obtain the eccentricity vector e
        let e_vec = (v * h) / mu - r / vec_len(r);

        dbg!(e_vec);

        // 1. c) Determine the vector n pointing towards the asscending
        // node and the true anomaly nu
        let n: Vector3 = [-h[1], h[0], 0.0].into();

        dbg!(n);

        let nu = if scalar_mul(r, v) >= 0.0 {
            angle_between(e_vec, r).acos()
        } else {
            2.0 * PI - angle_between(e_vec, r).acos()
        };


        dbg!(nu);

        // 2. Calculate the orbit inclination i
        let i = (h[2] / vec_len(h)).acos();

        dbg!(i);

        // 3. Determine the orbit eccentricity e and the eccentric anomaly E
        let e = vec_len(e_vec);
        assert!(e < 1.0);
        let E = 2.0 * ((nu / 2.0).tan() / ((1.0 + e) / (1.0 - e)).sqrt()).atan();

        dbg!(e);
        dbg!(E);

        // 4. Obtain the longitute of ascending node omega
        // and the argument of periapsis w

        dbg!(i);

        dbg!(i == PI);

        let omega = if i.abs() < 0.0001 || i == PI {
            0.0
        } else {
            let res = (n[0] / vec_len(n)).acos();
            if n[1] < 0.0 {
                2.0 * PI - res
            } else {
                res
            }
        };

        dbg!(omega);

        let w = if e.abs() < 0.0001 {
            0.0
        } else {
            let res = if i.abs() < 0.0001 || i == PI {
                e_vec[1].atan2(e_vec[0])
            } else {
                angle_between(n, e_vec).acos()
            };
            if e_vec[2] < 0.0 || i == PI {
                2.0 * PI - res
            } else {
                res
            }
        };

        dbg!(w);
        
        // 5. Compute the mean anomaly M
        let M = E - e * E.sin();

        dbg!(M);

        // 6. Compute the semi-major axis a
        let a = 1.0 / ((2.0 / vec_len(r)) - ((vec_len(v).powi(2) / mu)));

        dbg!(a);

        dbg!(step);

        Self { a, e, w, omega, i, M, mu, t0: 0.0, t: 0.0 + step, step }
    }

    pub fn set_current_time(&mut self, t: f64) {
        self.t = t;
    }

    pub fn set_init_time(&mut self, t: f64) {
        self.t0 = t;
    }
}

impl Iterator for Kepler {
    type Item = Vector5;

    // Return Vector5 = [t, x, y, z, 0]
    // Last component is always zero
    fn next(&mut self) -> Option<Self::Item> {
        // 1. Calculate Mt
        // i. Determine the time difference
        let delta_t = self.t - self.t0;
        // ii. Calculate mean anomaly Mt
        let Mt = self.M + delta_t * (self.mu / self.a.powi(3)).sqrt();


        // 2. Solve Kepler's Equation: Mt = Et - esinE using Newton's method
        let mut E = Mt;
        let mut F = E - self.e * E.sin() - Mt;

        let max_iter = 30;
        let delta = 0.00000001;

        for i in 0..max_iter {
            E = E - F / (1.0 - self.e * E.cos());
            F = E - self.e * E.sin() - Mt;
            if F.abs() < delta { break; }
        }


        // 3. Obtain the true anomaly nut
        let nut = 2.0 * ((1.0 + self.e).sqrt() * (E / 2.0).sin()).atan2((1.0 - self.e).sqrt() * (E / 2.0).cos());

        // 4. Use the eccentric anomaly to get the distance to the central body with
        let rc = self.a * (1.0 - self.e * E.cos());

        // 5. Obtain the position vector ot
        let ot = Vector3 { data: [nut.cos(), nut.sin(), 0.0] } * rc;

        // 6. Transform ot to the rectangular coordiantes r
        
        let x = ot[0] * (self.w.cos() * self.omega.cos() - self.w.sin() * self.i.cos() * self.omega.sin())
            - ot[1] * (self.w.sin() * self.omega.cos() + self.w.cos() * self.i.cos() * self.omega.sin());

        let y = ot[0] * (self.w.cos() * self.omega.sin() + self.w.sin() * self.i.cos() * self.omega.cos())
            + ot[1] * (self.w.cos() * self.i.cos() * self.omega.cos() - self.w.sin() * self.omega.sin());

        let z = ot[0] * (self.w.sin() * self.i.sin()) + ot[1] * (self.w.cos() * self.i.sin());

        self.t += self.step;
        
        Some ( [(self.t - self.step), x, y, z, 0.0].into() )
    }
}

