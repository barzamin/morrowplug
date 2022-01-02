use crate::esm::{Error, FieldParser, Result};
use binread::BinRead;
use std::io::{Read, Seek};

pub mod fields {
    use binread::BinRead;

    #[derive(BinRead)]
    #[br(little)]
    pub struct HEDR {
        ver: f32,
        flags: u32,
        #[br(count = 32, try_map = |x: Vec<u8>| String::from_utf8(x))]
        company: String,
        #[br(count = 128, try_map = |x: Vec<u8>| String::from_utf8(x))]
        desc: String,
        subsequent_records: u32,
    }
}

// struct TES3 {
//     header: fields::HEDR,
//     // mast:
// }

struct TES3;

enum TES3Fields {
    HEDR(fields::HEDR),
    MAST { file: String, size: u64 },
}

impl FieldParser for TES3 {
    type Field = TES3Fields;
    fn parse_field_data<R>(rdr: &mut R, ty: &str, size: u32) -> Result<Self::Field>
    where
        R: Read + Seek,
    {
        match ty {
            "HEDR" => Ok(TES3Fields::HEDR(fields::HEDR::read(rdr)?)),
            _ => Err(Error::BadField),
        }
    }
}
