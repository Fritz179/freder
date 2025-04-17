use std::{cell::RefCell, rc::Rc};

use crate::prelude::*;

use super::{KeyboardManager, MouseManager};

pub struct Window {
    window: minifb::Window,
    keyboard: Rc<RefCell<KeyboardManager>>,
    mouse: MouseManager,
}

impl From<minifb::Key> for Key {
    fn from(value: minifb::Key) -> Self {
        match value {
            minifb::Key::Key0 => Key::Key0,
            minifb::Key::Key1 => Key::Key1,
            minifb::Key::Key2 => Key::Key2,
            minifb::Key::Key3 => Key::Key3,
            minifb::Key::Key4 => Key::Key4,
            minifb::Key::Key5 => Key::Key5,
            minifb::Key::Key6 => Key::Key6,
            minifb::Key::Key7 => Key::Key7,
            minifb::Key::Key8 => Key::Key8,
            minifb::Key::Key9 => Key::Key9,

            minifb::Key::A => Key::A,
            minifb::Key::B => Key::B,
            minifb::Key::C => Key::C,
            minifb::Key::D => Key::D,
            minifb::Key::E => Key::E,
            minifb::Key::F => Key::F,
            minifb::Key::G => Key::G,
            minifb::Key::H => Key::H,
            minifb::Key::I => Key::I,
            minifb::Key::J => Key::J,
            minifb::Key::K => Key::K,
            minifb::Key::L => Key::L,
            minifb::Key::M => Key::M,
            minifb::Key::N => Key::N,
            minifb::Key::O => Key::O,
            minifb::Key::P => Key::P,
            minifb::Key::Q => Key::Q,
            minifb::Key::R => Key::R,
            minifb::Key::S => Key::S,
            minifb::Key::T => Key::T,
            minifb::Key::U => Key::U,
            minifb::Key::V => Key::V,
            minifb::Key::W => Key::W,
            minifb::Key::X => Key::X,
            minifb::Key::Y => Key::Y,
            minifb::Key::Z => Key::Z,

            minifb::Key::F1 => Key::F1,
            minifb::Key::F2 => Key::F2,
            minifb::Key::F3 => Key::F3,
            minifb::Key::F4 => Key::F4,
            minifb::Key::F5 => Key::F5,
            minifb::Key::F6 => Key::F6,
            minifb::Key::F7 => Key::F7,
            minifb::Key::F8 => Key::F8,
            minifb::Key::F9 => Key::F9,
            minifb::Key::F10 => Key::F10,
            minifb::Key::F11 => Key::F11,
            minifb::Key::F12 => Key::F12,
            minifb::Key::F13 => Key::F13,
            minifb::Key::F14 => Key::F14,
            minifb::Key::F15 => Key::F15,

            minifb::Key::Down => Key::Down,
            minifb::Key::Left => Key::Left,
            minifb::Key::Right => Key::Right,
            minifb::Key::Up => Key::Up,
            minifb::Key::Apostrophe => Key::Apostrophe,
            minifb::Key::Backquote => Key::Backquote,

            minifb::Key::Backslash => Key::Backslash,
            minifb::Key::Comma => Key::Comma,
            minifb::Key::Equal => Key::Equal,
            minifb::Key::LeftBracket => Key::LeftBracket,
            minifb::Key::Minus => Key::Minus,
            minifb::Key::Period => Key::Period,
            minifb::Key::RightBracket => Key::RightBracket,
            minifb::Key::Semicolon => Key::Semicolon,

            minifb::Key::Slash => Key::Slash,
            minifb::Key::Backspace => Key::Backspace,
            minifb::Key::Delete => Key::Delete,
            minifb::Key::End => Key::End,
            minifb::Key::Enter => Key::Enter,

            minifb::Key::Escape => Key::Escape,

            minifb::Key::Home => Key::Home,
            minifb::Key::Insert => Key::Insert,
            minifb::Key::Menu => Key::Menu,

            minifb::Key::PageDown => Key::PageDown,
            minifb::Key::PageUp => Key::PageUp,

            minifb::Key::Pause => Key::Pause,
            minifb::Key::Space => Key::Space,
            minifb::Key::Tab => Key::Tab,
            minifb::Key::NumLock => Key::NumLock,
            minifb::Key::CapsLock => Key::CapsLock,
            minifb::Key::ScrollLock => Key::ScrollLock,
            minifb::Key::LeftShift => Key::LeftShift,
            minifb::Key::RightShift => Key::RightShift,
            minifb::Key::LeftCtrl => Key::LeftCtrl,
            minifb::Key::RightCtrl => Key::RightCtrl,

            minifb::Key::NumPad0 => Key::NumPad0,
            minifb::Key::NumPad1 => Key::NumPad1,
            minifb::Key::NumPad2 => Key::NumPad2,
            minifb::Key::NumPad3 => Key::NumPad3,
            minifb::Key::NumPad4 => Key::NumPad4,
            minifb::Key::NumPad5 => Key::NumPad5,
            minifb::Key::NumPad6 => Key::NumPad6,
            minifb::Key::NumPad7 => Key::NumPad7,
            minifb::Key::NumPad8 => Key::NumPad8,
            minifb::Key::NumPad9 => Key::NumPad9,
            minifb::Key::NumPadDot => Key::NumPadDot,
            minifb::Key::NumPadSlash => Key::NumPadSlash,
            minifb::Key::NumPadAsterisk => Key::NumPadAsterisk,
            minifb::Key::NumPadMinus => Key::NumPadMinus,
            minifb::Key::NumPadPlus => Key::NumPadPlus,
            minifb::Key::NumPadEnter => Key::NumPadEnter,

            minifb::Key::LeftAlt => Key::LeftAlt,
            minifb::Key::RightAlt => Key::RightAlt,

            minifb::Key::LeftSuper => Key::LeftSuper,
            minifb::Key::RightSuper => Key::RightSuper,

            minifb::Key::Unknown => Key::Unknown,

            minifb::Key::Count => unreachable!("Count is not a valid key")
        }
    }
}

impl Window {
    pub fn render(&mut self, canvas: &mut CanvasImpl) {
        canvas.render_markers();

        // TODO: Do it better
        let buffer: Vec<u32> = canvas.buffer().into_iter().map(|color| color.as_u32()).collect();

        assert_eq!(buffer.len(), self.window.get_size().0 * self.window.get_size().1);

        let (w, h) = canvas.size();
        self.window.update_with_buffer(&buffer, w, h).unwrap();
    }
}

struct KeyboardManagerWrapper {
    manager: Rc<RefCell<KeyboardManager>>,
}

impl minifb::InputCallback for KeyboardManagerWrapper {
    fn add_char(&mut self, _uni_char: u32) {
        // TODO: Handle unicode characters
    }

    fn set_key_state(&mut self, key: minifb::Key, _state: bool) {
        self.manager.borrow_mut().set(key.into(), _state);
    }
}

impl WindowTrait for Window {
    fn new(title: &str, width: usize, height: usize, mut app: impl App) {
        let mut fb_window = minifb::Window::new(title, width, height, minifb::WindowOptions::default()).unwrap();
        // Limit to max ~60 fps update rate
        fb_window.set_target_fps(60);

        let keyboard = Rc::new(RefCell::new(KeyboardManager::new()));
        let mouse = MouseManager::new(fb_window.get_mouse_pos(minifb::MouseMode::Pass).map(|(x, y)| Vec2::new(x as i32, y as i32)).unwrap());

        fb_window.set_input_callback(Box::new(KeyboardManagerWrapper { manager: Rc::clone(&keyboard) }));
        
        let mut window = Self {
            window: fb_window,
            keyboard,
            mouse
        };

        let mut canvas = CanvasImpl::new(width, height);

        while window.is_open() {
            // Set new mouse state
            window.mouse.set_pos(window.window.get_mouse_pos(minifb::MouseMode::Pass).map(|(x, y)| Vec2::new(x as i32, y as i32)).unwrap());
            window.mouse.set(MouseButton::Left, window.window.get_mouse_down(minifb::MouseButton::Left));
            window.mouse.set(MouseButton::Right, window.window.get_mouse_down(minifb::MouseButton::Right));
            window.mouse.set(MouseButton::Middle, window.window.get_mouse_down(minifb::MouseButton::Middle));


            app.update(&mut window);

            app.render(&mut window, &mut canvas);

            // Set current state as old state
            window.keyboard.borrow_mut().update();
            window.mouse.update();

            // Render and wait, keyboard event are handled by callback
            window.render(&mut canvas);            
        }
    }

    fn is_open(&self) -> bool {
        self.window.is_open() && !self.key_is_pressed(Key::Q)
    }

    fn mouse_just_pressed(&self, button: MouseButton) -> bool { self.mouse.just_pressed(button) }
    fn mouse_just_released(&self, button: MouseButton) -> bool { self.mouse.just_released(button) }
    fn mouse_is_pressed(&self, button: MouseButton) -> bool { self.mouse.is_pressed(button) }
    fn mouse_pos(&self) -> Vec2<i32> { self.mouse.pos() }
    fn mouse_delta(&self) -> Vec2<i32> { self.mouse.delta() }

    fn key_just_pressed(&self, key: Key) -> bool { self.keyboard.borrow().just_pressed(key) }
    fn key_just_released(&self, key: Key) -> bool { self.keyboard.borrow().just_released(key) }
    fn key_is_pressed(&self, key: Key) -> bool { self.keyboard.borrow().is_pressed(key) }
}