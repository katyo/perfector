use super::{Edge, Shape};
use core::mem::size_of;
use cpp::{cpp, cpp_class};
use static_assertions::const_assert_eq;

cpp! {{
    #include <memory>

    #include <gp_Pnt.hxx>

    #include <TopoDS_Shape.hxx>
    #include <TopoDS_Edge.hxx>
    #include <TopoDS_Wire.hxx>

    #include <BRepBuilderAPI_MakeWire.hxx>

    using namespace std;
}}

enum_impls! {
    /// Wire construction error
    WireError {
        /// No initialization of the algorithm. Only an empty constructor was used.
        EmptyWire = 1,
        /// The last edge which you attempted to add was not connected to the wire.
        DisconnectedWire,
        /// The wire with some singularity.
        NonManifoldWire,
    }
}

#[repr(transparent)]
pub struct Wire(WirePtr);

cpp_class!(unsafe struct WirePtr as "unique_ptr<TopoDS_Wire>");

const_assert_eq!(size_of::<Wire>(), size_of::<*const u8>());

shape_impls! {
    Wire;
}

impl core::ops::Deref for Wire {
    type Target = Shape;
    fn deref(&self) -> &Self::Target {
        unsafe {
            cpp!([self as "const TopoDS_Wire*"] -> &Shape as "const TopoDS_Shape*" {
                return self;
            })
        }
    }
}

impl core::ops::DerefMut for Wire {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe {
            cpp!([self as "TopoDS_Wire*"] -> &mut Shape as "TopoDS_Shape*" {
                return self;
            })
        }
    }
}

cpp_class!(unsafe struct MakeWire as "unique_ptr<BRepBuilderAPI_MakeWire>");

impl Wire {
    pub fn from_edges<T: AsRef<Edge>>(e: impl IntoIterator<Item = T>) -> Result<Self, WireError> {
        let mut b = unsafe {
            cpp!([] -> MakeWire as "unique_ptr<BRepBuilderAPI_MakeWire>" {
                return unique_ptr<BRepBuilderAPI_MakeWire>(new BRepBuilderAPI_MakeWire());
            })
        };

        let br = &mut b;

        for e in e {
            let e = e.as_ref();

            unsafe {
                cpp!([br as "unique_ptr<BRepBuilderAPI_MakeWire>*", e as "const unique_ptr<TopoDS_Edge>*"] {
                    //BRepTools::Dump(**e, cout);
                    (*br)->Add(**e);
                });
            }
        }

        let mut w = WirePtr::default();
        let r = &mut w;

        let rc = unsafe {
            cpp!([br as "unique_ptr<BRepBuilderAPI_MakeWire>*", r as "unique_ptr<TopoDS_Wire>*"] -> u32 as "BRepBuilderAPI_WireError" {
                auto rc = (*br)->Error();
                if (rc == BRepBuilderAPI_WireDone) {
                    *r = unique_ptr<TopoDS_Wire>(new TopoDS_Wire((*br)->Wire()));
                }
                return rc;
            })
        };

        if let Ok(err) = rc.try_into() {
            Err(err)
        } else {
            Ok(Self(w))
        }
    }
}
