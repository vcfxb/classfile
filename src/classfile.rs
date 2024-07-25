use crate::{
    constant_pool::ConstPoolInfo,
    magic::Magic,
    version::{MajorVersion, MinorVersion},
};
use std::io::{self, Read, Write};

use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};

/// Classfile structure as specified at <https://docs.oracle.com/javase/specs/jvms/se22/html/jvms-4.html>.
#[derive(Debug)]
pub struct ClassFile {
    pub magic: Magic,
    pub minor_version: MinorVersion,
    pub major_version: MajorVersion,
    pub constant_pool_count: u16,
    pub constant_pool: Vec<ConstPoolInfo>,
}

impl ClassFile {
    /// Read a [ClassFile] from the given [Read]er.
    pub fn read<R: Read>(reader: &mut R) -> io::Result<Self> {
        let magic = Magic::read(reader)?;
        let minor_version = MinorVersion::read(reader)?;
        let major_version = MajorVersion::read(reader)?;
        let constant_pool_count = reader.read_u16::<BigEndian>()?;

        // Read the constant pool
        let mut constant_pool = Vec::with_capacity(constant_pool_count as usize - 1);

        // Note that the constant pool is 1-indexed for some reason.
        for _ in 1..=constant_pool_count - 1 {
            let item = ConstPoolInfo::read(reader)?;
            constant_pool.push(item);
        }

        Ok(ClassFile {
            magic,
            minor_version,
            major_version,
            constant_pool_count,
            constant_pool,
        })
    }

    /// Write this [ClassFile] to a writer.
    ///
    /// This will not call [Write::flush] -- that is left to the caller.
    pub fn write<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        // Write magic.
        self.magic.write(writer)?;

        // Write versions.
        self.minor_version.write(writer)?;
        self.major_version.write(writer)?;

        // Write constant pool.
        writer.write_u16::<BigEndian>(self.constant_pool_count)?;

        for constant_info in self.constant_pool.iter() {
            constant_info.write(writer)?;
        }

        Ok(())
    }
}
