use crate::align_four_engine::{AlignFourEngine, Team};
use sfml::{
    graphics::{CircleShape, Color, Rect, RenderTarget, RenderWindow, Shape, Transformable, View},
    system::Vector2,
    window::{mouse::Button, Event, Key, Style},
};
use std::{
    cmp::max,
    time::{Duration, Instant},
};

const BLUE_TEAM: Color = Color::rgb(22, 130, 224);
const RED_TEAM: Color = Color::rgb(150, 29, 47);
const FPS_LIMIT: u64 = 120;
const FRAME_TIME: Duration = Duration::from_millis(1000 / FPS_LIMIT);

pub struct GUIWrapper<'a> {
    engine: AlignFourEngine,
    circles: Vec<CircleShape<'a>>,
    window: RenderWindow,
}

impl<'a> GUIWrapper<'a> {
    //Constructors
    pub fn default() -> Self {
        Self {
            engine: AlignFourEngine::default(),
            circles: vec![CircleShape::new(50.0, 30); 6 * 7],
            window: RenderWindow::new(
                (1280, 720),
                "alignfour",
                Style::default(),
                &Default::default(),
            ),
        }
    }

    pub fn run(mut self) {
        while self.window.is_open() {
            let time_start_gameloop = Instant::now();
            self.handle_events();
            if self.handle_win() {
                self.window.close()
            }
            self.update_circles();
            self.window.clear(Color::rgb(96, 96, 96));
            self.draw_circles();
            self.window.display();

            let duration_gameloop = time_start_gameloop.elapsed();
            if duration_gameloop < FRAME_TIME {
                std::thread::sleep(FRAME_TIME - duration_gameloop);
            }
        }
    }

    fn handle_events(&mut self) {
        while let Some(ev) = self.window.poll_event() {
            match ev {
                Event::Closed
                | Event::KeyPressed {
                    code: Key::Escape, ..
                } => self.window.close(),
                Event::Resized { width, height } => {
                    self.window.set_view(&View::from_rect(Rect::new(
                        0.,
                        0.,
                        width as f32,
                        height as f32,
                    )));
                    self.resize(width, height);
                }
                Event::MouseButtonPressed {
                    button: Button::Left,
                    x,
                    y,
                } => self.handle_click(x, y),
                _ => (),
            }
        }
    }

    fn handle_win(&self) -> bool {
        match self.engine.check_win() {
            Some(Team::Red | Team::Blue) => {
                println!("Les {} ont gagnés !!!", self.turn_color());
                true
            }
            Some(Team::Nothing) => {
                println!("C'est une égalité !!!");
                true
            }
            _ => false,
        }
    }

    fn handle_click(&mut self, x: i32, y: i32) {
        for gx in 0..self.engine.width() {
            for gy in 0..self.engine.height() {
                if self
                    .at(gx, gy)
                    .global_bounds()
                    .contains(Vector2::new(x as f32, y as f32))
                {
                    match self.engine.play_at(gx) {
                        Ok(_) => self.engine.switch_turns(),
                        Err(_) => (),
                    }
                }
            }
        }
    }

    fn resize(&mut self, width: u32, height: u32) {
        let padding_x = 150;
        let padding_y = 120;
        let offset_x = max(
            50,
            (width as i32 - self.engine.width() as i32 * padding_x) / 2,
        );
        let offset_y = max(
            50,
            (height as i32 - self.engine.height() as i32 * padding_y) / 2,
        );
        for x in 0..self.engine.width() {
            for y in 0..self.engine.height() {
                self.at_mut(x, y).set_position((
                    x as f32 * padding_x as f32 + offset_x as f32,
                    y as f32 * padding_y as f32 + offset_y as f32,
                ));
            }
        }
    }

    fn draw_circles(&mut self) {
        for circle in &self.circles {
            self.window.draw(circle);
        }
    }

    fn update_circles(&mut self) {
        for (i, cell) in self.engine.grid().iter().enumerate() {
            self.circles[i].set_fill_color(match cell {
                Team::Blue => BLUE_TEAM,
                Team::Red => RED_TEAM,
                Team::Nothing => Color::rgb(64, 64, 64),
            });
        }
    }

    fn turn_color(&self) -> String {
        if self.engine.turn() != Team::Red {
            String::from("rouges")
        } else {
            String::from("bleus")
        }
    }

    // Setters \ Getters
    fn at(&self, x: usize, y: usize) -> &CircleShape {
        &self.circles[y * self.engine.width() + x]
    }

    fn at_mut<'b>(&'b mut self, x: usize, y: usize) -> &'b mut CircleShape<'a> {
        &mut self.circles[y * self.engine.width() + x]
    }
}
