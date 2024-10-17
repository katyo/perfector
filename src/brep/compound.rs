use super::{IsShape, Shape};
use core::mem::size_of;
use cpp::{cpp, cpp_class};
use static_assertions::const_assert_eq;

cpp! {{
    #include <memory>

    #include <gp_Pnt.hxx>

    #include <TopoDS_Shape.hxx>
    #include <TopoDS_Compound.hxx>

    using namespace std;
}}

#[repr(transparent)]
pub struct Compound(CompoundPtr);

cpp_class!(unsafe struct CompoundPtr as "unique_ptr<TopoDS_Compound>");

const_assert_eq!(size_of::<Compound>(), size_of::<*const u8>());

shape_impls! {
    Compound;
}
