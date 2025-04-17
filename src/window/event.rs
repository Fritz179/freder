use crate::prelude::*;

#[derive(Debug, Clone, Copy)]
pub enum Event {
    KeyDown(KeyDownEvent),
    MouseDown(MouseEvent),
    MouseMove(MouseMoveEvent),
    MouseUp(MouseEvent),
    MouseWheel(MouseWheelEvent),
    Quit,
}

#[derive(Debug, Clone, Copy)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
    Back,
    Forward,
}

#[derive(Debug, Clone, Copy)]
pub struct MouseEvent {
    pub position: Vec2,
    pub button: MouseButton,
}

#[derive(Debug, Clone, Copy)]
pub struct MouseMoveEvent {
    pub position: Vec2,
    pub delta: Vec2,
}

#[derive(Debug, Clone, Copy)]
pub struct MouseWheelEvent {
    pub position: Vec2,
    pub direction: bool,
}

#[derive(Debug, Clone, Copy)]
pub struct KeyDownEvent {
    pub repeat: bool,
    pub char: Option<char>,
    pub key: Key,
    pub physical_key: Key,
    pub modifiers: KeyModifiers,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct KeyModifiers {
    bits: u16
}

impl KeyModifiers {
    pub const L_SHIFT   : u16 = 1 << 0;
    pub const R_SHIFT   : u16 = 1 << 1;
    pub const L_CTRL    : u16 = 1 << 2;
    pub const R_CTRL    : u16 = 1 << 3;
    pub const L_ALT     : u16 = 1 << 4;
    pub const R_ALT     : u16 = 1 << 5;
    pub const L_CMD     : u16 = 1 << 6;
    pub const R_CMD     : u16 = 1 << 7;
    pub const NUM_LOCK  : u16 = 1 << 8;
    pub const CAPS_LOCK : u16 = 1 << 9;

    pub fn new(bits: u16) -> Self {
        Self {
            bits
        }
    }

    pub fn bits(&self) -> u16 { self.bits }

    pub fn l_shift(&self) -> bool { self.bits & Self::L_SHIFT != 0 }
    pub fn r_shift(&self) -> bool { self.bits & Self::R_SHIFT != 0 }
    pub fn l_ctrl(&self) -> bool { self.bits & Self::L_CTRL != 0 }
    pub fn r_ctrl(&self) -> bool { self.bits & Self::R_CTRL != 0 }
    pub fn l_alt(&self) -> bool { self.bits & Self::L_ALT != 0 }
    pub fn r_alt(&self) -> bool { self.bits & Self::R_ALT != 0 }
    pub fn l_cmd(&self) -> bool { self.bits & Self::L_CMD != 0 }
    pub fn r_cmd(&self) -> bool { self.bits & Self::R_CMD != 0 }
    pub fn num_lock(&self) -> bool { self.bits & Self::NUM_LOCK != 0 }
    pub fn caps_lock(&self) -> bool { self.bits & Self::CAPS_LOCK != 0 }

    pub fn shift(&self) -> bool { self.l_shift() || self.r_shift() }
    pub fn ctrl(&self) -> bool { self.l_ctrl() || self.r_ctrl() }
    pub fn alt(&self) -> bool { self.l_alt() || self.r_alt() }
    pub fn cmd(&self) -> bool { self.l_cmd() || self.r_cmd() }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Key {
    Key0,
    Key1,
    Key2,
    Key3,
    Key4,
    Key5,
    Key6,
    Key7,
    Key8,
    Key9,

    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,

    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
    F13,
    F14,
    F15,

    Down,
    Left,
    Right,
    Up,
    Apostrophe,
    Backquote,

    Backslash,
    Comma,
    Equal,
    LeftBracket,
    Minus,
    Period,
    RightBracket,
    Semicolon,

    Slash,
    Backspace,
    Delete,
    End,
    Enter,

    Escape,

    Home,
    Insert,
    Menu,

    PageDown,
    PageUp,

    Pause,
    Space,
    Tab,
    NumLock,
    CapsLock,
    ScrollLock,
    LeftShift,
    RightShift,
    LeftCtrl,
    RightCtrl,

    NumPad0,
    NumPad1,
    NumPad2,
    NumPad3,
    NumPad4,
    NumPad5,
    NumPad6,
    NumPad7,
    NumPad8,
    NumPad9,
    NumPadDot,
    NumPadSlash,
    NumPadAsterisk,
    NumPadMinus,
    NumPadPlus,
    NumPadEnter,

    LeftAlt,
    RightAlt,

    LeftSuper,
    RightSuper,

    /// Used when an Unknown key has been pressed
    Unknown,
}
