mod methods;
mod soe;
mod twobody;
mod vector;
mod kepler;

use twobody::{Body2d, TwoBodySystem2d};
use kepler::Kepler;
use crate::{methods::abs, vector::Vector2};

use std::process::exit;

fn main() {
// Create bodies
    let body1 = Body2d {
        m: 5.0,
        pos: [0.0, 0.0].into(),
        velocity: [0.5, 0.0].into()
    };

    let body2 = Body2d {
        m: 5.0,
        pos: [1.0, 1.0].into(),
        velocity: [-0.5, 0.0].into()
    };

    let g = 0.1;

    // Generate system
    let system = TwoBodySystem2d::new(body1, body2, g);

    // Build reader for converting 5d vector
    // of r position and velocity into
    // structure with `body1` and `body2` position
    let reader = system.build_reader();

    // Choose method for solving
    let h = 0.00001;
    let mut solver = system.construct_rk4(h);
    let mut solver = system.construct_rk45(h, 0.00000000001, 5.0);
    // Solver impl `Iterator` so we can just call next
    // to get some values
    //
    // # Example
    //
    // ```
    // let result: Vec<_> = rk4_solver.take(10).collect();
    // ```
    
    let init = system.get_init();
    let mut kepler = Kepler::new([init[1], init[2], 0.0].into(), [init[3], init[4], 0.0].into(), g * (body1.m + body2.m), h);
    
//    let mut kepler = system.construct_kepler(h);

    let limit = 500000;

    for i in 0..limit {
        let solve_step = match solver.next() {
            Some(v) => v,
            None => { exit(0) }
        };
        let position = reader.get(solve_step);
        println!("{},{},{}", solve_step[0], solve_step[1], solve_step[2]);
    }
}
