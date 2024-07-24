use crate::macros;

/// The magic number that appears at the beginning of classfiles.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Magic(u32);

macros::tuple_struct_impl!(Magic: u32);

impl Magic {
    /// The magic number that should appear at the beginning of all classfiles.
    pub const CAFEBABE: Self = Magic(0xCAFEBABE);

    /// Check if the value of this [Magic] number is [Magic::CAFEBABE] (what it should be on any valid classfile).
    pub const fn is_cafebabe(self) -> bool {
        self.get_inner() == Magic::CAFEBABE.get_inner()
    }
}
