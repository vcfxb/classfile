//! The tag field of the constant pool info structure.

use crate::{
    macros,
    version::{MajorVersion, MinorVersion},
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Tag(u8);

macros::tuple_struct_impl!(Tag: u8);

impl Tag {
    pub const CONSTANT_UTF8: Tag = Tag(1);
    pub const CONSTANT_INTEGER: Tag = Tag(3);
    pub const CONSTANT_FLOAT: Tag = Tag(4);
    pub const CONSTANT_LONG: Tag = Tag(5);
    pub const CONSTANT_DOUBLE: Tag = Tag(6);
    pub const CONSTANT_CLASS: Tag = Tag(7);
    pub const CONSTANT_STRING: Tag = Tag(8);
    pub const CONSTANT_FIELD_REF: Tag = Tag(9);
    pub const CONSTANT_METHOD_REF: Tag = Tag(10);
    pub const CONSTANT_INTERFACE_METHOD_REF: Tag = Tag(11);
    pub const CONSTANT_NAME_AND_TYPE: Tag = Tag(12);
    pub const CONSTANT_METHOD_HANDLE: Tag = Tag(15);
    pub const CONSTANT_METHOD_TYPE: Tag = Tag(16);
    pub const CONSTANT_DYNAMIC: Tag = Tag(17);
    pub const CONSTANT_INVOKE_DYNAMIC: Tag = Tag(18);
    pub const CONSTANT_MODULE: Tag = Tag(19);
    pub const CONSTANT_PACKAGE: Tag = Tag(20);

    pub const ALL_TAGS: &'static [Tag] = &[
        Tag::CONSTANT_UTF8,
        Tag::CONSTANT_INTEGER,
        Tag::CONSTANT_FLOAT,
        Tag::CONSTANT_LONG,
        Tag::CONSTANT_DOUBLE,
        Tag::CONSTANT_CLASS,
        Tag::CONSTANT_STRING,
        Tag::CONSTANT_FIELD_REF,
        Tag::CONSTANT_METHOD_REF,
        Tag::CONSTANT_INTERFACE_METHOD_REF,
        Tag::CONSTANT_NAME_AND_TYPE,
        Tag::CONSTANT_METHOD_HANDLE,
        Tag::CONSTANT_METHOD_TYPE,
        Tag::CONSTANT_DYNAMIC,
        Tag::CONSTANT_INVOKE_DYNAMIC,
        Tag::CONSTANT_MODULE,
        Tag::CONSTANT_PACKAGE,
    ];

    /// Check if this [Tag] is valid for any version of Java.
    pub const fn is_valid(self) -> bool {
        match self.get_inner() {
            1 | 3..=12 | 15..=20 => true,
            _ => false,
        }
    }

    /// # Panics
    /// - If this [Tag] is not a valid tag in [Tag::ALL_TAGS].
    pub const fn valid_since_version(self) -> MajorVersion {
        match self.get_inner() {
            1 | 3..=12 => MajorVersion::JAVA_SE_1_0_2,
            15 | 16 | 18 => MajorVersion::JAVA_SE_7,
            17 => MajorVersion::JAVA_SE_11,
            19 | 20 => MajorVersion::JAVA_SE_9,
            _ => panic!("Invalid Tag"),
        }
    }
}
