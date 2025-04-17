use std::ops::Range;

use draw_commands::{background::{Background, BackgroundOptions}, image::ImageOption, line::LineOption, circle::CircleOption, CloneCommand, Command, DrawCommand};

use crate::prelude::*;

pub mod color;
pub mod draw_commands;

pub trait Render {
    fn render(&self, canvas: &mut dyn Image);
}

#[derive(Debug)]
pub struct View {
        // Where the view is on the context
        clip: Rect,

        // How the view is transformed
        transform: Option<Transform2D>,
}

impl View {
    pub fn transform_mut(&mut self) -> &mut Option<Transform2D> {
        &mut self.transform
    }
}

pub trait Coloring {
    fn color(&mut self, color: Color);
}

impl Coloring for Option<&mut [Color]> {
    fn color(&mut self, color: Color) {
        if let Some(pixels) = self {
            pixels.fill(color);
        }
    }
}

impl Coloring for Option<&mut Color> {
    fn color(&mut self, color: Color) {
        if let Some(pixels) = self {
            **pixels = color;
        }
    }
}

#[derive(Debug)]
pub struct SimpleCanvasImpl {
    buffer: Vec<Color>,
    width: usize,
    height: usize,
}

#[derive(Debug)]
pub struct ImageImpl {
    buffer: Vec<Color>,
    width: usize,
    height: usize,

    view: View,
    markers: Vec<Box<dyn CloneCommand>>,
}

pub trait Image {
    fn new(width: usize, height: usize) -> Self where Self: Sized;
    fn new_buffer(width: usize, height: usize, buffer: Vec<Color>) -> Self where Self: Sized;

    fn size(&self) -> (usize, usize);
    fn size_i32(&self) -> (i32, i32);

    fn index(&self, x: i32, y: i32) -> Option<usize> {
        if x < 0 || y < 0 {
            return None;
        }

        let (w, h) = self.size();
        let (x, y) = (x as usize, y as usize);

        if x >= w || y >= h {
            return None;
        }

        Some(y * w + x)
    }

    fn buffer(&self) -> &[Color];
    fn buffer_mut(&mut self) -> &mut [Color];

    fn pixels(&self, x: Range<i32>, y: i32) -> Option<&[Color]> {
        let x1 = self.index(x.start, y)?;
        let x2 = self.index(x.end - 1, y)?;

        Some(&self.buffer()[x1..=x2])
    }

    fn pixels_mut(&mut self, x: Range<i32>, y: i32) -> Option<&mut [Color]> {
        let x1 = self.index(x.start, y)?;
        let x2 = self.index(x.end - 1, y)?;

        Some(&mut self.buffer_mut()[x1..=x2])
    }

    fn pixel(&self, x: i32, y: i32) -> Option<Color> {
        Some(self.buffer()[self.index(x, y)?])
    }

    fn pixel_mut(&mut self, x: i32, y: i32) -> Option<&mut Color> {
        let i = self.index(x, y)?;
        Some(&mut self.buffer_mut()[i])
    }

    fn draw_command(&mut self, command: &mut dyn Command);

    fn markers(&mut self) -> &mut Vec<Box<dyn CloneCommand>>;
    fn render_markers(&mut self);
}

pub trait ImageGeneric: Image {
    fn draw<T: DrawCommand, O: Into<T::Options>>(&mut self, shape: T, options: O);

    fn background(&mut self, color: impl Into<BackgroundOptions>) {
        self.draw(Background, color);
    }

    fn line<O: Into<LineOption>>(&mut self, x1: i32, y1: i32, x2: i32, y2: i32, options: O) {
        self.draw(Line::new(x1, y1, x2, y2), options);
    }

    fn image(&mut self, image: &ImageImpl, x: i32, y: i32, scale: i32) {
        self.draw(image, <Vec2 as Into<ImageOption>>::into(Vec2::new(x, y)).scaling(scale));
    }

    fn circle<O: Into<CircleOption>>(&mut self, x: i32, y: i32, radius: i32, options: O) {
        self.draw(Circle::new(x, y, radius), options);
    }

    fn from_image_path(path: &str) -> Self where Self: Sized{
        let image = image::open(path).unwrap().to_rgba8();
        let (width, height) = image.dimensions();

        let buffer = image.into_raw();
        
        let buffer = buffer.chunks_exact(4)
            .map(|chunk| u32::from_ne_bytes(chunk.try_into().unwrap()).into())
            .collect();

        Self::new_buffer(width as usize, height as usize, buffer)
    }

    fn save_image_path(&self, path: &str) {
        let (w, h) = self.size();
        let mut image = image::ImageBuffer::new(w as u32, h as u32);

        // Iterate over the coordinates and pixels of the image
        for (pixel, source) in image.pixels_mut().zip(self.buffer().iter()) {
            *pixel = image::Rgb([source.r(), source.g(), source.b()]);
        }

        image.save(path).unwrap();
    }
}

pub trait Canvas: Image {
    fn view_mut(&mut self) -> &mut View;
}

pub trait CanvasGeneric: Canvas {
    fn draw<T: DrawCommand, O: Into<T::Options>>(&mut self, shape: T, options: O);
    fn marker<T: DrawCommand, O: Into<T::Options>>(&mut self, shape: T, options: O) where <T as DrawCommand>::Command: CloneCommand + 'static;
}

impl Image for ImageImpl {
    fn new(width: usize, height: usize) -> Self {
        let clip = Rect::new(0, 0, width as i32, height as i32);

        Self {
            buffer: vec![0.into(); width * height],
            width,
            height,
            view: View {
                clip,
                transform: None,
            },
            markers: Vec::new(),
        }
    }

    fn new_buffer(width: usize, height: usize, buffer: Vec<Color>) -> Self {
        assert_eq!(buffer.len(), width * height);

        let clip = Rect::new(0, 0, width as i32, height as i32);

        Self {
            buffer,
            width,
            height,
            view: View {
                clip,
                transform: None,
            },
            markers: Vec::new(),
        }
    }

    fn buffer(&self) -> &[Color] {
        &self.buffer
    }

    fn buffer_mut(&mut self) -> &mut [Color] {
        &mut self.buffer
    }

    fn size(&self) -> (usize, usize) {
        (self.width, self.height)
    }

    fn size_i32(&self) -> (i32, i32) {
        (self.width as i32, self.height as i32)
    }

    fn draw_command(&mut self, command: &mut dyn Command) {
        command.render(self);
    }

    fn markers(&mut self) -> &mut Vec<Box<dyn CloneCommand>> {
        &mut self.markers
    }

    // fn marker<T: DrawCommand, O: Into<T::Options>>(&mut self, shape: T, options: O) where <T as DrawCommand>::Command: CloneCommand + 'static {
    //     let mut command = shape.into_renderable(options);

    //     if let Some(transform) = self.view.transform {
    //         command.transform(&transform);
    //     }

    //     self.markers.push(Box::new(command));
    // }

    fn render_markers(&mut self) {
        let markers = std::mem::take(&mut self.markers);

        for mut marker in markers {
            marker.render(self);
        }

        self.markers().clear();
    }
}

impl<C: Image + ?Sized> ImageGeneric for C {
    fn draw<T: DrawCommand, O: Into<T::Options>>(&mut self, shape: T, options: O) {
        let mut command = shape.into_renderable(options);

        self.draw_command(&mut command);
    }
}

impl<C: Canvas + ?Sized> CanvasGeneric for C {
    fn draw<T: DrawCommand, O: Into<T::Options>>(&mut self, shape: T, options: O) {
        let mut command = shape.into_renderable(options);

        if let Some(transform) = self.view_mut().transform {
            command.transform(&transform);
        }

        self.draw_command(&mut command);
    }

    fn marker<T: DrawCommand, O: Into<T::Options>>(&mut self, shape: T, options: O) where <T as DrawCommand>::Command: CloneCommand + 'static {
        let mut command = shape.into_renderable(options);

        if let Some(transform) = self.view_mut().transform {
            command.transform(&transform);
        }

        self.markers().push(Box::new(command));
    }
}

impl Canvas for ImageImpl {
    fn view_mut(&mut self) -> &mut View {
        &mut self.view
    }

    // fn render(&mut self, instance: &dyn Render) {
    //     instance.render(self);
    // }
}