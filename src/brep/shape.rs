use core::mem::size_of;
use cpp::{cpp, cpp_class};
use static_assertions::const_assert_eq;

cpp! {{
    #include <memory>

    #include <TopoDS_Shape.hxx>

    #include <TopExp.hxx>
    #include <TopTools_ShapeMapHasher.hxx>
    #include <NCollection_IndexedMap.hxx>

    #include <BRepTools.hxx>

    using namespace std;
}}

#[repr(transparent)]
pub struct Shape(ShapePtr);

cpp_class!(unsafe struct ShapePtr as "unique_ptr<TopoDS_Shape>");

const_assert_eq!(size_of::<Shape>(), size_of::<*const u8>());

impl Default for Shape {
    fn default() -> Self {
        Self(unsafe {
            cpp!([] -> ShapePtr as "unique_ptr<TopoDS_Shape>" {
                return unique_ptr<TopoDS_Shape>(new TopoDS_Shape());
            })
        })
    }
}

impl AsRef<Shape> for Shape {
    fn as_ref(&self) -> &Shape {
        self
    }
}

impl AsMut<Shape> for Shape {
    fn as_mut(&mut self) -> &mut Shape {
        &mut *self
    }
}

enum_impls! {
    /// The type of shape
    ShapeType {
        /// A group of any of the shapes below.
        Compound,
        /// A set of solids connected by their faces. This expands the notions of WIRE and SHELL to solids.
        Compsolid,
        /// A part of 3D space bounded by shells.
        Solid,
        /// A set of faces connected by some of the edges of their wire boundaries. A shell can be open or closed.
        Shell,
        /// Part of a plane (in 2D geometry) or a surface (in 3D geometry) bounded by a closed wire. Its geometry is constrained (trimmed) by contours.
        Face,
        /// A sequence of edges connected by their vertices. It can be open or closed depending on whether the edges are linked or not.
        Wire,
        /// A single dimensional shape corresponding to a curve, and bound by a vertex at each extremity.
        Edge,
        /// A zero-dimensional shape corresponding to a point in geometry.
        Vertex,
    }
}

impl Shape {
    pub fn type_(&self) -> ShapeType {
        unsafe {
            cpp!([self as "const unique_ptr<TopoDS_Shape>*"] -> ShapeType as "TopAbs_ShapeEnum" {
                return (*self)->ShapeType();
            })
        }
    }
}
