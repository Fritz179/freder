pub mod math;
pub mod window;
pub mod canvas;

pub mod prelude;
use std::{cell::Cell, fmt::Debug, marker::PhantomData};

use canvas::draw_commands::{CloneCommand, DrawCommand};
use prelude::*;

// const WIDTH: usize = 640;
// const HEIGHT: usize = 480;

const WIDTH: usize = 1280;
const HEIGHT: usize = 720;

fn main() {
    let (mut window, mut canvas) = Window::new("Test", WIDTH, HEIGHT).unwrap();
    let mut saved = false;

    let mut demos: Vec<Box<dyn FnMut(&Window, &mut Canvas)>> = vec![
        Box::new(draw_lines), 
        Box::new(draw_x10(fill_10x, ())),
        Box::new(draw_x10(shapes, (vec![], None))),
    ];

    while window.is_open() {

        demos[2](&window, &mut canvas);

        window.render(&mut canvas);

        if window.is_key_down(Key::S) != saved {
            saved = !saved;

            if saved {
                canvas.save_image_path("out/test.png");
            }
        }
    }
}

#[derive(Debug)]
struct Builder<const N: usize> {
    points: [Vec2<i32>; N],
}

impl ShapeBuilder for Builder<0> {
    fn commit(self: Box<Self>, position: Vec2) -> (Box<dyn CloneCommand>, Option<Box<dyn ShapeBuilder>>) {
        (self.preview(position), Some(Box::new(Builder::<1> {
            points: [position],
        })))
    }

    fn preview(&self, position: Vec2) -> Box<dyn CloneCommand> {
        Box::new(Line::new_vec(Vec2::zero(), position).into_renderable(RED))
    }
}

impl ShapeBuilder for Builder<1> {
    fn commit(self: Box<Self>, position: Vec2) -> (Box<dyn CloneCommand>, Option<Box<dyn ShapeBuilder>>) {
        (Box::new(Line::new_vec(self.points[0], position).into_renderable(WHITE)), None)
    }

    fn preview(&self, position: Vec2) -> Box<dyn CloneCommand> {
        Box::new(Line::new_vec(self.points[0], position).into_renderable(RED))
    }
}


trait ShapeBuilder: Debug {
    fn preview(&self, position: Vec2) -> Box<dyn CloneCommand>;
    fn commit(self: Box<Self>, position: Vec2) -> (Box<dyn CloneCommand>, Option<Box<dyn ShapeBuilder>>);
}

static mut pressed: bool = false;
fn shapes(window: &Window, canvas: &mut Canvas, _w: i32, _h: i32, data: &mut (Vec<Box<dyn CloneCommand>>, Option<Box<dyn ShapeBuilder>>)) {
    let (data, builder) = data;

    if builder.is_none() {
        *builder = Some(Box::new(Builder::<0> {
            points: [],
        }));
    }

    let mouse_pos = window.get_mouse_pos();
    let mouse_down = window.is_mouse_down();

    if let Some(current) = builder.take() {
        if mouse_down && !unsafe { pressed } {
            let (command, new) = current.commit(mouse_pos.unwrap());
            *builder = new;

            if builder.is_none() {
                data.push(command);
            }
        } else {
            let mut command = current.preview(mouse_pos.unwrap());
            command.render(canvas);
            builder.replace(current);
        }
    }

    unsafe { pressed = mouse_down };

    for x in data.iter() {
        x.clone().render(canvas);
    }
}

fn fill_10x(_window: &Window, canvas: &mut Canvas, w: i32, h: i32, _data: &mut ()) {
    canvas.circle(5, 20, 5, RED);

    let line = Line::new(1, 1, w - 2, h - 2);
    canvas.draw(line, WHITE);
    canvas.marker(line, RED.middle());

    let line = Line::new(1, h - 2, w - 2, 1);
    canvas.draw(line, WHITE);
    canvas.marker(line, RED.middle());
}

fn draw_x10<T>(callback: impl Fn(&Window, &mut Canvas, i32, i32, &mut T), mut data: T) -> impl FnMut(&Window, &mut Canvas) {
    move |window: &Window, canvas: &mut Canvas| {
        canvas.background(0);

        const SCALE: usize = 20;
    
        const W: usize = WIDTH / SCALE;
        const H: usize = HEIGHT / SCALE;
    
        let mut buffer = Canvas::new(W, H);
        buffer.background(0);
    
        // fill_10x(&mut buffer, W as i32, H as i32);
        callback(window, &mut buffer, W as i32, H as i32, &mut data);
    
    
        canvas.image(&buffer, 0, 0, SCALE as i32);
    
        for x in 0..W {
            let x1 = (x * SCALE) as i32;
            let x2 = x1 + SCALE as i32 - 1;
    
            canvas.line(x1, 0, x1, HEIGHT as i32, GRAY);
            canvas.line(x2, 0, x2, HEIGHT as i32, GRAY);
        }
    
        for y in 0..H {
            let y1 = (y * SCALE) as i32;
            let y2 = y1 + SCALE as i32 - 1;
    
            canvas.line(0, y1, WIDTH as i32, y1, GRAY);
            canvas.line(0, y2, WIDTH as i32, y2, GRAY);
        }
    }
}

fn draw_lines(_window: &Window, canvas: &mut Canvas) {
    canvas.background(0);

    const SCALE: i32 = 10;

    const W: i32 = WIDTH as i32 / SCALE;
    const H: i32 = HEIGHT as i32 / SCALE;
    const COLS: i32 = 5;
    const ROWS: i32 = 5;

    let mut buffer = Canvas::new(W as usize, H as usize);
    *canvas.view_mut().transform_mut() = None;


    // Draw grid
    for x in 0..COLS {
        canvas.line(x * (W + 2), 0, x * (W + 2), HEIGHT as i32, GRAY);
        canvas.line(x * (W + 2) + W + 1, 0, x * (W + 2) + W + 1, HEIGHT as i32, GRAY);
    }

    for y in 0..ROWS {
        canvas.line(0, y * (H + 2), WIDTH as i32, y * (H + 2), GRAY);
        canvas.line(0, y * (H + 2) + H + 1, WIDTH as i32, y * (H + 2) + H + 1, GRAY);
    }


    for x in 0..COLS {
        for y in 0..ROWS {
            buffer.background(0);

            buffer.line(0, 0, x * 5, y * 5, WHITE);

            let xp = x * (W + 2) + 1;
            let yp = y * (H + 2) + 1;

            let t = if x == 4 && y == 4 {
                buffer.marker(Line::new(0, 0, x * 5, y * 5), RED);
                Some(Transform2D::new(Vec2::new(-xp * 4, -yp * 4), Vec2::new(5, 5)))
            } else {
                None
            };

            *canvas.view_mut().transform_mut() = t;

            canvas.image(&buffer, xp , yp, 1);
            buffer.clear_markers();

            *canvas.view_mut().transform_mut() = None;

        }
    }
}