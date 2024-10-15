macro_rules! shape_impls {
    ( $($Type:ident;)* ) => {
        $(
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
        )*
    };
}

mod edge;
mod face;
mod shape;
mod vertex;
mod wire;

pub use edge::*;
pub use face::*;
pub use shape::*;
pub use vertex::*;
pub use wire::*;

use super::math::*;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn vertex() {
        let v = Vertex::from(&[0.0, 0.0, 0.0]);

        assert_eq!(v.type_(), ShapeType::Vertex);

        //assert!(false);
    }

    #[test]
    fn edge() {
        let e1 = Edge::try_from((&[0.0, 0.0, 0.0], &[1.0, 0.0, 0.0]));
        let e2 = Edge::try_from((&[0.0, 0.0, 0.0], &[0.0, 0.0, 0.0]));

        let e1 = e1.unwrap();
        if let Err(e) = e2 {
            assert_eq!(e, EdgeError::LineThroughIdenticPoints);
        } else {
            panic!("Error expected");
        }

        assert_eq!(e1.type_(), ShapeType::Edge);

        //assert!(false);
    }

    #[test]
    fn wire() {
        let e1 = Edge::try_from((&[0.0, 0.0, 0.0], &[1.0, 0.0, 0.0])).unwrap();
        let e2 = Edge::try_from((&[1.0, 0.0, 0.0], &[0.0, 1.0, 0.0])).unwrap();
        let e3 =
            Edge::try_from((&[-1.0, 1.0, 0.0], &[1.0, -1.0, 0.0])).unwrap();

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

        assert_eq!(w1.type_(), ShapeType::Wire);
        assert_eq!(w2.type_(), ShapeType::Wire);
        //assert!(false);
    }
}
