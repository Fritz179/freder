pub use minifb::Key;

use crate::canvas::Canvas;

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

        assert_eq!(canvas.get_buffer().len(), self.window.get_size().0 * self.window.get_size().1);

        self.window.update_with_buffer(canvas.get_buffer(), canvas.width(), canvas.height()).unwrap();
    }

    pub fn is_key_down(&self, key: Key) -> bool {
        self.window.is_key_down(key)
    }
}