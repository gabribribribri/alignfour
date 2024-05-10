use crate::align_four_engine::{AlignFourEngine, Team};
use sfml::{
    graphics::{CircleShape, Color, Rect, RenderTarget, RenderWindow, Shape, Transformable, View},
    window::{mouse::Button, Event, Key, Style},
};
use std::{
    cmp::max,
    time::{Duration, Instant},
};

const BLUE_COLOR_LIGHT: Color = Color::rgb(22, 130, 224);
const RED_COLOR_LIGHT: Color = Color::rgb(150, 29, 47);
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
        let circles = vec![CircleShape::new(50.0, 30); 6 * 7];
        Self {
            engine: AlignFourEngine::default(),
            circles,
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
            self.handle_over();
            self.update_circles();
            //draw to screen
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
                    ..
                } => self.handle_click(x),
                _ => (),
            }
        }
    }

    fn handle_win(&self) -> bool {
        match self.engine.check_win() {
            Some(Team::Red | Team::Blue) => {
                println!("Les {} ont gagnés !!!", self.turn_color_string());
                true
            }
            Some(Team::Nothing) => {
                println!("C'est une égalité !!!");
                true
            }
            _ => false,
        }
    }

    fn handle_click(&mut self, x: i32) {
        for gx in 0..self.engine.width() {
            let col = self.at(gx, 0).global_bounds();
            let col_left = col.left;
            let col_right = col.left + col.width;
            if col_left < x as f32 && col_right > x as f32 {
                match self.engine.play_at(gx) {
                    Ok(_) => self.engine.switch_turns(),
                    Err(_) => (),
                }
            }
        }
    }

    fn handle_over(&mut self) {
        let x = self.window.mouse_position().x;
        for gx in 0..self.engine.width() {
            let col = self.at(gx, 0).global_bounds();
            let col_left = col.left;
            let col_right = col.left + col.width;
            if col_left < x as f32 && col_right > x as f32 {
                for gy in 0..self.engine.height() {
                    let turn_color = self.turn_color_light();
                    self.at_mut(gx, gy).set_outline_thickness(-5.0);
                    self.at_mut(gx, gy).set_outline_color(turn_color);
                }
            } else {
                for gy in 0..self.engine.height() {
                    self.at_mut(gx, gy).set_outline_thickness(0.0);
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
                Team::Blue => BLUE_COLOR_LIGHT,
                Team::Red => RED_COLOR_LIGHT,
                Team::Nothing => Color::rgb(64, 64, 64),
            });
        }
    }

    fn turn_color_string(&self) -> String {
        if self.engine.turn() != Team::Red {
            String::from("rouges")
        } else {
            String::from("bleus")
        }
    }

    fn turn_color_light(&self) -> Color {
        if self.engine.turn() == Team::Red {
            RED_COLOR_LIGHT
        } else {
            BLUE_COLOR_LIGHT
        }
    }

    // Setters \ Getters
    fn at(&self, x: usize, y: usize) -> &CircleShape {
        &self.circles[y * self.engine.width() + x]
    }

    // j'ai spam toutes les combinaisons de lifetime jusqu'à ce que ça fonctionne mais maintenant j'ai compris
    fn at_mut<'b>(&'b mut self, x: usize, y: usize) -> &'b mut CircleShape<'a> {
        &mut self.circles[y * self.engine.width() + x]
    }
}
