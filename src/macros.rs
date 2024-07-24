//! Private macros that make implementing the classfile standard easier.

macro_rules! tuple_struct_impl_read {
    {
        $(#[$($attrss:tt)*])*
        read $inner:ty
    } => {
        $(#[$($attrss)*])*
        pub fn read<R: std::io::Read>(r: &mut R) -> std::io::Result<Self> {
            let mut buf = [0; std::mem::size_of::<$inner>()];
            r.read_exact(&mut buf)?;
            Ok(Self(<$inner>::from_be_bytes(buf)))
        }
    };
}

macro_rules! tuple_struct_impl_write {
    {
        $(#[$($attrss:tt)*])*
        write $inner:ty
    } => {
        $(#[$($attrss)*])*
        pub fn write<W: std::io::Write>(self, w: &mut W) -> std::io::Result<()> {
            let mut buf = self.0.to_be_bytes();
            w.write_all(&mut buf)
        }
    };
}

macro_rules! tuple_struct_impl_inner {
    {
        $(#[$($attrss:tt)*])*
        inner $inner:ty
    } => {
        $(#[$($attrss)*])*
        pub const fn get_inner(self) -> $inner {
            self.0
        }
    };
}

macro_rules! tuple_struct_all {
    ($inner:ty) => {
        $crate::macros::tuple_struct_impl_inner! { inner $inner }
        $crate::macros::tuple_struct_impl_read! { read $inner }
        $crate::macros::tuple_struct_impl_write! { write $inner }
    };
}

macro_rules! tuple_struct_impl {
    ($name:ident: $inner:ty) => {
        impl $name {
            crate::macros::tuple_struct_all!($inner);
        }
    };
}

pub(crate) use tuple_struct_impl_read;
pub(crate) use tuple_struct_impl_write;
pub(crate) use tuple_struct_impl_inner;
pub(crate) use tuple_struct_all;
pub(crate) use tuple_struct_impl;
