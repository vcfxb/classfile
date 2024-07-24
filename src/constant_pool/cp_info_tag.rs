//! The tag field of the constant pool info structure.

use crate::macros;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Tag(u8);

macros::tuple_struct_impl!(Tag: u8);

impl Tag {
    /// Check if this is a valid [Tag] value.
    pub const fn is_valid(self) -> bool {
        match self.0 {
            1..=20 => true,
            _ => false
        }
    }
}
