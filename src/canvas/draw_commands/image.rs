use crate::prelude::*;

use super::{CloneCommand, Command, DrawCommand};

#[derive(Debug)]
pub struct ImageOption {
    destination: Vec2<i32>,
    scaling: Vec2<i32>,
}

impl ImageOption {
    pub fn scaling(mut self, scale: i32) -> Self {
        self.scaling = Vec2::new(scale, scale);
        self
    }
}

impl From<Vec2<i32>> for ImageOption {
    fn from(destination: Vec2<i32>) -> Self {
        Self { 
            destination,
            scaling: Vec2::one(),
        }
    }
}

#[derive(Debug)]
pub struct ImageCommand<'a> {
    image: &'a ImageImpl,
    options: ImageOption,
    markers: Vec<Box<dyn CloneCommand>>,
}

impl<'a> DrawCommand for &'a ImageImpl {
    type Options = ImageOption;
    type Command = ImageCommand<'a>;

    fn into_renderable(self, options: impl Into<Self::Options>) -> Self::Command {
        let options = options.into();
        let mut markers = self.markers.clone();

        let transform = Transform2D::new(options.destination, options.scaling);
        for marker in &mut markers {
            marker.transform(&transform);
        }

        ImageCommand {
            image: self,
            options: options,
            markers,
        }
    }
}

impl<'a> Command for ImageCommand<'a> {
    fn render(&mut self, canvas: &mut dyn Image) {
        canvas.markers().append(&mut self.markers);

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
                    let x = x + i * *scale.x()..x + i * *scale.x() + *scale.x();
                    canvas.pixels_mut(x, y + j * *scale.y() + dy).color(color);
                }
            }
        }
        
    }
}

impl<'a> Transform for ImageCommand<'a> {
    fn transform(&mut self, transform: &dyn Transformer<i32, 2>) {
        for marker in &mut self.markers {
            marker.transform(transform);
        }

        self.options.destination.transform(transform);
        self.options.scaling.transform(&Transform2D::new_scaling(transform.scaling()));
    }
}
