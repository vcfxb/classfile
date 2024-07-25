//! Implementation relating to the version fields of a classfile.

use crate::macros;

/// A major version in a java class file.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MajorVersion(u16);

macros::tuple_struct_impl!(MajorVersion: u16);

impl MajorVersion {
    pub const JAVA_SE_1_0_2: MajorVersion = MajorVersion(45);
    pub const JAVA_SE_1_1: MajorVersion = MajorVersion(45);
    pub const JAVA_SE_1_2: MajorVersion = MajorVersion(46);
    pub const JAVA_SE_1_3: MajorVersion = MajorVersion(47);
    pub const JAVA_SE_1_4: MajorVersion = MajorVersion(48);
    pub const JAVA_SE_5_0: MajorVersion = MajorVersion(49);
    pub const JAVA_SE_6: MajorVersion = MajorVersion(50);
    pub const JAVA_SE_7: MajorVersion = MajorVersion(51);
    pub const JAVA_SE_8: MajorVersion = MajorVersion(52);
    pub const JAVA_SE_9: MajorVersion = MajorVersion(53);
    pub const JAVA_SE_10: MajorVersion = MajorVersion(54);
    pub const JAVA_SE_11: MajorVersion = MajorVersion(55);
    pub const JAVA_SE_12: MajorVersion = MajorVersion(56);
    pub const JAVA_SE_13: MajorVersion = MajorVersion(57);
    pub const JAVA_SE_14: MajorVersion = MajorVersion(58);
    pub const JAVA_SE_15: MajorVersion = MajorVersion(59);
    pub const JAVA_SE_16: MajorVersion = MajorVersion(60);
    pub const JAVA_SE_17: MajorVersion = MajorVersion(61);
    pub const JAVA_SE_18: MajorVersion = MajorVersion(62);
    pub const JAVA_SE_19: MajorVersion = MajorVersion(63);
    pub const JAVA_SE_20: MajorVersion = MajorVersion(64);
    pub const JAVA_SE_21: MajorVersion = MajorVersion(65);
    pub const JAVA_SE_22: MajorVersion = MajorVersion(66);

    /// Return `true` if this is a valid value for a classfile's major_version field.
    ///
    /// This is valid as of July 2024 (Java SE 22).
    pub const fn is_valid(self) -> bool {
        match self.0 {
            45..=66 => true,
            _ => false,
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
