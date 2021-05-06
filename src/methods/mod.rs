use crate::soe::Soe;

pub mod rk4;
pub mod euler;
pub mod rk45;

fn call_soe<T, S>(soe: S, args: T) -> T
where
    T: AsMut<[f64]>,
    S: Soe<Args = T>
{
    let result = soe.call(&args);
    result.as_mut()[0] = 1.0f64;
    result
}

