use criterion::{Criterion, criterion_main, criterion_group, Fun};
use two_body::{TwoBodySystem2d, Body2d};

fn create_system() -> TwoBodySystem2d {
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

    TwoBodySystem2d::new(body1, body2, g)
}

fn criterion_benchmark(c: &mut Criterion) {

    let system = create_system();

    let h = 0.1;

    let mut rk4 = system.construct_rk4(h);
    let mut euler = system.construct_euler(h);
    let mut rk45 = system.construct_rk45(h, 0.00001, 10000000.0);
    
    // Hack: get 2nd init parameter for ab2 and am2
    let next_step = rk4.next().unwrap();

    let mut ab2 = system.construct_ab2(h, next_step);
    let mut am2 = system.construct_am2(h, next_step);

    let mut group = c.benchmark_group("Solvers");

    group.bench_function("Rk4", |b| b.iter(|| rk4.next()));
    group.bench_function("Euler", |b| b.iter(|| euler.next()));
    group.bench_function("Rk45", |b| b.iter(|| rk45.next()));
    group.bench_function("Ab2", |b| b.iter(|| ab2.next()));
    group.bench_function("Am2", |b| b.iter(|| am2.next()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

