use super::{Shape, Wire};
use core::mem::size_of;
use cpp::{cpp, cpp_class};
use static_assertions::const_assert_eq;

cpp! {{
    #include <memory>

    #include <gp_Pnt.hxx>

    #include <TopoDS_Shape.hxx>
    #include <TopoDS_Wire.hxx>
    #include <TopoDS_Face.hxx>

    #include <BRepBuilderAPI_MakeFace.hxx>

    using namespace std;
}}

enum_impls! {
    /// Face construction error
    FaceError {
        /// No initialization of the algorithm; only an empty constructor was used.
        NoFace = 1,
        /// No surface was given and the wire was not planar.
        NotPlanar,
        /// Not used so far.
        CurveProjectionFailed,
        /// The parameters given to limit the surface are out of its bounds.
        ParametersOutOfRange,
    }
}

#[repr(transparent)]
pub struct Face(FacePtr);

cpp_class!(unsafe struct FacePtr as "unique_ptr<TopoDS_Face>");

const_assert_eq!(size_of::<Face>(), size_of::<*const u8>());

shape_impls! {
    Face;
}

impl core::ops::Deref for Face {
    type Target = Shape;
    fn deref(&self) -> &Self::Target {
        unsafe {
            cpp!([self as "const TopoDS_Face*"] -> &Shape as "const TopoDS_Shape*" {
                return self;
            })
        }
    }
}

impl core::ops::DerefMut for Face {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe {
            cpp!([self as "TopoDS_Face*"] -> &mut Shape as "TopoDS_Shape*" {
                return self;
            })
        }
    }
}

cpp_class!(unsafe struct MakeFace as "unique_ptr<BRepBuilderAPI_MakeFace>");

impl Face {
    pub fn from_wires<T: AsRef<Wire>>(
        items: impl IntoIterator<Item = T>,
    ) -> Result<Self, FaceError> {
        let mut b = unsafe {
            cpp!([] -> MakeFace as "unique_ptr<BRepBuilderAPI_MakeFace>" {
                return unique_ptr<BRepBuilderAPI_MakeFace>(new BRepBuilderAPI_MakeFace());
            })
        };

        let br = &mut b;

        for item in items {
            let w = item.as_ref();

            unsafe {
                cpp!([br as "unique_ptr<BRepBuilderAPI_MakeFace>*", w as "const unique_ptr<TopoDS_Wire>*"] {
                    //BRepTools::Dump(**w, cout);
                    (*br)->Add(**w);
                });
            }
        }

        let mut f = FacePtr::default();
        let r = &mut f;

        let rc = unsafe {
            cpp!([br as "unique_ptr<BRepBuilderAPI_MakeFace>*", r as "unique_ptr<TopoDS_Face>*"] -> u32 as "BRepBuilderAPI_FaceError" {
                auto rc = (*br)->Error();
                if (rc == BRepBuilderAPI_FaceDone) {
                    *r = unique_ptr<TopoDS_Face>(new TopoDS_Face((*br)->Face()));
                }
                return rc;
            })
        };

        if let Ok(err) = rc.try_into() {
            Err(err)
        } else {
            Ok(Self(f))
        }
    }
}
