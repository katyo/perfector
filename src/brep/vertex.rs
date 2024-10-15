use super::{IsPoint, Point, Shape};
use core::mem::size_of;
use cpp::{cpp, cpp_class};
use static_assertions::const_assert_eq;

cpp! {{
    #include <memory>

    #include <gp_Pnt.hxx>

    #include <TopoDS_Shape.hxx>
    #include <TopoDS_Vertex.hxx>

    #include <BRepBuilderAPI_MakeVertex.hxx>

    using namespace std;
}}

#[repr(transparent)]
pub struct Vertex(VertexPtr);

cpp_class!(unsafe struct VertexPtr as "unique_ptr<TopoDS_Vertex>");

const_assert_eq!(size_of::<Vertex>(), size_of::<*const u8>());

shape_impls! {
    Vertex;
}

impl Default for Vertex {
    fn default() -> Self {
        Self::from_point(&Point::default())
    }
}

impl<P: IsPoint> From<&P> for Vertex {
    fn from(p: &P) -> Self {
        Self::from_point(unsafe { &*(p as *const _ as *const _) })
    }
}

impl Vertex {
    fn from_point(p: &Point) -> Self {
        let v = unsafe {
            cpp!([p as "const gp_Pnt*"] -> VertexPtr as "unique_ptr<TopoDS_Vertex>" {
                return unique_ptr<TopoDS_Vertex>(new TopoDS_Vertex(BRepBuilderAPI_MakeVertex(*p)));
            })
        };
        Self(v)
    }
}

impl core::ops::Deref for Vertex {
    type Target = Shape;
    fn deref(&self) -> &Self::Target {
        unsafe {
            cpp!([self as "const TopoDS_Vertex*"] -> &Shape as "const TopoDS_Shape*" {
                return self;
            })
        }
    }
}

impl core::ops::DerefMut for Vertex {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe {
            cpp!([self as "TopoDS_Vertex*"] -> &mut Shape as "TopoDS_Shape*" {
                return self;
            })
        }
    }
}
