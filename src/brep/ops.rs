use super::{Shape, Vector, Vertex, Axis1, Angle, AngleUnits, Edge, Wire, Face, Shell, Solid, CompSolid};
use cpp::cpp;

cpp! {{
    #include <memory>

    #include <gp_Vec.hxx>

    #include <TopoDS_Shape.hxx>

    #include <BRepPrimAPI_MakePrism.hxx>
    #include <BRepPrimAPI_MakeRevol.hxx>

    using namespace std;
}}

#[derive(Clone, Copy, Debug)]
pub struct PrimError;

impl Shape {
    fn make_extrude(&self, vector: &Vector) -> Result<Shape, PrimError> {
        let mut shape = Shape::default();
        let r = &mut shape;
        let ok = unsafe {
            cpp!([self as "const unique_ptr<TopoDS_Shape>*", vector as "const gp_Vec*", r as "unique_ptr<TopoDS_Shape>*"] -> bool as "Standard_Boolean" {
                BRepPrimAPI_MakePrism b(**self, *vector);
                if (!b.IsDone()) {
                    return Standard_False;
                }
                *r = unique_ptr<TopoDS_Shape>(new TopoDS_Shape(b));
                return Standard_True;
            })
        };
        if ok {
            Ok(shape)
        } else {
            Err(PrimError)
        }
    }

    fn make_revolve(&self, axis: &Axis1, angle: &Angle) -> Result<Shape, PrimError> {
        let rad = *angle.to(AngleUnits::Rad).raw();
        let mut shape = Shape::default();
        let r = &mut shape;
        let ok = unsafe {
            cpp!([self as "const unique_ptr<TopoDS_Shape>*", axis as "const gp_Ax1*", rad as "Standard_Real", r as "unique_ptr<TopoDS_Shape>*"] -> bool as "Standard_Boolean" {
                BRepPrimAPI_MakeRevol b(**self, *axis, rad);
                if (!b.IsDone()) {
                    return Standard_False;
                }
                *r = unique_ptr<TopoDS_Shape>(new TopoDS_Shape(b));
                return Standard_True;
            })
        };
        if ok {
            Ok(shape)
        } else {
            Err(PrimError)
        }
    }
}

macro_rules! extrude_impls {
    ( $($type:ident => $rtype:ident,)* ) => {
        $(
            impl $type {
                pub fn extrude(&self, vector: impl AsRef<Vector>) -> Result<$rtype, PrimError> {
                    self.make_extrude(vector.as_ref())?.try_into().map_err(|_| PrimError)
                }

                pub fn revolve(&self, axis: impl AsRef<Axis1>, angle: impl AsRef<Angle>) -> Result<$rtype, PrimError> {
                    self.make_revolve(axis.as_ref(), angle.as_ref())?.try_into().map_err(|_| PrimError)
                }
            }
        )*
    };
}

extrude_impls! {
    Vertex => Edge,
    Edge => Face,
    Wire => Shell,
    Face => Solid,
    Shell => CompSolid,
}

#[cfg(test)]
mod test {
    use super::super::ShapeType;
    use super::*;

    #[test]
    fn extrude() {
        let v1 = Vertex::from(&[0.0, 0.0, 0.0]);
        let e1 = v1.extrude(&[0.0, 0.0, 1.0]).unwrap();
        let f1 = e1.extrude(&[0.0, 1.0, 0.0]).unwrap();
        let s1 = f1.extrude(&[1.0, 0.0, 0.0]).unwrap();

        assert_eq!(e1.type_().unwrap(), ShapeType::Edge);
        assert_eq!(f1.type_().unwrap(), ShapeType::Face);
        assert_eq!(s1.type_().unwrap(), ShapeType::Solid);
    }

    #[test]
    fn revolve() {
        let v1 = Vertex::from(&[0.0, 0.0, 1.0]);
        let x1: Axis1 = ([0.0, 0.0, 0.0], [0.0, 1.0, 0.0]).into();
        let a1 = Angle::new(90.0, AngleUnits::Deg);
        let x2: Axis1 = ([0.0, 0.0, 0.0], [1.0, 0.0, 0.0]).into();
        let a2 = Angle::new(360.0, AngleUnits::Deg);
        let e1 = v1.revolve(&x1, a1).unwrap();
        let f1 = e1.revolve(&x2, a2).unwrap();

        /*for v in e1.traverse::<Vertex>() {
            eprintln!("{:?}", v.point::<[f64; 3]>());
        }*/

        assert_eq!(e1.type_().unwrap(), ShapeType::Edge);
        assert_eq!(f1.type_().unwrap(), ShapeType::Face);
    }
}
