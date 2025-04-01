mod math;

mod window;
use math::{Line, Transform2D, Vec2};
use minifb::Key;
use window::Window;

mod canvas;
use canvas::{color::*, draw_commands::LineOption, Canvas};

const WIDTH: usize = 640;
const HEIGHT: usize = 480;

fn main() {
    let (mut window, mut canvas) = Window::new("Test", WIDTH, HEIGHT).unwrap();
    let mut saved = false;

    while window.is_open() {

        // draw_lines(&mut canvas);
        draw_x10(&mut canvas);

        window.render(&mut canvas);

        if window.is_key_down(Key::S) != saved {
            saved = !saved;

            if saved {
                canvas.save_image_path("out/test.png");
            }
        }
    }
}

fn draw_x10(canvas: &mut Canvas) {
    canvas.background(0);

    const SCALE: usize = 20;

    const W: usize = WIDTH / SCALE;
    const H: usize = HEIGHT / SCALE;

    let mut buffer = Canvas::new(W, H);
    buffer.background(0);

    let line = Line::new(1, 1, 30, 22);
    buffer.draw(line, WHITE);
    buffer.marker(line, RED.middle());

    let line = Line::new(1, 22, 30, 1);
    buffer.draw(line, WHITE);
    buffer.marker(line, RED.middle());

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

fn draw_lines(canvas: &mut Canvas) {
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
                Some(Transform2D::new(Vec2::new(-xp * 4, -yp * 4), 5.0))
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