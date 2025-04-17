use crate::prelude::*;

pub mod event;

mod minifb;
pub use minifb::Window;

pub trait WindowTrait {
    fn new(title: &str, width: usize, height: usize, app: impl App);
    
    fn is_open(&self) -> bool;

    fn mouse_just_pressed(&self, button: MouseButton) -> bool;
    fn mouse_just_released(&self, button: MouseButton) -> bool;
    fn mouse_is_pressed(&self, button: MouseButton) -> bool;
    fn mouse_pos(&self) -> Vec2<i32>;
    fn mouse_delta(&self) -> Vec2<i32>;

    fn key_just_pressed(&self, key: Key) -> bool;
    fn key_just_released(&self, key: Key) -> bool;
    fn key_is_pressed(&self, key: Key) -> bool;
}

pub trait App {
    fn event(&mut self, _window: &mut Window, _event: Event) {}
    fn update(&mut self, _window: &mut Window) {}
    fn render(&mut self, window: &mut Window, canvas: &mut dyn Canvas);
}

pub struct MouseManager {
    current:[bool; std::mem::variant_count::<MouseButton>()],
    previous: [bool; std::mem::variant_count::<MouseButton>()],

    position: Vec2<i32>,
    prev_position: Vec2<i32>,
}

impl MouseManager {
    fn new(mouse_pos: Vec2) -> Self {
        Self {
            current: [false; std::mem::variant_count::<MouseButton>()],
            previous: [false; std::mem::variant_count::<MouseButton>()],
            position: mouse_pos,
            prev_position: mouse_pos,
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

    fn set_pos(&mut self, pos: Vec2<i32>) {
        self.prev_position = self.position;
        self.position = pos;
    }

    fn pos(&self) -> Vec2<i32> {
        self.position
    }

    fn delta(&self) -> Vec2<i32> {
        self.position - self.prev_position
    }
}

pub struct KeyboardManager {
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