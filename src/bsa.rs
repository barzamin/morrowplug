use byteorder::{ReadBytesExt, LE};
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::path::Path;
use thiserror::Error;

use crate::util::parse_zstring;

#[derive(Error, Debug)]
pub enum Error {
    #[error("bad BSA header magic")]
    BadMagic,
    #[error("i/o error")]
    IoError(#[from] std::io::Error),
    #[error("bad filename encoding")]
    BadFilenameEncoding(std::ffi::IntoStringError),
    #[error("woof")]
    Xxx,
}

type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
struct Header {
    offs_hashtbl: u32, // minus header size; assumes seek from end of header
    n_files: u32,
}

fn parse_header<R>(rdr: &mut R) -> Result<Header>
where
    R: Read + ReadBytesExt,
{
    let magic = rdr.read_u32::<LE>()?;
    if magic != 0x00000100 {
        return Err(Error::BadMagic);
    }

    let offs_hashtbl = rdr.read_u32::<LE>()?;
    let n_files = rdr.read_u32::<LE>()?;

    Ok(Header {
        offs_hashtbl,
        n_files,
    })
}

#[derive(Debug, Copy, Clone)]
struct FileTableEntry {
    size: u32,
    offs: u32,
}
#[derive(Debug)]
struct FileTable(Vec<FileTableEntry>);

fn parse_filetab<R>(rdr: &mut R, n_files: u32) -> Result<FileTable>
where
    R: Read + ReadBytesExt,
{
    let mut entries = Vec::new();
    for _ in 0..n_files {
        let size = rdr.read_u32::<LE>()?;
        let offs = rdr.read_u32::<LE>()?;

        entries.push(FileTableEntry { size, offs });
    }

    Ok(FileTable(entries))
}

fn parse_nametab<R>(rdr: &mut R, n_files: u32) -> Result<Vec<String>>
where
    R: Read + Seek + ReadBytesExt,
{
    // skip offsets because we don't care; we read everything anyway
    rdr.seek(SeekFrom::Current(n_files as i64 * 4))?; // name_offset_table: [u32; n_files]

    // let mut fname_offsets = Vec::new();
    // for _ in 0..n_files {
    //     fname_offsets.push(rdr.read_u32::<LE>()?);
    // }

    let mut names = Vec::new();
    for _ in 0..n_files {
        names.push(
            parse_zstring(rdr)?
                .into_string()
                .map_err(Error::BadFilenameEncoding)?,
        );
    }

    Ok(names)
}

fn parse_hashes<R>(rdr: &mut R, n_files: u32) -> Result<Vec<u64>> where R: Read + ReadBytesExt {
    let mut hashes = Vec::new();
    for _ in 0..n_files {
        hashes.push(rdr.read_u64::<LE>()?);
    }

    Ok(hashes)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse_header() {
        let mut rdr = std::io::Cursor::new(&[
            0x00, 0x01, 0x00, 0x00, 0xb8, 0x7c, 0x07, 0x00, 0x52, 0x2b, 0x00, 0x00,
        ]);

        let hdr = parse_header(&mut rdr).unwrap();
        assert_eq!(hdr.n_files, 11090);
        assert_eq!(hdr.offs_hashtbl, 490680);
    }

    #[test]
    fn test_parse_filetab() {
        let mut rdr = File::open(Path::new(env!("MORROWIND_DATA")).join("Morrowind.bsa")).unwrap();
        let hdr = parse_header(&mut rdr).unwrap();
        assert_eq!(hdr.n_files, 11090);
        assert_eq!(hdr.offs_hashtbl, 490680);

        let _filetab = parse_filetab(&mut rdr, hdr.n_files).unwrap();
    }

    #[test]
    fn test_parse_nametab() {
        let mut rdr = File::open(Path::new(env!("MORROWIND_DATA")).join("Morrowind.bsa")).unwrap();
        let hdr = parse_header(&mut rdr).unwrap();
        assert_eq!(hdr.n_files, 11090);
        assert_eq!(hdr.offs_hashtbl, 490680);

        let _filetab = parse_filetab(&mut rdr, hdr.n_files).unwrap();
        let _nametab = parse_nametab(&mut rdr, hdr.n_files).unwrap();
    }

    #[test]
    fn test_parse_hashes() {
        let mut rdr = File::open(Path::new(env!("MORROWIND_DATA")).join("Morrowind.bsa")).unwrap();
        let hdr = parse_header(&mut rdr).unwrap();
        assert_eq!(hdr.n_files, 11090);
        assert_eq!(hdr.offs_hashtbl, 490680);

        let _filetab = parse_filetab(&mut rdr, hdr.n_files).unwrap();
        let _nametab = parse_nametab(&mut rdr, hdr.n_files).unwrap();
        let _hashes = parse_hashes(&mut rdr, hdr.n_files).unwrap();
    }
}
