use crate::{canvas::Canvas, math::{One, Transform, Transform2D, Vec2}};

use super::{CloneCommand, Command, DrawShape};

#[derive(Debug)]
pub struct ImageOptions {
    destination: Vec2<i32>,
    scaling: Vec2<i32>,
}

impl ImageOptions {
    pub fn scaling(mut self, scale: i32) -> Self {
        self.scaling = Vec2::new(scale, scale);
        self
    }
}

impl From<Vec2<i32>> for ImageOptions {
    fn from(destination: Vec2<i32>) -> Self {
        Self { 
            destination,
            scaling: Vec2::one(),
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

        let transform = Transform2D::new(options.destination, options.scaling);
        for marker in &mut markers {
            marker.transform(&transform);
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

        if self.options.scaling.is_one() {
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
                for dy in 0..*scale.y() {
                    canvas.pixel_slice_mut(x + i * *scale.x()..x + i * *scale.x() + *scale.x(), y + j * *scale.y() + dy).fill(color);
                }
            }
        }
        
    }
}

impl<'a> Transform for Image<'a> {
    fn transform(&mut self, transform: &dyn crate::math::Transformer<i32, 2>) {
        for marker in &mut self.markers {
            marker.transform(transform);
        }

        self.options.destination.transform(transform);
        self.options.scaling.transform(transform.get_scaling_part().as_ref());
    }
}
