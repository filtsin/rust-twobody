mod methods;
mod soe;
mod twobody;
mod vector;

use methods::ab2::Ab2;
use methods::am2::Am2;
use methods::euler::Euler;
use methods::rk4::Rk4;
use methods::rk45::Rk45;
use soe::{SimpleSoe, SimpleSoeBuilder, Soe};
use twobody::{Body2d, TwoBodyReader2d, TwoBodySystem2d};
use vector::{Vector, Vector1, Vector2};

fn main() {
    //    // Just an example for solving simple y' = xy
    //    let f = |args: &Vector2| Vector1 {
    //        data: [args[0] * args[1]],
    //    };
    //    let soe = SimpleSoeBuilder::<f64, 2, 1>::new().build(f);
    //
    //    let init = Vector2 { data: [0.0, 1.0] };
    //
    //    let mut rk = Am2::new(Vector2 { data: [0.0, 1.0] }, Vector2 { data: [0.01, 1.0] }, soe, 0.01);
    //
    //    let vec: Vec<Vector2> = rk.take(400).collect();
    //
    //    for Vector2 { data: [x, y] } in vec.iter() {
    //        println!("{},{}", x, y);
    //    }
    //
    // Create bodies
    let body1 = Body2d {
        m: 5.0,
        pos: [0.0, 0.0].into(),
        velocity: [0.5, 0.0].into(),
    };

    let body2 = Body2d {
        m: 5.0,
        pos: [1.0, 1.0].into(),
        velocity: [-0.5, 0.0].into(),
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
    let mut solver = system.construct_rk45(h, 0.0000001, 100.0);

    // Solver impl `Iterator` so we can just call next
    // to get some values
    //
    // # Example
    //
    // ```
    // let result: Vec<_> = rk4_solver.take(10).collect();
    // ```

    let limit = 2000;

    for i in 0..limit {
        let solve_step = match solver.next() {
            Some(v) => { v },
            None => { return; }
        };
        let position = reader.get(solve_step);
        println!("{}", position);
    }
}
