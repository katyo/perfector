use super::{IsShape, Point, Shape, Vertex};
use core::mem::size_of;
use cpp::{cpp, cpp_class};
use static_assertions::const_assert_eq;

cpp! {{
    #include <memory>

    #include <gp_Pnt.hxx>

    #include <TopoDS_Shape.hxx>
    #include <TopoDS_Vertex.hxx>
    #include <TopoDS_Edge.hxx>

    #include <BRepBuilderAPI_MakeEdge.hxx>

    using namespace std;
}}

enum_impls! {
    /// Edge construction error
    EdgeError {
        /// No parameters were given but the projection of the 3D points on the curve failed. This happens when the point distance to the curve is greater than the precision value.
        PointProjectionFailed = 1,
        /// The given parameters are not in the parametric range C->FirstParameter(), C->LastParameter()
        ParameterOutOfRange,
        /// The two vertices or points are the extremities of a closed curve but have different locations.
        DifferentPointsOnClosedCurve,
        /// A finite coordinate point was associated with an infinite parameter (see the Precision package for a definition of infinite values).
        PointWithInfiniteParameter,
        /// The distance between the 3D point and the point evaluated on the curve with the parameter is greater than the precision.
        DifferentsPointAndParameter,
        /// Two identical points were given to define a line (construction of an edge without curve); gp::Resolution is used for the confusion test.
        LineThroughIdenticPoints,
    }
}

#[repr(transparent)]
pub struct Edge(EdgePtr);

cpp_class!(unsafe struct EdgePtr as "unique_ptr<TopoDS_Edge>");

const_assert_eq!(size_of::<Edge>(), size_of::<*const u8>());

shape_impls! {
    Edge;
}

impl<A: AsRef<Point>, B: AsRef<Point>> TryFrom<(A, B)> for Edge {
    type Error = EdgeError;
    fn try_from((p1, p2): (A, B)) -> Result<Self, Self::Error> {
        Self::line_from_points(p1.as_ref(), p2.as_ref())
    }
}

impl Edge {
    fn line_from_points(p1: &Point, p2: &Point) -> Result<Self, EdgeError> {
        let mut e = EdgePtr::default();
        let r = &mut e;
        let rc = unsafe {
            cpp!([p1 as "const gp_Pnt*", p2 as "const gp_Pnt*", r as "unique_ptr<TopoDS_Edge>*"] -> u32 as "BRepBuilderAPI_EdgeError" {
                BRepBuilderAPI_MakeEdge b(*p1, *p2);
                auto rc = b.Error();
                if (rc == BRepBuilderAPI_EdgeDone) {
                    *r = unique_ptr<TopoDS_Edge>(new TopoDS_Edge(b));
                }
                return rc;
            })
        };
        if let Ok(err) = rc.try_into() {
            Err(err)
        } else {
            Ok(Self(e))
        }
    }
}

impl TryFrom<(&Vertex, &Vertex)> for Edge {
    type Error = EdgeError;
    fn try_from((p1, p2): (&Vertex, &Vertex)) -> Result<Self, Self::Error> {
        let mut e = EdgePtr::default();
        let r = &mut e;
        let rc = unsafe {
            cpp!([p1 as "const unique_ptr<TopoDS_Vertex>*", p2 as "const unique_ptr<TopoDS_Vertex>*", r as "unique_ptr<TopoDS_Edge>*"] -> u32 as "BRepBuilderAPI_EdgeError" {
                auto b = BRepBuilderAPI_MakeEdge(**p1, **p2);
                auto rc = b.Error();
                if (rc == BRepBuilderAPI_EdgeDone) {
                    *r = unique_ptr<TopoDS_Edge>(new TopoDS_Edge(b));
                }
                return rc;
            })
        };
        if let Ok(err) = rc.try_into() {
            Err(err)
        } else {
            Ok(Self(e))
        }
    }
}
