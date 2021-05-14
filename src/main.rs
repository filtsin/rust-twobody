mod methods;
mod soe;
mod twobody;
mod vector;
mod kepler;

use twobody::{Body2d, TwoBodySystem2d};

fn main() {
// Create bodies
    let body1 = Body2d {
        m: 10.0,
        pos: [0.0, 0.0].into(),
        velocity: [0.0, 0.0].into()
    };

    let body2 = Body2d {
        m: 0.001,
        pos: [1.0, 1.0].into(),
        velocity: [1.0, 0.0].into()
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
    let mut solver = system.construct_rk4(h);
    let mut solver = system.construct_rk45(h, 0.0000001, 100000.0);

    // Solver impl `Iterator` so we can just call next
    // to get some values
    //
    // # Example
    //
    // ```
    // let result: Vec<_> = rk4_solver.take(10).collect();
    // ```
    
    let mut kepler = system.construct_kepler(h);

    let limit = 500000;

    for i in 0..limit {
        let solve_step = solver.next().unwrap();
        let position = reader.get(solve_step);
        println!("{}", position);
    }
}
