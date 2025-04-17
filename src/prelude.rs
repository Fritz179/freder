pub use crate::math::numeric::{Number, Float, Integer, Signed, Unsigned, One, Zero};

pub use crate::math::vector::Vec2;
pub use crate::math::shapes::{line::Line, rect::Rect, triangle::Triangle, circle::Circle};
pub use crate::math::transform::{Transformer, Transform2D, Transform};

// TODO: Define option vs options trait
pub use crate::canvas::{ImageImpl, color::*, Render, Image, ImageGeneric, Canvas, CanvasGeneric, Coloring};
pub use crate::canvas::draw_commands::{line::LineOptionTrait, circle::CircleOptionTrait};

pub use crate::window::{App, Window, WindowTrait};
pub use crate::window::event::{Event, MouseButton, MouseEvent, MouseMoveEvent, MouseWheelEvent, KeyDownEvent, KeyModifiers, Key};
