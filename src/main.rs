use ggez::{graphics, Context, ContextBuilder, GameResult};
use ggez::event::{self, EventHandler};
use ggez::nalgebra as na;

fn main() {
    const SIZE: [i32; 2] = [600, 600];
    const SCALE: i32 = 10;
    let (mut ctx, mut event_loop) = ContextBuilder::new("Conway's game of life", "Shane")
        .window_setup(ggez::conf::WindowSetup {
            title: "Conway's Game of life".to_owned(),
            samples: ggez::conf::NumSamples::Zero,
            vsync: true,
            icon: "".to_owned(),
            srgb: true,
        })
        .window_mode(ggez::conf::WindowMode {
            width: SIZE[0] as f32,
            height: SIZE[1] as f32,
            maximized: false,
            fullscreen_type: ggez::conf::FullscreenType::Windowed,
            borderless: false,
            min_width: 0.0,
            max_width: 0.0,
            min_height: 0.0,
            max_height: 0.0,
            resizable: false,
        })
        .build()
        .expect("Could not create context");

    let mut game = ConwaysGame::new(SIZE, SCALE);

    match event::run(&mut ctx, &mut event_loop, &mut game) {
        Ok(_) => println!("Success"),
        Err(e) => println!("Failure: {}", e)
    }
}

struct ConwaysGame {
    size: [i32; 2],
    scale: i32,
    board: Vec<Vec<bool>>,
}

impl ConwaysGame {
    fn new(size: [i32; 2], scale: i32) -> Self {
        Self {
            size: size,
            scale: scale,
            board: vec![vec![false; size[0] as usize]; size[1] as usize],
        }
    }
}

impl EventHandler for ConwaysGame {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, graphics::WHITE);
        graphics::present(ctx);
        Ok(())
    }
}
