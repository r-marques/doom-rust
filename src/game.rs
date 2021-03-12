use crate::wad::Wad;
use ggez::graphics::{DrawMode, DrawParam};
use ggez::nalgebra as na;
use ggez::{conf, event, graphics, Context, GameResult};

use crate::wad::Linedef;
use crate::wad::Vertex;

const SCALE: i16 = 5;

struct GameState {
    vertexes: Vec<Vertex>,
    linedefs: Vec<Linedef>,
}

impl event::EventHandler for GameState {
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

pub struct Game {
    game_state: GameState,
}

impl Game {
    pub fn new(path: &str) -> Self {
        let wad = Wad::new(path);

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

        // initialize the game state
        let game_state = GameState {
            vertexes: vertexes_corrected,
            linedefs: wad.linedefs,
        };

        Game { game_state }
    }

    pub fn run(&mut self) -> GameResult {
        // initialize ggez
        let context_builder = ggez::ContextBuilder::new("doom_rust", "doom")
            .window_setup(conf::WindowSetup::default().title("Oxidizing Doom"))
            .window_mode(conf::WindowMode::default().dimensions(1280.0, 720.0));

        // create the context and event_loop
        let (context, event_loop) = &mut context_builder.build()?;

        // run ggez
        event::run(context, event_loop, &mut self.game_state)
    }
}
