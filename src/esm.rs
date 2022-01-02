use bitflags::bitflags;
use byteorder::{ReadBytesExt, LE};
use std::io::{Read, Seek, SeekFrom};
use thiserror::Error;

mod tes3;

bitflags! {
    struct RecordFlags: u32 {
        const DELETED      = 0x0020;
        const PRESISTENT   = 0x0400;
        const INIT_DISABLE = 0x0800;
        const BLOCKED      = 0x2000;
    }
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("bad record name encoding")]
    BadRecordName(std::str::Utf8Error),

    #[error("i/o error")]
    IoError(#[from] std::io::Error),

    #[error("bad record flags")]
    RecordFlagsInvalid,

    #[error("bad field name encoding")]
    BadFieldEncoding(std::str::Utf8Error),

    #[error("unknown field name")]
    BadField,
}

type Result<T> = std::result::Result<T, Error>;

trait Record: FieldParser {
    fn parse() -> Self;
}

trait FieldParser {
    type Field;
    fn parse_field<R>(rdr: &mut R) -> Result<Self::Field>
    where
        R: Read + Seek,
    {
        let mut ty: [u8; 4] = [0; 4];
        rdr.read_exact(&mut ty)?;
        let ty = std::str::from_utf8(&ty)
            .map_err(Error::BadFieldEncoding)?
            .to_owned();

        let size = rdr.read_u32::<LE>()?;

        Self::parse_field_data(rdr, &ty, size)
    }

    fn parse_field_data<R>(rdr: &mut R, ty: &str, size: u32) -> Result<Self::Field>
    where
        R: Read + Seek;
}

// struct RecordHeader {
//  ty: String,
//  size: u32,
//  flags: RecordFlags,
// }

fn parse_recordhdr<R>(rdr: &mut R) -> Result<()>
where
    R: Read + ReadBytesExt,
{
    let mut name: [u8; 4] = [0; 4];
    rdr.read_exact(&mut name)?;
    let name = std::str::from_utf8(&name)
        .map_err(Error::BadRecordName)?
        .to_owned();

    let size = rdr.read_u32::<LE>()?;
    let flags = RecordFlags::from_bits(rdr.read_u32::<LE>()?).ok_or(Error::RecordFlagsInvalid)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
}
