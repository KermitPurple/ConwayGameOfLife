use ggez::{graphics, Context, ContextBuilder, GameResult};
use ggez::event::{self, EventHandler, KeyCode};
use ggez::nalgebra as na;
use ggez::input::{keyboard, mouse};


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
    ggez::timer::yield_now();
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

    fn randomize_board(&mut self){
        for i in 0..((self.size[0] / self.scale) as usize) {
            for j in 0..((self.size[1] / self.scale) as usize) {
                self.board[i][j] = rand::random();
            }
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
        let mut counts = vec![vec![0; (self.size[0] / self.scale) as usize]; (self.size[1] / self.scale) as usize];
        for i in 0..(self.size[1] / self.scale){
            for j in 0..(self.size[0] / self.scale){
                counts[i as usize][j as usize] = self.count_neighbors(i, j);
            }
        }
        for i in 0..(self.size[1] / self.scale){
            for j in 0..(self.size[0] / self.scale){
                self.act_on_count(i as usize, j as usize, counts[i as usize][j as usize]);
            }
        }
    }

    fn print_board(&self, ctx: &mut Context){
        let rectangle = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            graphics::Rect {
                x: 0_f32,
                y: 0_f32,
                w: self.scale as f32,
                h: self.scale as f32,
            },
            graphics::BLACK,
            ).unwrap();
        for i in 0..(self.size[1] / self.scale){
            for j in 0..(self.size[0] / self.scale){
                if self.board[i as usize][j as usize] {
                    graphics::draw(
                        ctx,
                        &rectangle,
                        graphics::DrawParam::default().dest(na::Point2::new((j * self.scale) as f32, (i * self.scale) as f32)),
                        ).unwrap();
                }
            }
        }
    }

    fn toggle_on_at_click(&mut self, position: [f32; 2]){
        self.board[position[1] as usize / self.scale as usize][position[0] as usize / self.scale as usize] = true;
    }

    fn toggle_off_at_click(&mut self, position: [f32; 2]){
        self.board[position[1] as usize / self.scale as usize][position[0] as usize / self.scale as usize] = false;
    }

    fn clear_board(&mut self){
        for i in 0..(self.size[1] / self.scale){
            for j in 0..(self.size[0] / self.scale){
                self.board[i as usize][j as usize] = false;
            }
        }
    }
}

impl EventHandler for ConwaysGame {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        if keyboard::is_key_pressed(_ctx, KeyCode::Space){
            self.update_board();
        }
        if keyboard::is_key_pressed(_ctx, KeyCode::R) {
            self.randomize_board();
        }
        if keyboard::is_key_pressed(_ctx, KeyCode::C){
            self.clear_board();
        }
        if mouse::button_pressed(_ctx, mouse::MouseButton::Left) {
            let position = mouse::position(_ctx);
            self.toggle_on_at_click([position.x, position.y]);
        }
        if mouse::button_pressed(_ctx, mouse::MouseButton::Right) {
            let position = mouse::position(_ctx);
            self.toggle_off_at_click([position.x, position.y]);
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, graphics::WHITE);
        self.print_board(ctx);
        graphics::present(ctx)
    }
}
