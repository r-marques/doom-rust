use crate::graphics::DrawParam;

use byte::ctx::*;
use byte::*;
use ggez::nalgebra as na;
use ggez::{conf, event, graphics, Context, GameResult};
use std::fs::File;
use std::io::Read;

#[derive(Debug)]
struct WadInfo {
    identification: String,
    numlumps: u32,
    infotableofs: u32,
}

#[derive(Debug)]
struct FileLump {
    filepos: u32,
    size: u32,
    name: String,
}

#[derive(Debug)]
struct Vertex {
    x: i16,
    y: i16,
}

#[derive(Debug)]
struct Linedef {
    v1: i16,
    v2: i16,
    flags: i16,
    special: i16,
    tag: i16,
    right_sidedef: i16,
    left_sidedef: i16,
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

struct Game {
    vertexes: Vec<Vertex>,
    linedefs: Vec<Linedef>,
}

impl event::EventHandler for Game {
    fn update(&mut self, _context: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, context: &mut Context) -> GameResult {
        let mb = &mut graphics::MeshBuilder::new();

        // draw all the lines with a dowscaled factor of 8
        for line in self.linedefs.iter() {
            let v1 = &self.vertexes[line.v1 as usize];
            let v2 = &self.vertexes[line.v2 as usize];
            let p1 = na::Point2::new((v1.x / 8) as f32, (v1.y / 8) as f32);
            let p2 = na::Point2::new((v2.x / 8) as f32, (v2.y / 8) as f32);
            mb.line(&[p1, p2], 1.0, graphics::WHITE)?;
        }

        let mesh = mb.build(context)?;
        graphics::draw(
            context,
            &mesh,
            DrawParam::new().dest(na::Point2::new(10.0, 10.0)),
        )?;

        graphics::present(context)
    }
}

fn main() -> GameResult {
    // Read the wad file
    let mut file = File::open("doom1.wad").unwrap();

    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();

    // Read the wad info / header
    let wad_info: WadInfo = buffer.as_slice().read_with(&mut 0, ()).unwrap();
    println!("{:?}", wad_info);

    // Read all the lumps
    let mut offset = wad_info.infotableofs as usize;
    let mut file_lumps = Vec::new();
    for _ in 0..wad_info.numlumps {
        let file_lump: FileLump = buffer.as_slice().read_with(&mut offset, ()).unwrap();
        println!("[{}] {:?}", offset, file_lump);
        file_lumps.push(file_lump);
    }

    // This reads the vertexes for the E1M1 map and breaks
    // out of the loop early skipping the other lumps
    let mut vertexes = Vec::new();
    for file_lump in file_lumps.iter() {
        if file_lump.name == "VERTEXES" {
            let numvertexes = file_lump.size / 4;
            let mut offset = file_lump.filepos as usize;
            println!("{} {}", file_lump.size, numvertexes);

            for _ in 0..numvertexes {
                let vertex: Vertex = buffer.as_slice().read_with(&mut offset, ()).unwrap();
                println!("{:?}", vertex);

                vertexes.push(vertex);
            }
            break;
        }
    }

    // Vertexes may have negative coordinates
    // here we are just finding the min x and y ...
    let min_x = vertexes.iter().map(|v| v.x).min().unwrap();
    let min_y = vertexes.iter().map(|v| v.y).min().unwrap();
    // and shifting the coordinates so that they are all positive
    let vertexes_corrected: Vec<_> = vertexes
        .iter()
        .map(|v| Vertex {
            x: v.x - min_x,
            y: v.y - min_y,
        })
        .collect();

    // This reads the linedefs for the E1M1 map and breaks
    // out of the loop early skipping the other lumps
    let mut linedefs = Vec::new();
    for file_lump in file_lumps.iter() {
        if file_lump.name == "LINEDEFS" {
            let numlines = file_lump.size / 14;
            let mut offset = file_lump.filepos as usize;

            for _ in 0..numlines {
                let linedef: Linedef = buffer.as_slice().read_with(&mut offset, ()).unwrap();
                println!("{:?}", linedef);

                linedefs.push(linedef);
            }
            break;
        }
    }

    // initialize ggez
    let context_builder = ggez::ContextBuilder::new("doom_rust", "doom")
        .window_setup(conf::WindowSetup::default().title("Oxidizing Doom"));

    // create the context and event_loop
    let (context, event_loop) = &mut context_builder.build()?;

    // initialize the game state
    let game = &mut Game {
        vertexes: vertexes_corrected,
        linedefs,
    };

    // run ggez
    event::run(context, event_loop, game)
}
