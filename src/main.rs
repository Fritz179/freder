mod math;

mod window;
use window::Window;

mod canvas;
use canvas::{color::*, draw_commands::LineOption};

const WIDTH: usize = 640;
const HEIGHT: usize = 480;

fn main() {
    let (mut window, mut canvas) = Window::new("Test", WIDTH, HEIGHT).unwrap();


    while window.is_open() {

        canvas.background(0);
        canvas.line(10, 10, 35, 20, WHITE.width(8));

        window.render(&canvas);
    }
}
