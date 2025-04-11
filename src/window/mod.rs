use crate::prelude::*;

pub mod event;
pub mod minifb;

use event::*;

pub struct Frender {

}

impl Frender {
    pub fn new(title: &str, width: usize, height: usize, mut app: impl App) {
        let (mut window, mut canvas) = Window::new(title, width, height).unwrap();
        while window.is_open() {

            app.update(&mut window);

            app.render(&mut window, &mut canvas);

            window.render(&mut canvas);

        }
    }
}


pub trait WindowTrait {
    fn new(title: &str, width: usize, height: usize) -> Self where Self: Sized;
    
    fn is_open(&self) -> bool;

    fn mouse_just_pressed(&self) -> bool;
    fn mouse_just_released(&self) -> bool;
    fn mouse_is_pressed(&self) -> bool;
    fn mouse_pos(&self) -> Option<Vec2<i32>>;

    fn key_just_pressed(&self, key: Key) -> bool;
    fn key_just_released(&self, key: Key) -> bool;
    fn key_is_pressed(&self, key: Key) -> bool;
}

pub trait App {
    fn event(&mut self, _window: &mut Window, _event: Event) {}
    fn update(&mut self, _window: &mut Window) {}
    fn render(&mut self, window: &mut Window, canvas: &mut Canvas);
}

struct MouseManager {
    current:[bool; std::mem::variant_count::<MouseButton>()],
    previous: [bool; std::mem::variant_count::<MouseButton>()],
    position: Vec2<i32>,
}

impl MouseManager {
    fn new() -> Self {
        Self {
            current: [false; std::mem::variant_count::<MouseButton>()],
            previous: [false; std::mem::variant_count::<MouseButton>()],
            position: Vec2::new(0, 0),
        }
    }

    fn update(&mut self) {
        self.previous.copy_from_slice(&self.current);
    }

    fn set(&mut self, button: MouseButton, pressed: bool) {
        self.current[button as usize] = pressed;
    }

    fn is_pressed(&self, button: MouseButton) -> bool {
        self.current[button as usize]
    }

    fn just_pressed(&self, button: MouseButton) -> bool {
        self.current[button as usize] && !self.previous[button as usize]
    }

    fn just_released(&self, button: MouseButton) -> bool {
        !self.current[button as usize] && self.previous[button as usize]
    }
}

struct KeyboardManager {
    current: [bool; std::mem::variant_count::<Key>()],
    previous: [bool; std::mem::variant_count::<Key>()],
}

impl KeyboardManager {
    fn new() -> Self {
        Self {
            current: [false; std::mem::variant_count::<Key>()],
            previous: [false; std::mem::variant_count::<Key>()],
        }
    }

    fn update(&mut self) {
        self.previous.copy_from_slice(&self.current);
    }

    fn set(&mut self, key: Key, pressed: bool) {
        self.current[key as usize] = pressed;
    }

    fn is_pressed(&self, key: Key) -> bool {
        self.current[key as usize]
    }

    fn just_pressed(&self, key: Key) -> bool {
        self.current[key as usize] && !self.previous[key as usize]
    }

    fn just_released(&self, key: Key) -> bool {
        !self.current[key as usize] && self.previous[key as usize]
    }
}