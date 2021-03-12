use crate::graphics::DrawParam;
use ggez::graphics::DrawMode;

use ggez::nalgebra as na;
use ggez::{conf, event, graphics, Context, GameResult};

use doom_rust::wad::*;

const SCALE: i16 = 5;

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

        graphics::clear(context, graphics::WHITE);

        // draw all the lines with a dowscaled factor of SCALE
        for line in self.linedefs.iter() {
            let v1 = &self.vertexes[line.v1 as usize];
            let v2 = &self.vertexes[line.v2 as usize];
            let p1 = na::Point2::new((v1.x / SCALE) as f32, (v1.y / SCALE) as f32);
            let p2 = na::Point2::new((v2.x / SCALE) as f32, (v2.y / SCALE) as f32);

            let color = if line.left_sidedef == -1 {
                graphics::BLACK
            } else {
                graphics::Color::new(1.0, 0.0, 0.0, 1.0)
            };
            mb.line(&[p1, p2], 1.0, color)?;
            mb.circle(
                DrawMode::fill(),
                na::Point2::new((v1.x / SCALE) as f32, (v1.y / SCALE) as f32),
                2.0,
                2.0,
                graphics::BLACK,
            );
            mb.circle(
                DrawMode::fill(),
                na::Point2::new((v2.x / SCALE) as f32, (v2.y / SCALE) as f32),
                2.0,
                2.0,
                graphics::BLACK,
            );
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
    let wad = Wad::new("doom1.wad");

    // Vertexes may have negative coordinates
    // here we are just finding the min x and y ...
    let min_x = wad.vertexes.iter().map(|v| v.x).min().unwrap();
    let min_y = wad.vertexes.iter().map(|v| v.y).min().unwrap();
    // and shifting the coordinates so that they are all positive
    let vertexes_corrected: Vec<_> = wad
        .vertexes
        .iter()
        .map(|v| Vertex {
            x: v.x - min_x,
            y: v.y - min_y,
        })
        .collect();

    // initialize ggez
    let context_builder = ggez::ContextBuilder::new("doom_rust", "doom")
        .window_setup(conf::WindowSetup::default().title("Oxidizing Doom"))
        .window_mode(conf::WindowMode::default().dimensions(1280.0, 720.0));

    // create the context and event_loop
    let (context, event_loop) = &mut context_builder.build()?;

    // initialize the game state
    let game = &mut Game {
        vertexes: vertexes_corrected,
        linedefs: wad.linedefs,
    };

    // run ggez
    event::run(context, event_loop, game)
}
