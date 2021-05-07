mod methods;
mod soe;
mod twobody;
mod vector;

use methods::euler::Euler;
use methods::rk4::Rk4;
use methods::rk45::Rk45;
use methods::ab2::Ab2;
use methods::am2::Am2;
use soe::{SimpleSoe, SimpleSoeBuilder, Soe};
use twobody::{Body2d, TwoBodyReader2d, TwoBodySystem2d};
use vector::{Vector, Vector1, Vector2};

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
    let h = 0.001;
    let mut rk4_solver = system.construct_rk45(h, 0.0000001, 1000000.0);

    // Solver impl `Iterator` so we can just call next
    // to get some values
    //
    // # Example
    //
    // ```
    // let result: Vec<_> = rk4_solver.take(10).collect();
    // ```

    let limit = 50000;

    for i in 0..limit {
        let solve_step = rk4_solver.next().unwrap();
        let position = reader.get(solve_step);
        println!("{}", position);
    }
}
