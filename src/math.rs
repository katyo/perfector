/// A marker trait for types which represents points in 3D
///
/// The points is represented as `[f64; 3]`.
/// Technically any values with size `8*3` bytes may be used as point.
pub trait IsPoint: Sized {}

/// A marker trait for types which represents vectors in 3D
///
/// The vectors is represented as `[f64; 3]`.
/// Technically any values with size `8*3` bytes may be used as vector.
pub trait IsVector: Sized {}

#[cfg(test)]
pub use nalgebra_glm::DVec3;

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Default)]
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl From<(f64, f64, f64)> for Point {
    fn from((x, y, z): (f64, f64, f64)) -> Self {
        Self { x, y, z }
    }
}

impl From<Point> for (f64, f64, f64) {
    fn from(Point { x, y, z }: Point) -> Self {
        (x, y, z)
    }
}

impl From<[f64; 3]> for Point {
    fn from([x, y, z]: [f64; 3]) -> Self {
        Self { x, y, z }
    }
}

impl From<Point> for [f64; 3] {
    fn from(Point { x, y, z }: Point) -> Self {
        [x, y, z]
    }
}

impl Point {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }
}

impl IsPoint for Point {}

impl IsPoint for (f64, f64, f64) {}
impl IsVector for (f64, f64, f64) {}
impl IsPoint for [f64; 3] {}
impl IsVector for [f64; 3] {}

#[cfg(feature = "glam")]
mod glam_impls {
    use super::*;

    impl IsPoint for glam::f64::DVec3 {}
    impl IsVector for glam::f64::DVec3 {}
}

#[cfg(feature = "nalgebra-glm")]
mod glm_impls {
    use super::*;

    impl IsPoint for nalgebra_glm::DVec3 {}
    impl IsVector for nalgebra_glm::DVec3 {}
}

#[cfg(feature = "euclid")]
mod euclid_impls {
    use super::*;

    impl<U> IsPoint for euclid::Point3D<f64, U> {}
    impl<U> IsVector for euclid::Point3D<f64, U> {}
}

#[cfg(feature = "ultraviolet")]
mod uv_impls {
    use super::*;

    impl IsPoint for ultraviolet::DVec3 {}
    impl IsVector for ultraviolet::DVec3 {}
}

#[cfg(feature = "vek")]
mod vek_impls {
    use super::*;

    impl IsPoint for vek::Vec3<f64> {}
    impl IsVector for vek::Vec3<f64> {}
}
