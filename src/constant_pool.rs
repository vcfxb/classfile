//! Types and implementation relating to the constant pool.

use std::io::{self, Read, Write};
use cp_info_tag::Tag;

pub mod cp_info_tag;

#[derive(Debug)]
pub struct ConstPoolInfo {
    pub tag: Tag,
    pub info: Vec<u8>
}

impl ConstPoolInfo {
    pub fn read<R: Read>(r: &mut R) -> io::Result<Self> {
        let tag = Tag::read(r)?;

        unimplemented!()
    }

    pub fn write<W: Write>(&self, w: &mut W) -> io::Result<()> {
        self.tag.write(w)?;

        unimplemented!()
    }
}
