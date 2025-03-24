mod math;
use std::{cell::RefCell, rc::Rc};

use math::{Line, Rect, Transform, Transformable, Vec2};

mod window;
use window::Window;

mod canvas;
use canvas::{color::*, Context, Contextable, Draw, draw_commands::{LineOption, DrawCommandInstance}};

const WIDTH: usize = 640;
const HEIGHT: usize = 480;

fn main() {
    let (mut window, canvas) = Window::new("Test", WIDTH, HEIGHT).unwrap();

    let canvas = Rc::new(RefCell::new(canvas));
    
    let mut context = Context::Canvas(Rc::downgrade(&canvas));

    while window.is_open() {

        context.background(0);
        context.line(10, 10, 35, 20, WHITE.width(8));
        
        window.render(&canvas.borrow());
    }
}
