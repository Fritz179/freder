use crate::{canvas::Canvas, math::{Transformable, Vec2}};

use super::{CloneCommand, Command, DrawShape};

#[derive(Debug)]
pub struct ImageOptions {
    destination: Vec2<i32>,
    scaling: i32,
}

impl ImageOptions {
    pub fn scaling(mut self, scale: i32) -> Self {
        self.scaling = scale;
        self
    }
}

impl From<Vec2<i32>> for ImageOptions {
    fn from(destination: Vec2<i32>) -> Self {
        Self { 
            destination,
            scaling: 1,
        }
    }
}


#[derive(Debug)]
pub struct Image<'a> {
    image: &'a Canvas,
    options: ImageOptions,
    markers: Vec<Box<dyn CloneCommand>>,
}

impl<'a> DrawShape for &'a Canvas {
    type Options = ImageOptions;
    type Command = Image<'a>;

    fn into_renderable(self, options: impl Into<Self::Options>) -> Self::Command {
        let options = options.into();
        let mut markers = self.markers.clone();

        for marker in &mut markers {
            marker.scale(options.scaling as f32);
            marker.translate(options.destination);
        }

        Image {
            image: self,
            options: options,
            markers,
        }
    }
}

impl<'a> Command for Image<'a> {
    fn render(&mut self, canvas: &mut Canvas) {
        canvas.markers.append(&mut self.markers);

        let (width, height) = self.image.size();
        let (x, y) = self.options.destination.as_tuple();

        if self.options.scaling == 1 {
            for i in 0..width as i32 {
                for j in 0..height as i32 {
                    let Some(color) = self.image.pixel(i, j) else { continue };
                    let Some(pixel) = canvas.pixel_mut(x + i, y + j) else { continue };
    
                    *pixel = color;
                }
            }

            return
        }

        let scale = self.options.scaling;
        for i in 0..width as i32 {
            for j in 0..height as i32 {
                let Some(color) = self.image.pixel(i, j) else { continue };
                for dy in 0..scale {
                    canvas.pixel_slice_mut(x + i * scale..x + i * scale + scale, y + j * scale + dy).fill(color);
                }
            }
        }
        
    }
}

impl<'a> Transformable for Image<'a> {
    fn scale(&mut self, factor: f32) {
        for marker in &mut self.markers {
            marker.scale(factor);
        }

        self.options.destination.scale(factor);
        
        let int = factor as i32;
        assert_eq!(int as f32, factor, "We only support integer scaling factors");

        self.options.scaling *= int;
    }

    fn translate(&mut self, offset: Vec2<i32>) {
        for marker in &mut self.markers {
            marker.translate(offset);
        }

        <Vec2 as Transformable>::translate(&mut self.options.destination, offset);
    }
}
