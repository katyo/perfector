mod transformation;

pub use transformation::*;

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
