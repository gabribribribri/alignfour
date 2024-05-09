use sfml::{
    graphics::{Color, RenderTarget, RenderWindow},
    window::{Event, Key, Style, VideoMode},
};

use crate::align_four_engine::AlignFourEngine;

pub struct GUIWrapper {
    engine: AlignFourEngine,
    window: RenderWindow,
}

impl GUIWrapper {
    pub fn default() -> Self {
        Self {
            engine: AlignFourEngine::default(),
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
            while let Some(ev) = self.window.poll_event() {
                match ev {
                    Event::Closed
                    | Event::KeyPressed {
                        code: Key::Escape, ..
                    } => self.window.close(),
                    _ => (),
                }
            }
            self.window.clear(Color::rgb(32, 32, 32));
            self.window.display();
        }
    }
}
