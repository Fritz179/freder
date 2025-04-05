pub mod math;
pub mod window;
pub mod canvas;

pub mod prelude;
use prelude::*;

// const WIDTH: usize = 640;
// const HEIGHT: usize = 480;

const WIDTH: usize = 1280;
const HEIGHT: usize = 720;

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

fn circle_midpoint(canvas: &mut Canvas, center: Vec2, radius: i32) {
    let (cx, cy) = center.to_tuple();
    let mut x = 0;
    let mut y = radius;

    // Decision parameter.
    let mut p = 1 - radius;
    let color = RED;

    // Iterate through the first octant.
    while x <= y {

        // Each computed (x,y) produces eight symmetrical points:
        *canvas.pixel_mut(cx + x, cy + y).unwrap() = color;
        *canvas.pixel_mut(cx - x, cy + y).unwrap() = color;
        *canvas.pixel_mut(cx + x, cy - y).unwrap() = color;
        *canvas.pixel_mut(cx - x, cy - y).unwrap() = color;
        *canvas.pixel_mut(cx + y, cy + x).unwrap() = color;
        *canvas.pixel_mut(cx - y, cy + x).unwrap() = color;
        *canvas.pixel_mut(cx + y, cy - x).unwrap() = color;
        *canvas.pixel_mut(cx - y, cy - x).unwrap() = color;

        // Update decision parameter and coordinates.
        if p < 0 {
            p += 2 * x + 3;
        } else {
            p += 2 * (x - y) + 5;
            y -= 1;
        }
        x += 1;
    }
}

fn fill_10x(canvas: &mut Canvas, w: i32, h: i32) {
    circle_midpoint(canvas, Vec2::new(5, 20), 5);

    let line = Line::new(1, 1, w - 2, h - 2);
    canvas.draw(line, WHITE);
    canvas.marker(line, RED.middle());

    let line = Line::new(1, h - 2, w - 2, 1);
    canvas.draw(line, WHITE);
    canvas.marker(line, RED.middle());
}

fn draw_x10(canvas: &mut Canvas) {
    canvas.background(0);

    const SCALE: usize = 20;

    const W: usize = WIDTH / SCALE;
    const H: usize = HEIGHT / SCALE;

    let mut buffer = Canvas::new(W, H);
    buffer.background(0);

    fill_10x(&mut buffer, W as i32, H as i32);

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