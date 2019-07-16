use std::ops::{DerefMut, Deref};
use std::hash::{self, Hash};
use std::cmp::Ordering;
use std::fmt;


macro_rules! doc_comment {
    ($x:expr, $($tt:tt)*) => {
        #[doc = $x]
        $($tt)*
    };
}

macro_rules! wrapper {
    ($name:ident, $size:expr, $size_s:tt, $comm:tt) => {
        doc_comment! {
            concat!("Mostly-transparent wrapper for a heap-allocated ", $size_s, " `u8` array", $comm),
            #[derive(Clone)]
            #[repr(transparent)]
            pub struct $name(Box<[u8; $size]>);
        }

        impl $name {
            pub fn new() -> $name {
                $name(Box::new([0; $size]))
            }
        }

        impl Default for $name {
            fn default() -> $name {
                $name::new()
            }
        }

        impl Deref for $name {
            type Target = [u8; $size];

            #[inline]
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl DerefMut for $name {
            #[inline]
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }

        impl fmt::Debug for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.debug_list().entries(self.iter()).finish()
            }
        }

        impl Hash for $name {
            fn hash<H: hash::Hasher>(&self, state: &mut H) {
                Hash::hash(&self[..], state)
            }
        }

        impl PartialEq<[u8]> for $name {
            #[inline]
            fn eq(&self, other: &[u8]) -> bool {
                self[..] == other[..]
            }

            #[inline]
            fn ne(&self, other: &[u8]) -> bool {
                self[..] != other[..]
            }
        }

        impl PartialEq<$name> for $name {
            #[inline]
            fn eq(&self, other: &$name) -> bool {
                self[..] == other[..]
            }

            #[inline]
            fn ne(&self, other: &$name) -> bool {
                self[..] != other[..]
            }
        }

        impl Eq for $name {}

        impl PartialOrd for $name {
            #[inline]
            fn partial_cmp(&self, other: &$name) -> Option<Ordering> {
                PartialOrd::partial_cmp(&&self[..], &&other[..])
            }

            #[inline]
            fn lt(&self, other: &$name) -> bool {
                PartialOrd::lt(&&self[..], &&other[..])
            }

            #[inline]
            fn le(&self, other: &$name) -> bool {
                PartialOrd::le(&&self[..], &&other[..])
            }

            #[inline]
            fn ge(&self, other: &$name) -> bool {
                PartialOrd::ge(&&self[..], &&other[..])
            }

            #[inline]
            fn gt(&self, other: &$name) -> bool {
                PartialOrd::gt(&&self[..], &&other[..])
            }
        }

        impl Ord for $name {
            #[inline]
            fn cmp(&self, other: &$name) -> Ordering {
                Ord::cmp(&&self[..], &&other[..])
            }
        }
    };
}


wrapper!(Memory, 0xFFFF + 1, "64KiB", "");
wrapper!(Ports,  0xFF   + 1, "256B",  " for I/O ports");
