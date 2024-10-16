use super::{IsShape, Point, Shape};
use core::mem::size_of;
use cpp::{cpp, cpp_class};
use static_assertions::const_assert_eq;

cpp! {{
    #include <memory>

    #include <gp_Pnt.hxx>

    #include <BRep_Tool.hxx>

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

impl<P: AsRef<Point>> From<P> for Vertex {
    fn from(p: P) -> Self {
        Self::from_point(p.as_ref())
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

    fn get_point(&self) -> Point {
        unsafe {
            cpp!([self as "const unique_ptr<TopoDS_Vertex>*"] -> Point as "gp_Pnt" {
                return BRep_Tool::Pnt(**self);
            })
        }
    }

    /// Get coords of vertex
    pub fn point<T: From<Point>>(&self) -> T {
        self.get_point().into()
    }
}
