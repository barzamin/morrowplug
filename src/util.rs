use std::ffi::CString;
use std::io::{self, Read};
use std::num::NonZeroU8;

pub fn parse_zstring<R>(rdr: &mut R) -> Result<CString, io::Error>
where
    R: Read,
{
    let mut buf: Vec<NonZeroU8> = Vec::new();
    let mut c: u8 = 0;
    loop {
        let slice = std::slice::from_mut(&mut c);
        rdr.read_exact(slice)?;

        if let Some(ch) = NonZeroU8::new(c) {
            buf.push(ch);
        } else {
            return Ok(CString::from(buf));
        }
    }
}
