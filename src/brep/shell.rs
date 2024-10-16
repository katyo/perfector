use super::{IsShape, Shape};
use core::mem::size_of;
use cpp::{cpp, cpp_class};
use static_assertions::const_assert_eq;

cpp! {{
    #include <memory>

    #include <gp_Pnt.hxx>

    #include <TopoDS_Shape.hxx>
    #include <TopoDS_Shell.hxx>

    //#include <BRepBuilderAPI_MakeShell.hxx>

    using namespace std;
}}

#[repr(transparent)]
pub struct Shell(ShellPtr);

cpp_class!(unsafe struct ShellPtr as "unique_ptr<TopoDS_Shell>");

const_assert_eq!(size_of::<Shell>(), size_of::<*const u8>());

shape_impls! {
    Shell;
}
