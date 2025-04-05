pub use minifb::Key;
use crate::prelude::*;

pub struct Window {
    window: minifb::Window,
}

impl Window {
    pub fn new(title: &str, width: usize, height: usize) -> Result<(Self, Canvas), minifb::Error> {
        let mut window = minifb::Window::new(title, width, height, minifb::WindowOptions::default())?;
        // Limit to max ~60 fps update rate
        window.set_target_fps(60);

        let canvas = Canvas::new(width, height);
        Ok((Self { window }, canvas))
    }

    pub fn is_open(&self) -> bool {
        self.window.is_open() && !self.window.is_key_down(Key::Q)
    }

    pub fn render(&mut self, canvas: &mut Canvas) {
        canvas.render_markers();

        // TODO: Do it better
        let buffer: Vec<u32> = canvas.get_buffer().into_iter().map(|color| color.as_u32()).collect();

        assert_eq!(buffer.len(), self.window.get_size().0 * self.window.get_size().1);

        self.window.update_with_buffer(&buffer, canvas.width(), canvas.height()).unwrap();
    }

    pub fn is_key_down(&self, key: Key) -> bool {
        self.window.is_key_down(key)
    }

    pub fn get_mouse_pos(&self) -> Option<Vec2<i32>> {
        self.window.get_mouse_pos(minifb::MouseMode::Pass).map(|(x, y)| Vec2::new(x as i32, y as i32))
    }

    pub fn is_mouse_down(&self) -> bool {
        self.window.get_mouse_down(minifb::MouseButton::Left)
    }
}