pub use crate::math::numeric::{Number, Float, Integer, Signed, Unsigned, One, Zero};

pub use crate::math::vector::Vec2;
pub use crate::math::shapes::{line::Line, rect::Rect, triangle::Triangle, circle::Circle};
pub use crate::math::transform::{Transformer, Transform2D, Transform};

// TODO: Define option vs options trait
pub use crate::canvas::{Canvas, color::*, Render, draw_commands::{line::LineOption, image::ImageOptions, circle::CircleOptions}};


pub use crate::window::{Window, Key};