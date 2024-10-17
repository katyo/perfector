mod transformation;
mod angle;

pub use transformation::*;
pub use angle::*;

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Default)]
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Point {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }
}

impl AsRef<Point> for Point {
    fn as_ref(&self) -> &Self {
        self
    }
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Default)]
pub struct Vector {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }
}

impl AsRef<Vector> for Vector {
    fn as_ref(&self) -> &Self {
        self
    }
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Quaternion {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

impl Default for Quaternion {
    fn default() -> Self {
        Self::new(0.0, 0.0, 0.0, 1.0)
    }
}

impl Quaternion {
    pub fn new(x: f64, y: f64, z: f64, w: f64) -> Self {
        Self { x, y, z, w }
    }
}

impl AsRef<Quaternion> for Quaternion {
    fn as_ref(&self) -> &Self {
        self
    }
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Default)]
pub struct Axis1 {
    pub point: Point,
    pub dir: Vector,
}

impl Axis1 {
    pub fn new(point: impl Into<Point>, dir: impl Into<Vector>) -> Self {
        Self { point: point.into(), dir: dir.into() }
    }
}

impl<P, V> From<(P, V)> for Axis1
where
    Point: From<P>,
    Vector: From<V>,
{
    fn from((p, v): (P, V)) -> Self {
        Self::new(p, v)
    }
}

impl AsRef<Axis1> for Axis1 {
    fn as_ref(&self) -> &Self {
        self
    }
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Default)]
pub struct Axis3 {
    pub axis: Axis1,
    pub ydir: Vector,
    pub xdir: Vector,
}

impl Axis3 {
    pub fn new(axis: impl Into<Axis1>, ydir: impl Into<Vector>, xdir: impl Into<Vector>) -> Self {
        Self { axis: axis.into(), ydir: ydir.into(), xdir: xdir.into() }
    }
}

impl<A, Y, X> From<(A, Y, X)> for Axis3
where
    Axis1: From<A>,
    Vector: From<Y>,
    Vector: From<X>,
{
    fn from((a, y, x): (A, Y, X)) -> Self {
        Self::new(a, y, x)
    }
}

impl AsRef<Axis3> for Axis3 {
    fn as_ref(&self) -> &Self {
        self
    }
}

macro_rules! math_type {
    ( $( $mtype:ident { $( $(#[$($meta:meta)*])* $({$($par:ident),*})* $type:ty; )* } )* ) => {
        $(
            $(
                $(#[$($meta)*])*
                impl $(<$($par),*>)* AsRef<$type> for $mtype {
                    fn as_ref(&self) -> &$type {
                        unsafe { core::mem::transmute(self) }
                    }
                }

                $(#[$($meta)*])*
                impl $(<$($par),*>)* AsMut<$type> for $mtype {
                    fn as_mut(&mut self) -> &mut $type {
                        unsafe { core::mem::transmute(self) }
                    }
                }

                $(#[$($meta)*])*
                impl $(<$($par),*>)* AsRef<$mtype> for $type {
                    fn as_ref(&self) -> &$mtype {
                        unsafe { core::mem::transmute(self) }
                    }
                }

                $(#[$($meta)*])*
                impl $(<$($par),*>)* AsMut<$mtype> for $type {
                    fn as_mut(&mut self) -> &mut $mtype {
                        unsafe { core::mem::transmute(self) }
                    }
                }

                $(#[$($meta)*])*
                impl $(<$($par),*>)* From<$type> for $mtype {
                    fn from(val: $type) -> Self {
                        unsafe { core::mem::transmute(val) }
                    }
                }

                $(#[$($meta)*])*
                impl $(<$($par),*>)* From<$mtype> for $type {
                    fn from(val: $mtype) -> Self {
                        unsafe { core::mem::transmute(val) }
                    }
                }
            )*
        )*
    }
}

math_type! {
    Point {
        (f64, f64, f64);
        [f64; 3];

        #[cfg(feature = "glam")]
        glam::f64::DVec3;

        #[cfg(feature = "nalgebra-glm")]
        nalgebra_glm::DVec3;

        #[cfg(feature = "euclid")]
        {U} euclid::Point3D<f64, U>;

        #[cfg(feature = "ultraviolet")]
        ultraviolet::DVec3;

        #[cfg(feature = "vek")]
        vek::Vec3<f64>;
    }

    Vector {
        (f64, f64, f64);
        [f64; 3];

        #[cfg(feature = "glam")]
        glam::f64::DVec3;

        #[cfg(feature = "nalgebra-glm")]
        nalgebra_glm::DVec3;

        #[cfg(feature = "euclid")]
        {U} euclid::Point3D<f64, U>;

        #[cfg(feature = "ultraviolet")]
        ultraviolet::DVec3;

        #[cfg(feature = "vek")]
        vek::Vec3<f64>;
    }

    Quaternion {
        (f64, f64, f64, f64);
        [f64; 4];

        #[cfg(feature = "glam")]
        glam::f64::DQuat;

        #[cfg(feature = "nalgebra-glm")]
        nalgebra_glm::DQuat;

        #[cfg(feature = "euclid")]
        {U, V} euclid::Rotation3D<f64, U, V>;

        #[cfg(feature = "vek")]
        vek::Quaternion<f64>;
    }
}

#[cfg(feature = "ultraviolet")]
impl From<ultraviolet::DRotor3> for Quaternion {
    fn from(val: ultraviolet::DRotor3) -> Self {
        val.into_quaternion_array().into()
    }
}

#[cfg(feature = "ultraviolet")]
impl From<Quaternion> for ultraviolet::DRotor3 {
    fn from(val: Quaternion) -> Self {
        Self::from_quaternion_array(val.into())
    }
}
