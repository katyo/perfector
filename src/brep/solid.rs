use super::{IsShape, Shape};
use core::mem::size_of;
use cpp::{cpp, cpp_class};
use static_assertions::const_assert_eq;

cpp! {{
    #include <memory>

    #include <gp_Pnt.hxx>

    #include <TopoDS_Shape.hxx>
    #include <TopoDS_Solid.hxx>

    //#include <BRepBuilderAPI_MakeSolid.hxx>

    using namespace std;
}}

#[repr(transparent)]
pub struct Solid(SolidPtr);

cpp_class!(unsafe struct SolidPtr as "unique_ptr<TopoDS_Solid>");

const_assert_eq!(size_of::<Solid>(), size_of::<*const u8>());

shape_impls! {
    Solid;
}
