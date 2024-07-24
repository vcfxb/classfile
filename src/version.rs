//! Implementation relating to the version fields of a classfile.

use crate::macros;

/// A major version in a java class file.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MajorVersion(u16);

macros::tuple_struct_impl!(MajorVersion: u16);

impl MajorVersion {
    /// The lowest valid [MajorVersion].
    pub const MIN: Self = MajorVersion(45);

    /// The highest valid [MajorVersion] at time of writing (Java SE 22).
    pub const MAX: Self = MajorVersion(66);

    /// Return `true` if this is a valid value for a classfile's major_version field.
    /// 
    /// This is valid as of July 2024 (Java SE 22).
    pub const fn is_valid(self) -> bool {
        match self.0 {
            45..=66 => true,
            _ => false
        }
    }
}

/// A minor version in a java class file.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MinorVersion(u16);

macros::tuple_struct_impl!(MinorVersion: u16);

impl MinorVersion {
    /// Check if this [MinorVersion] is valid for the given [MajorVersion]. 
    /// Will return false if the [MajorVersion] is not valid.
    pub const fn is_valid_for_major(self, major: MajorVersion) -> bool {
        match (major.0, self.0) {
            (45..=55, _) => true,
            (56..=66, 0 | 65535) => true,
            _ => false,
        }
    }
}
