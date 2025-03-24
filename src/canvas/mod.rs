pub mod color;
use std::{cell::RefCell, rc::Weak};

use color::Color;
use draw_commands::{Background, DrawCommand, DrawCommandInstance, LineOptions, Marker};

use crate::math::{Clip, Line, Rect, Transform, Transformable};

pub mod draw_commands;

pub struct Canvas {
    buffer: Vec<u32>,
    width: usize,
    height: usize,
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            buffer: vec![0; width * height],
            width,
            height,
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

enum ScalingMode {
    /// Draw on bigger buffer, more details 
    Direct,

    /// Draw on a smaller buffer and then upscale
    Buffered(Canvas, Vec<DrawCommandInstance<Marker>>),
}

pub struct View {
    context: Box<Context>,

    // Where the view is on the context
    screen: Rect,
    
    // How the view is transformed
    transform: Transform,

    // How the view is scaled
    scaling: ScalingMode,
}

pub enum Context {
    Canvas(Weak<RefCell<Canvas>>),
    View(Weak<RefCell<View>>),
}

pub trait Contextable {
    fn draw<T: DrawCommand + Transformable + Clip>(&mut self, shape: T, options: impl Into<T::Options>);

    fn background(&mut self, color: impl Into<Color>) {
        self.draw(Background, color);
    }

    fn line<O: Into<LineOptions>>(&mut self, x1: i32, y1: i32, x2: i32, y2: i32, options: O) {
        self.draw( Line::new(x1, y1, x2, y2), options);
    }
}

impl Contextable for Canvas {
    fn draw<T: DrawCommand>(&mut self, shape: T, options: impl Into<T::Options>) {
        shape.draw(self, options);
    }
}

impl Contextable for View {
    fn draw<T: DrawCommand + Transformable>(&mut self, mut shape: T, options: impl Into<T::Options>) {
        shape.transform(self.transform);

        self.context.draw(shape, options);
    }
} 

impl Contextable for Context {
    fn draw<T: DrawCommand + Transformable>(&mut self, shape: T, options: impl Into<T::Options>) {
        match self {
            Self::Canvas(canvas) => canvas.upgrade().unwrap().borrow_mut().draw(shape, options),
            Self::View(view) => view.upgrade().unwrap().borrow_mut().draw(shape, options),
        }
    }
}

pub trait Draw {
    fn render(&self, canvas: &mut Canvas);
}