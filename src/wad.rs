use std::fs::File;
use std::io::Read;

use byte::ctx::Str;
use byte::{BytesExt, Result, TryRead, LE};

#[derive(Debug)]
pub struct WadInfo {
    identification: String,
    numlumps: u32,
    infotableofs: u32,
}

#[derive(Debug)]
pub struct FileLump {
    filepos: u32,
    size: u32,
    name: String,
}

#[derive(Debug)]
pub struct Vertex {
    pub x: i16,
    pub y: i16,
}

#[derive(Debug)]
pub struct Linedef {
    pub v1: i16,
    pub v2: i16,
    flags: i16,
    special: i16,
    tag: i16,
    pub right_sidedef: i16,
    pub left_sidedef: i16,
}

impl<'a> TryRead<'a> for WadInfo {
    fn try_read(bytes: &'a [u8], _ctx: ()) -> Result<(Self, usize)> {
        let offset = &mut 0;

        let wad_info = WadInfo {
            identification: bytes.read_with::<&str>(offset, Str::Len(4))?.into(),
            numlumps: bytes.read_with::<u32>(offset, LE)?,
            infotableofs: bytes.read_with::<u32>(offset, LE)?,
        };

        Ok((wad_info, *offset))
    }
}

impl<'a> TryRead<'a> for FileLump {
    fn try_read(bytes: &'a [u8], _ctx: ()) -> Result<(Self, usize)> {
        let offset = &mut 0;

        let file_lump = FileLump {
            filepos: bytes.read_with::<u32>(offset, LE)?,
            size: bytes.read_with::<u32>(offset, LE)?,
            name: bytes
                .read_with::<&str>(offset, Str::Len(8))?
                .trim_end_matches('\u{0}')
                .into(),
        };

        Ok((file_lump, *offset))
    }
}

impl<'a> TryRead<'a> for Vertex {
    fn try_read(bytes: &'a [u8], _ctx: ()) -> Result<(Self, usize)> {
        let offset = &mut 0;

        let vertex = Vertex {
            x: bytes.read_with::<i16>(offset, LE)?,
            y: bytes.read_with::<i16>(offset, LE)?,
        };

        Ok((vertex, *offset))
    }
}

impl<'a> TryRead<'a> for Linedef {
    fn try_read(bytes: &'a [u8], _ctx: ()) -> Result<(Self, usize)> {
        let offset = &mut 0;

        let linedef = Linedef {
            v1: bytes.read_with::<i16>(offset, LE)?,
            v2: bytes.read_with::<i16>(offset, LE)?,
            flags: bytes.read_with::<i16>(offset, LE)?,
            special: bytes.read_with::<i16>(offset, LE)?,
            tag: bytes.read_with::<i16>(offset, LE)?,
            right_sidedef: bytes.read_with::<i16>(offset, LE)?,
            left_sidedef: bytes.read_with::<i16>(offset, LE)?,
        };

        Ok((linedef, *offset))
    }
}

pub struct Wad {
    pub wad_info: WadInfo,
    pub file_lumps: Vec<FileLump>,
    pub vertexes: Vec<Vertex>,
    pub linedefs: Vec<Linedef>,
}

impl Wad {
    pub fn new(path: &str) -> Self {
        let mut file = File::open(path).expect("File not found");
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).expect("Error reading file");

        let wad_info: WadInfo = buffer
            .as_slice()
            .read_with(&mut 0, ())
            .expect("Error reading WadInfo");

        let mut offset = wad_info.infotableofs as usize;
        let file_lumps: Vec<FileLump> = (0..wad_info.numlumps)
            .map(|_| {
                buffer
                    .as_slice()
                    .read_with(&mut offset, ())
                    .expect("Error reading lump")
            })
            .collect();

        let mut vertexes: Vec<Vertex> = Vec::new();
        for lump in file_lumps.iter() {
            if lump.name == "VERTEXES" {
                let numvertexes = lump.size / 4;
                let mut offset = lump.filepos as usize;

                vertexes = (0..numvertexes)
                    .map(|_| {
                        buffer
                            .as_slice()
                            .read_with(&mut offset, ())
                            .expect("Error reading vertexes")
                    })
                    .collect();
                break;
            }
        }

        let mut linedefs: Vec<Linedef> = Vec::new();
        for lump in file_lumps.iter() {
            if lump.name == "LINEDEFS" {
                let numlines = lump.size / 14;
                let mut offset = lump.filepos as usize;

                linedefs = (0..numlines)
                    .map(|_| {
                        buffer
                            .as_slice()
                            .read_with(&mut offset, ())
                            .expect("Error reading vertexes")
                    })
                    .collect();
                break;
            }
        }

        Wad {
            wad_info,
            file_lumps,
            vertexes,
            linedefs,
        }
    }
}
