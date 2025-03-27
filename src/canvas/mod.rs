pub mod color;
use color::Color;
use draw_commands::{Background, Draw, DrawShape, LineOptions};

use crate::math::{Clip, Line, Rect, Transform, Transformable, Vec2};

pub mod draw_commands;

pub struct View {
        // Where the view is on the context
        clip: Rect,

        // How the view is transformed
        transform: Option<Transform>,
}

pub struct Canvas {
    buffer: Vec<u32>,
    width: usize,
    height: usize,

    view: View,
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Self {
        let clip = Rect::new(0, 0, width as i32, height as i32);

        Self {
            buffer: vec![0; width * height],
            width,
            height,
            view: View {
                clip,
                transform: None,
            },
        }
    }

    pub fn get_buffer(&self) -> &Vec<u32> {
        &self.buffer
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }
}

impl Canvas {
    fn draw_command(&mut self, instance: &mut dyn Draw) {
        instance.draw(self);
    }

    // pub fn draw_command<T: DrawCommand>(&mut self, instance: T) {
    //     instance.draw( self);
    // }

    pub fn draw<T: DrawShape, O: Into<T::Options>>(&mut self, shape: T, options: O) {
        self.draw_command(&mut shape.new_command(options));
    }

    pub fn background(&mut self, color: impl Into<Color>) {
        self.draw_command(&mut Background::new(color));
    }

    pub fn line<O: Into<LineOptions>>(&mut self, x1: i32, y1: i32, x2: i32, y2: i32, options: O) {
        self.draw(Line::new(x1, y1, x2, y2), options);
    }
}