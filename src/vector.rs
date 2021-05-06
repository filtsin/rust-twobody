use ndarray::{ArrayView1, ArrayViewMut1, IntoNdProducer, Ix1, NdProducer};
use std::fmt::Display;
use std::ops::{Add, Deref, DerefMut, Div, Mul, Sub};
use std::{
    convert::{AsMut, AsRef, From},
    ops::AddAssign,
};

#[derive(Debug, Clone, Copy)]
pub struct Vector<T, const N: usize> {
    pub data: [T; N],
}

impl<T, const N: usize> Vector<T, N>
where
    T: Default + Copy,
{
    pub fn new() -> Self {
        Self {
            data: [T::default(); N],
        }
    }

    /// Construct `result` vector with length `N`
    /// from two vectors `vec1` with length N1 and
    /// `vec2` with length N2.
    ///
    /// If `N1` + `N2` == `N` then `result` is full copy
    /// of `vec1` and `vec2`
    ///
    /// If `N1` + `N2` < `N` then `result` will be padded
    /// with leading default `T` for `N` length.
    ///
    /// If `N1` + `N2` > `N` then `result` will be cut to
    /// `N` length.
    ///
    /// # Examples
    ///
    /// ```
    /// let v1 = Vector::<i32, 2> { data: [1, 2] };
    /// let v2 = Vector::<i32, 3> { data: [3, 4, 5] };
    ///
    /// let res1 = Vector::<i32, 5>::construct_from_two(v1, v2);
    /// assert_eq!(res1.data, [1, 2, 3, 4, 5]);
    /// ```
    pub fn construct_from_two<const N1: usize, const N2: usize>(
        vec1: &Vector<T, N1>,
        vec2: &Vector<T, N2>,
    ) -> Vector<T, N> {
        let mut result = Self::new();

        let mut start = 0;

        if N1 + N2 < N {
            // Can not be const because of
            // `use of generic parameter from outer function` problem
            let delta = N - N1 - N2;
            for i in 0..delta {
                result[i] = T::default();
            }
            start = delta;
        }

        Self::fill_from_vector(&mut result, start, vec1);
        Self::fill_from_vector(&mut result, start + vec1.len(), vec2);

        result
    }

    /// Fill `target` with values from `vec` starting from position `start`
    /// to the end of `target` or `vec`
    ///
    /// # Examples
    ///
    /// ```
    /// let v1 = Vector::<i32, 2> { data: [1, 2] };
    /// let v2 = Vector::<i32, 3>::new();
    ///
    /// Vector::<i32, 3>::fill_from_vector(v2, 0, v1);
    /// assert_eq(v2.data, [1, 2, 0]);
    ///
    /// ```
    pub fn fill_from_vector<const N1: usize>(
        target: &mut Vector<T, N>,
        start: usize,
        vec: &Vector<T, N1>,
    ) {
        let max = std::cmp::min(start + N1, N);
        let slice = &mut target[start..max];

        for i in 0..slice.len() {
            slice[i] = vec[i]
        }
    }
}

impl<T, const N: usize> Default for Vector<T, N>
where
    T: Default + Copy,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<T, const N: usize> Deref for Vector<T, N> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<T, const N: usize> DerefMut for Vector<T, N> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

impl<T, const N: usize> Add for Vector<T, N>
where
    T: Default + Copy + Add<Output = T>,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut result = Self::new();
        for i in 0..N {
            result[i] = self[i] + rhs[i];
        }
        result
    }
}

impl<T, const N: usize> Sub for Vector<T, N>
where
    T: Default + Copy + Sub<Output = T>,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut result = Self::new();
        for i in 0..N {
            result[i] = self[i] - rhs[i];
        }
        result
    }
}

impl<T, const N: usize> Mul<T> for Vector<T, N>
where
    T: Default + Copy + Mul<Output = T>,
{
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        let mut result = Self::new();
        for i in 0..N {
            result[i] = self[i] * rhs;
        }
        result
    }
}

impl<T, const N: usize> Div<T> for Vector<T, N>
where
    T: Default + Copy + Div<Output = T>,
{
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        let mut result = Self::new();
        for i in 0..N {
            result[i] = self[i] / rhs;
        }
        result
    }
}

impl<T, const N: usize> AsRef<[T]> for Vector<T, N> {
    fn as_ref(&self) -> &[T] {
        self
    }
}

impl<T, const N: usize> AsMut<[T]> for Vector<T, N> {
    fn as_mut(&mut self) -> &mut [T] {
        self
    }
}

impl<T, const N: usize> From<[T; N]> for Vector<T, N> {
    fn from(data: [T; N]) -> Self {
        Self { data }
    }
}

impl<T, const N: usize> Display for Vector<T, N>
where
    T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.len() >= 1 {
            write!(f, "{}", self.data[0])?;
        }

        for i in 1..self.len() {
            write!(f, ",{}", self.data[i])?;
        }

        Ok(())
    }
}

pub type Vector1 = Vector<f64, 1>;
pub type Vector2 = Vector<f64, 2>;
pub type Vector3 = Vector<f64, 3>;
pub type Vector4 = Vector<f64, 4>;
pub type Vector5 = Vector<f64, 5>;
pub type Vector6 = Vector<f64, 6>;
pub type Vector7 = Vector<f64, 7>;

#[test]
fn test_fill_from_vector() {
    let v1 = Vector::<i32, 2> { data: [1, 2] };
    let mut v2 = Vector::<i32, 3>::new();

    Vector::<i32, 3>::fill_from_vector(&mut v2, 0, &v1);

    assert_eq!(v2.data, [1, 2, 0]);

    let mut v3 = Vector::<i32, 5>::new();

    Vector::<i32, 5>::fill_from_vector(&mut v3, 4, &v1);

    assert_eq!(v3.data, [0, 0, 0, 0, 1]);
}

#[test]
fn test_construct_from_two() {
    let v1 = Vector::<i32, 2> { data: [1, 2] };
    let v2 = Vector::<i32, 3> { data: [3, 4, 5] };

    let res1 = Vector::<i32, 5>::construct_from_two(&v1, &v2);

    assert_eq!(res1.data, [1, 2, 3, 4, 5]);

    let res2 = Vector::<i32, 6>::construct_from_two(&v1, &v2);

    assert_eq!(res2.data, [0, 1, 2, 3, 4, 5]);

    let res3 = Vector::<i32, 3>::construct_from_two(&v1, &v2);

    assert_eq!(res3.data, [1, 2, 3]);
}
