use super::{IsShape, Shape};
use core::mem::size_of;
use cpp::{cpp, cpp_class};
use static_assertions::const_assert_eq;

cpp! {{
    #include <memory>

    #include <gp_Pnt.hxx>

    #include <TopoDS_Shape.hxx>
    #include <TopoDS_CompSolid.hxx>

    using namespace std;
}}

#[repr(transparent)]
pub struct CompSolid(CompSolidPtr);

cpp_class!(unsafe struct CompSolidPtr as "unique_ptr<TopoDS_CompSolid>");

const_assert_eq!(size_of::<CompSolid>(), size_of::<*const u8>());

shape_impls! {
    CompSolid;
}
