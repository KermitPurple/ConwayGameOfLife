use ggez::{graphics, Context, ContextBuilder, GameResult};
use ggez::event::{self, EventHandler};
use ggez::nalgebra as na;

fn main() {
    const SIZE: [i32; 2] = [600, 600];
    const SCALE: i32 = 10;
    let (mut ctx, mut event_loop) = ContextBuilder::new("Conway's game of life", "Shane McDonough")
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
            board: vec![vec![false; (size[0] / scale) as usize]; (size[1] / scale) as usize],
        }
    }

    fn count_neighbors(&self, y: i32, x: i32) -> i32{
        let mut count: i32 = 0;
        for mut i in -1..=1{
            i += y;
            for mut j in -1..=1{
                j += x;
                if (i != y || j != x) && i >= 0 && i < self.size[1] / self.scale && j >= 0 && j < self.size[0] / self.scale{
                    if self.board[i as usize][j as usize] {
                        count += 1;
                    }
                }
            }
        }
        count
    }

    fn act_on_count(&mut self, i: usize, j: usize, count: i32){
        if self.board[i][j] {
            if count < 2 || count > 3{
                self.board[i][j] = false;
            }
        } else {
            if count == 3 {
                self.board[i][j] = true;
            }
        }
    }

    fn update_board(&mut self){
        for i in 0..(self.size[0] / self.scale){
            for j in 0..(self.size[1] / self.scale){
                self.act_on_count(i as usize, j as usize, self.count_neighbors(i, j));
            }
        }
    }
}

impl EventHandler for ConwaysGame {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        self.update_board();
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, graphics::WHITE);
        graphics::present(ctx);
        Ok(())
    }
}
