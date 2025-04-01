pub mod color;

use std::ops::Range;

use color::Color;
use draw_commands::{Background, CloneCommand, Command, DrawShape, ImageOptions, LineOptions, Render};

use crate::math::{Line, Rect, Transform2D, Transformable, Vec2};

pub mod draw_commands;

#[derive(Debug)]
pub enum ScalingMode {
    // No scaling, lines remain thin
    None,

    // Nearest neighbor scaling 
    Buffer(Box<Canvas>),
}

#[derive(Debug)]
pub struct View {
        // Where the view is on the context
        clip: Rect,

        // How the view is transformed
        transform: Option<Transform2D>,

        scaling_mode: ScalingMode,
}

impl View {
    pub fn transform_mut(&mut self) -> &mut Option<Transform2D> {
        &mut self.transform
    }
}

#[derive(Debug)]
pub struct Canvas {
    buffer: Vec<Color>,
    width: usize,
    height: usize,

    view: View,
    pub markers: Vec<Box<dyn CloneCommand>>,
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Self {
        let clip = Rect::new(0, 0, width as i32, height as i32);

        Self {
            buffer: vec![0.into(); width * height],
            width,
            height,
            view: View {
                clip,
                transform: None,
                scaling_mode: ScalingMode::None,
            },
            markers: Vec::new(),
        }
    }

    pub fn new_buffer(width: usize, height: usize, buffer: Vec<Color>) -> Self {
        assert_eq!(buffer.len(), width * height);

        let clip = Rect::new(0, 0, width as i32, height as i32);

        Self {
            buffer,
            width,
            height,
            view: View {
                clip,
                transform: None,
                scaling_mode: ScalingMode::None,
            },
            markers: Vec::new(),
        }
    }

    pub fn from_image_path(path: &str) -> Self {
        let image = image::open(path).unwrap().to_rgba8();
        let (width, height) = image.dimensions();

        let buffer = image.into_raw();
        
        let buffer = buffer.chunks_exact(4)
            .map(|chunk| u32::from_ne_bytes(chunk.try_into().unwrap()).into())
            .collect();

        Self::new_buffer(width as usize, height as usize, buffer)
    }

    pub fn save_image_path(&self, path: &str) {
        let mut image = image::ImageBuffer::new(self.width as u32, self.height as u32);

        // Iterate over the coordinates and pixels of the image
        for ((x, y, pixel), source) in image.enumerate_pixels_mut().zip(self.buffer.iter()) {
            *pixel = image::Rgb([source.r(), source.g(), source.b()]);
        }

        image.save(path).unwrap();
    }

    pub fn get_buffer(&self) -> &Vec<Color> {
        &self.buffer
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn size(&self) -> (usize, usize) {
        (self.width, self.height)
    }

    pub fn index(&self, x: i32, y: i32) -> Option<usize> {
        if x < 0 || y < 0 {
            return None;
        }

        let (x, y) = (x as usize, y as usize);

        if x >= self.width || y >= self.height {
            return None;
        }

        Some(y * self.width + x)
    }

    pub fn pixel(&self, x: i32, y: i32) -> Option<Color> {
        Some(self.buffer[self.index(x, y)?])
    }

    pub fn pixel_mut(&mut self, x: i32, y: i32) -> Option<&mut Color> {
        let i = self.index(x, y)?;
        Some(&mut self.buffer[i])
    }

    pub fn pixel_slice(&self, x: Range<i32>, y: i32) -> &[Color] {
        let Some(x1) = self.index(x.start, y) else {
            return &[]
        };

        let Some(x2) = self.index(x.end - 1, y) else {
            return &[]
        };

        &self.buffer[x1..=x2]
    }

    pub fn pixel_slice_mut(&mut self, x: Range<i32>, y: i32) -> &mut [Color] {
        let Some(x1) = self.index(x.start, y) else {
            return &mut[]
        };

        let Some(x2) = self.index(x.end - 1, y) else {
            return &mut[]
        };

        &mut self.buffer[x1..=x2]
    }

    pub fn view_mut(&mut self) -> &mut View {
        &mut self.view
    }
}

impl Canvas {
    fn render(&mut self, instance: &dyn Render) {
        instance.render(self);
    }

    pub fn marker<T: DrawShape, O: Into<T::Options>>(&mut self, shape: T, options: O) where <T as DrawShape>::Command: CloneCommand + 'static {
        let mut command = shape.into_renderable(options);

        if let Some(transform) = &self.view.transform {
            command.transform(transform);
        }

        self.markers.push(Box::new(command));
    }

    pub fn clear_markers(&mut self) {
        self.markers.clear();
    }

    pub fn render_markers(&mut self) {
        let markers = std::mem::take(&mut self.markers);

        for mut marker in markers {
            marker.render(self);
        }

        self.clear_markers();
    }

    pub fn draw<T: DrawShape, O: Into<T::Options>>(&mut self, shape: T, options: O) {
        let mut command = shape.into_renderable(options);

        if let Some(transform) = &self.view.transform {
            command.transform(transform);
        }

        command.render(self);
    }

    pub fn background(&mut self, color: impl Into<Color>) {
        self.render(&mut Background::new(color));
    }

    pub fn line<O: Into<LineOptions>>(&mut self, x1: i32, y1: i32, x2: i32, y2: i32, options: O) {
        self.draw(Line::new(x1, y1, x2, y2), options);
    }

    pub fn image(&mut self, image: &Canvas, x: i32, y: i32, scale: i32) {
        self.draw(image, <Vec2 as Into<ImageOptions>>::into(Vec2::new(x, y)).scaling(scale));
    }
}