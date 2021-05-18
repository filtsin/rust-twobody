use crate::soe::Soe;

pub mod ab2;
pub mod am2;
pub mod euler;
pub mod rk4;
pub mod rk45;

fn call_soe<T, S>(soe: &mut S, args: &T) -> T
where
    T: AsMut<[f64]>,
    S: Soe<Args = T>,
{
    let mut result = soe.call(&args);
    result.as_mut()[0] = 1.0f64;
    result
}

pub fn abs<T>(v: &T) -> f64
where
    T: AsRef<[f64]>,
{
    let mut result = 0.0;

    for el in v.as_ref() {
        result += el * el;
    }

    result.sqrt()
}
