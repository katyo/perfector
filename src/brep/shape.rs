use crate::{Ref, Transformation};
use core::{marker::PhantomData, mem::size_of};
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

/// The marker trait for BRep topoloty shapes
pub trait IsShape {
    const TYPE: ShapeType;
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

    /// The orinetation of shape
    Orientation {
        /// Left-to-right edge direction
        Forward,
        /// Right-to-left edge direction
        Reversed,
        /// Internal volume of solid
        Internal,
        /// External volume of solid
        External,
    }
}

impl PartialEq for Shape {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            cpp!([self as "const unique_ptr<TopoDS_Shape>*", other as "const unique_ptr<TopoDS_Shape>*"] -> bool as "Standard_Boolean" {
                return (**self) == (**other);
            })
        }
    }
}

impl Shape {
    pub fn is_null(&self) -> bool {
        unsafe {
            cpp!([self as "const unique_ptr<TopoDS_Shape>*"] -> bool as "Standard_Boolean" {
                return (*self)->IsNull();
            })
        }
    }

    /// Get the type of shape
    pub fn type_(&self) -> Option<ShapeType> {
        if self.is_null() {
            None
        } else {
            Some(unsafe {
                cpp!([self as "const unique_ptr<TopoDS_Shape>*"] -> ShapeType as "TopAbs_ShapeEnum" {
                    return (*self)->ShapeType();
                })
            })
        }
    }

    pub fn traverse<'i, T: IsShape + TryFrom<Shape>>(&'i self) -> ShapeIter<'i, T> {
        ShapeIter::new(self)
    }

    pub fn orientation(&self) -> Orientation {
        unsafe {
            cpp!([self as "const unique_ptr<TopoDS_Shape>*"] -> Orientation as "TopAbs_Orientation" {
                return (*self)->Orientation();
            })
        }
    }

    pub fn set_orientation(&mut self, o: Orientation) {
        unsafe {
            cpp!([self as "unique_ptr<TopoDS_Shape>*", o as "TopAbs_Orientation"] {
                (*self)->Orientation(o);
            })
        }
    }

    pub fn transformation(&self) -> &Transformation {
        unsafe {
            cpp!([self as "const unique_ptr<TopoDS_Shape>*"] -> &Transformation as "const gp_Trsf*" {
                return &(*self)->Location().Transformation();
            })
        }
    }

    pub fn set_transformation(&mut self, t: &Transformation) {
        unsafe {
            cpp!([self as "unique_ptr<TopoDS_Shape>*", t as "const gp_Trsf*"] {
                TopLoc_Location loc(*t);
                (*self)->Location(loc, false);
            })
        }
    }
}

pub struct ShapeIter<'i, S> {
    indexed_map: IndexedMapOfShape,
    iterator: IndexedMapOfShapeIterator,
    _phantom: PhantomData<&'i S>,
}

impl<'i, T: IsShape + TryFrom<Shape> + 'i> ShapeIter<'i, T> {
    fn new(shape: &'i Shape) -> Self {
        let type_ = T::TYPE;
        let indexed_map = IndexedMapOfShape::new(shape, type_);
        let iterator = indexed_map.begin();
        Self {
            indexed_map,
            iterator,
            _phantom: PhantomData,
        }
    }
}

impl<'i, T: IsShape + TryFrom<Shape> + 'i> Iterator for ShapeIter<'i, T> {
    type Item = Ref<'i, T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.iterator != self.indexed_map.end() {
            let shape = self.iterator.key();
            self.iterator.increment();
            if let Ok(shape) = T::try_from(shape) {
                Some(Ref::from_raw(shape))
            } else {
                None
            }
        } else {
            None
        }
    }
}

impl<'i, T: IsShape + TryFrom<Shape> + 'i> ExactSizeIterator for ShapeIter<'i, T> {
    fn len(&self) -> usize {
        self.indexed_map.len()
    }
}

cpp_class!(unsafe struct IndexedMapOfShape as "unique_ptr<TopTools_IndexedMapOfShape>");
cpp_class!(unsafe struct IndexedMapOfShapeIterator as "unique_ptr<TopTools_IndexedMapOfShape::const_iterator>");

impl PartialEq for IndexedMapOfShapeIterator {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            cpp!([self as "const unique_ptr<TopTools_IndexedMapOfShape::const_iterator>*", other as "const unique_ptr<TopTools_IndexedMapOfShape::const_iterator>*"] -> bool as "Standard_Boolean" {
                return **self == **other;
            })
        }
    }
}

impl IndexedMapOfShapeIterator {
    fn key(&self) -> Shape {
        let shape = unsafe {
            cpp!([self as "const unique_ptr<TopTools_IndexedMapOfShape::const_iterator>*"] -> ShapePtr as "const TopoDS_Shape*" {
                return &***self;
            })
        };
        Shape(shape)
    }

    fn increment(&mut self) {
        unsafe {
            cpp!([self as "unique_ptr<TopTools_IndexedMapOfShape::const_iterator>*"] {
                (**self) ++;
            })
        }
    }
}

impl IndexedMapOfShape {
    fn new(shape: &Shape, type_: ShapeType) -> Self {
        unsafe {
            cpp!([shape as "const unique_ptr<TopoDS_Shape>*", type_ as "TopAbs_ShapeEnum"] -> IndexedMapOfShape as "unique_ptr<TopTools_IndexedMapOfShape>" {
                auto indexed_map = unique_ptr<TopTools_IndexedMapOfShape>(new TopTools_IndexedMapOfShape());
                TopExp::MapShapes(**shape, type_, *indexed_map);
                return indexed_map;
            })
        }
    }

    fn begin(&self) -> IndexedMapOfShapeIterator {
        unsafe {
            cpp!([self as "const unique_ptr<TopTools_IndexedMapOfShape>*"] -> IndexedMapOfShapeIterator as "unique_ptr<TopTools_IndexedMapOfShape::const_iterator>" {
                return unique_ptr<TopTools_IndexedMapOfShape::const_iterator>(new TopTools_IndexedMapOfShape::const_iterator((*self)->cbegin()));
            })
        }
    }

    fn end(&self) -> IndexedMapOfShapeIterator {
        unsafe {
            cpp!([self as "const unique_ptr<TopTools_IndexedMapOfShape>*"] -> IndexedMapOfShapeIterator as "unique_ptr<TopTools_IndexedMapOfShape::const_iterator>" {
                return unique_ptr<TopTools_IndexedMapOfShape::const_iterator>(new TopTools_IndexedMapOfShape::const_iterator((*self)->cend()));
            })
        }
    }

    fn len(&self) -> usize {
        (unsafe {
            cpp!([self as "const unique_ptr<TopTools_IndexedMapOfShape>*"] -> i32 as "Standard_Integer" {
                auto len = (*self)->Size();
                cout << "len: " << len << endl;
                return len;
            })
        }) as _
    }
}
