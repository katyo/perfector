macro_rules! enum_impls {
    ( $( $(#[$($TypeMeta:meta)*])* $Type:ident { $( $(#[$($VarMeta:meta)*])* $Var:ident $(= $Val:literal)*, )* } )* ) => {
        $(
            $(#[$($TypeMeta)*])*
            #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
            #[repr(u32)]
            pub enum $Type {
                $( $(#[$($VarMeta)*])* $Var $(= $Val)*, )*
            }

            impl AsRef<str> for $Type {
                fn as_ref(&self) -> &str {
                    match self {
                        $( Self::$Var => stringify!($Var), )*
                    }
                }
            }

            impl core::fmt::Display for $Type {
                fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                    self.as_ref().fmt(f)
                }
            }

            impl core::str::FromStr for $Type {
                type Err = ();
                fn from_str(s: &str) -> Result<Self, Self::Err> {
                    Ok(match s {
                        $( stringify!($Var) => Self::$Var, )*
                        _ => return Err(()),
                    })
                }
            }

            impl TryFrom<u32> for $Type {
                type Error = ();
                fn try_from(rc: u32) -> Result<Self, Self::Error> {
                    if rc < enum_impls!(@firstvar $($Var),*) as _ {
                        return Err(());
                    }
                    if rc > enum_impls!(@lastvar $($Var),*) as _ {
                        panic!("Unknown error code {} for {}", rc, stringify!($Type));
                    }
                    Ok(unsafe { *(&rc as *const _ as *const _) })
                }
            }
        )*
    };

    (@firstvar $Var:ident $(, $Vars:ident)*) => { Self::$Var };
    (@lastvar $Var:ident) => { Self::$Var };
    (@lastvar $Var:ident $(, $Vars:ident)*) => { enum_impls!(@lastvar $($Vars),*) };
}

mod brep;
mod math;

pub use brep::*;
pub use math::*;

use core::{marker::PhantomData, mem::ManuallyDrop};

pub struct Ref<'r, T> {
    inner: ManuallyDrop<T>,
    _phantom: PhantomData<&'r ()>,
}

impl<'r, T> Ref<'r, T> {
    pub(crate) fn from_raw(raw: T) -> Self {
        Self {
            inner: ManuallyDrop::new(raw),
            _phantom: PhantomData,
        }
    }
}

impl<'r, T> core::ops::Deref for Ref<'r, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &*self.inner
    }
}

impl<'r, T> AsRef<T> for Ref<'r, T> {
    fn as_ref(&self) -> &T {
        &*self.inner
    }
}
