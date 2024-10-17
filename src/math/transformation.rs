use super::{Point, Quaternion, Vector};
use cpp::{cpp, cpp_class};

cpp! {{
    #include "gp_XYZ.hxx"
    #include "gp_Pnt.hxx"
    #include "gp_Mat.hxx"
    #include "gp_Quaternion.hxx"
    #include "gp_TrsfForm.hxx"
    #include "gp_Trsf.hxx"

    using namespace std;

    static_assert(is_trivially_copyable<gp_Trsf>::value,
                  "gp_Trsf is relocatable");
}}

cpp_class!(pub unsafe struct Transformation as "gp_Trsf");

enum_impls! {
    /// The type of a geometric transformation
    TransformationForm {
        /// No transformation (matrix is identity)
        Identity,
        /// Rotation
        Rotation,
        /// Translation
        Translation,
        /// Central symmetry
        PntMirror,
        /// Rotational symmetry
        Ax1Mirror,
        /// Bilateral symmetry
        Ax2Mirror,
        /// Scale
        Scale,
        /// Combination of the above transformations
        CompoundTrsf,
        /// Transformation with not-orthogonal matrix
        Other,
    }
}

impl Transformation {
    pub fn scale_factor(&self) -> f64 {
        unsafe {
            cpp!([self as "const gp_Trsf*"] -> f64 as "Standard_Real" {
                return self->ScaleFactor();
            })
        }
    }

    pub fn set_scale_factor(&mut self, s: f64) {
        unsafe {
            cpp!([self as "gp_Trsf*", s as "Standard_Real"] {
                self->SetScaleFactor(s);
            })
        }
    }

    pub fn form(&self) -> TransformationForm {
        unsafe {
            cpp!([self as "const gp_Trsf*"] -> TransformationForm as "gp_TrsfForm" {
                return self->Form();
            })
        }
    }

    pub fn set_form(&mut self, form: TransformationForm) {
        unsafe {
            cpp!([self as "gp_Trsf*", form as "gp_TrsfForm"] {
                self->SetForm(form);
            });
        }
    }

    pub fn translation_part<T>(&self) -> &T
    where
        Point: AsRef<T>,
    {
        self._translation_part().as_ref()
    }

    fn _translation_part(&self) -> &Point {
        unsafe {
            cpp!([self as "const gp_Trsf*"] -> &Point as "const gp_XYZ*" {
                return &self->TranslationPart();
            })
        }
    }

    pub fn set_translation_part<T: AsRef<Vector>>(&mut self, v: &T) {
        self._set_translation_part(v.as_ref());
    }

    fn _set_translation_part(&mut self, v: &Vector) {
        unsafe {
            cpp!([self as "gp_Trsf*", v as "const gp_Vec*"] {
                self->SetTranslationPart(*v);
            })
        }
    }

    pub fn rotation_part<T: From<Quaternion>>(&self) -> T {
        self._rotation_part().into()
    }

    fn _rotation_part(&self) -> Quaternion {
        unsafe {
            cpp!([self as "const gp_Trsf*"] -> Quaternion as "gp_Quaternion" {
                return self->GetRotation();
            })
        }
    }

    pub fn set_rotation_part(&mut self, r: &Quaternion) {
        self._set_rotation_part(r);
    }

    fn _set_rotation_part(&mut self, r: &Quaternion) {
        unsafe {
            cpp!([self as "gp_Trsf*", r as "const gp_Quaternion*" ] {
                self->SetRotationPart(*r);
            })
        }
    }

    pub fn set_mirror(&mut self, p: impl AsRef<Point>) {
        self._set_mirror(p.as_ref())
    }

    fn _set_mirror(&mut self, p: &Point) {
        unsafe {
            cpp!([self as "gp_Trsf*", p as "const gp_Pnt*"] {
                self->SetMirror(*p);
            })
        }
    }

    pub fn set_scale(&mut self, p: impl AsRef<Point>, s: f64) {
        self._set_scale(p.as_ref(), s)
    }

    fn _set_scale(&mut self, p: &Point, s: f64) {
        unsafe {
            cpp!([self as "gp_Trsf*", p as "const gp_Pnt*", s as "Standard_Real"] {
                self->SetScale(*p, s);
            })
        }
    }

    pub fn set_translation<T: AsRef<Vector>>(&mut self, v: &T) {
        self._set_translation(v.as_ref());
    }

    fn _set_translation(&mut self, v: &Vector) {
        unsafe {
            cpp!([self as "gp_Trsf*", v as "const gp_Vec*"] {
                self->SetTranslation(*v);
            })
        }
    }

    pub fn invert(&mut self) {
        unsafe {
            cpp!([self as "gp_Trsf*"] {
                self->Invert();
            })
        }
    }

    pub fn transforms<T: AsMut<Point>>(&self, p: &mut T) {
        self._transforms(p.as_mut());
    }

    fn _transforms(&self, p: &mut Point) {
        unsafe {
            cpp!([self as "gp_Trsf*", p as "gp_XYZ*"] {
                self->Transforms(*p);
            })
        }
    }
}
