macro_rules! shape_impls {
    ( $($Type:ident;)* ) => {
        $(
            impl IsShape for $Type {
                const TYPE: crate::ShapeType = crate::ShapeType::$Type;
            }

            impl AsRef<$Type> for $Type {
                fn as_ref(&self) -> &Self {
                    self
                }
            }

            impl AsMut<$Type> for $Type {
                fn as_mut(&mut self) -> &mut Self {
                    self
                }
            }

            impl AsRef<Shape> for $Type {
                fn as_ref(&self) -> &Shape {
                    &*self
                }
            }

            impl AsMut<Shape> for $Type {
                fn as_mut(&mut self) -> &mut Shape {
                    &mut *self
                }
            }

            impl core::ops::Deref for $Type {
                type Target = Shape;
                fn deref(&self) -> &Self::Target {
                    unsafe { &*(self as *const _ as *const _) }
                }
            }

            impl core::ops::DerefMut for $Type {
                fn deref_mut(&mut self) -> &mut Self::Target {
                    unsafe { &mut *(self as *mut _ as *mut _) }
                }
            }

            impl From<$Type> for Shape {
                fn from(shape: $Type) -> Self {
                    unsafe { core::mem::transmute(shape) }
                }
            }

            impl TryFrom<Shape> for $Type {
                type Error = ();
                fn try_from(shape: Shape) -> Result<Self, Self::Error> {
                    if shape.type_().map(|type_| type_ == Self::TYPE).unwrap_or(false) {
                        Ok(unsafe { core::mem::transmute(shape) })
                    } else {
                        Err(())
                    }
                }
            }
        )*
    };
}

mod edge;
mod face;
mod shape;
mod shell;
mod solid;
mod vertex;
mod wire;

pub use edge::*;
pub use face::*;
pub use shape::*;
pub use shell::*;
pub use solid::*;
pub use vertex::*;
pub use wire::*;

use super::math::*;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn vertex() {
        let p = [0.0, 0.0, 0.0];
        let v = Vertex::from(&p);

        assert_eq!(v.type_().unwrap(), ShapeType::Vertex);
        assert_eq!(v.point::<[f64; 3]>(), p);

        //assert!(false);
    }

    #[test]
    fn edge() {
        let p1 = [0.0, 0.0, 0.0];
        let p2 = [1.0, 0.0, 0.0];

        let e1 = Edge::try_from((&p1, &p2));
        let e2 = Edge::try_from((&p1, &p1));

        let e1 = e1.unwrap();
        if let Err(e) = e2 {
            assert_eq!(e, EdgeError::LineThroughIdenticPoints);
        } else {
            panic!("Error expected");
        }

        assert_eq!(e1.type_().unwrap(), ShapeType::Edge);
        assert_eq!(e1.orientation(), Orientation::Forward);

        let mut t1 = e1.traverse::<Vertex>();
        assert_eq!(t1.next().unwrap().point::<[f64; 3]>(), p1);
        assert_eq!(t1.next().unwrap().point::<[f64; 3]>(), p2);
        assert!(t1.next().is_none());

        //assert!(false);
    }

    #[test]
    fn wire() {
        let p1 = [0.0, 0.0, 0.0];
        let p2 = [1.0, 0.0, 0.0];
        let p3 = [0.0, 1.0, 0.0];
        let p4 = [-1.0, 1.0, 0.0];
        let p5 = [1.0, -1.0, 0.0];

        let e1 = Edge::try_from((&p1, &p2)).unwrap();
        let e2 = Edge::try_from((&p2, &p3)).unwrap();
        let e3 = Edge::try_from((&p4, &p5)).unwrap();

        let w1 = Wire::from_edges([&e1]);
        let w2 = Wire::from_edges([&e1, &e2]);
        let w3 = Wire::from_edges([] as [Edge; 0]);
        let w4 = Wire::from_edges([&e1, &e3]);

        let w1 = w1.unwrap();
        let w2 = w2.unwrap();
        if let Err(e) = w3 {
            assert_eq!(e, WireError::EmptyWire);
        } else {
            panic!("Error expected");
        }
        if let Err(e) = w4 {
            assert_eq!(e, WireError::DisconnectedWire);
        } else {
            panic!("Error expected");
        }

        assert_eq!(w1.type_().unwrap(), ShapeType::Wire);
        assert_eq!(w1.orientation(), Orientation::Forward);
        assert_eq!(w2.type_().unwrap(), ShapeType::Wire);

        let mut t1 = w1.traverse::<Edge>();
        let e1 = t1.next().unwrap();
        assert!(t1.next().is_none());
        let mut t1 = e1.traverse::<Vertex>();
        assert_eq!(t1.next().unwrap().point::<[f64; 3]>(), p1);
        assert_eq!(t1.next().unwrap().point::<[f64; 3]>(), p2);
        assert!(t1.next().is_none());

        //assert!(false);
    }
}
