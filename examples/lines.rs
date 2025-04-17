use frender::prelude::*;

const WIDTH: usize = 1280;
const HEIGHT: usize = 720;

#[allow(unused)]
fn main() {
    Window::new("Test", WIDTH, HEIGHT, LinesApp::new());
}

pub struct LinesApp {

}

impl LinesApp {
    pub fn new() -> Self {
        Self {}
    }
}

impl App for LinesApp {
    fn render(&mut self, _window: &mut Window, canvas: &mut dyn Canvas) {
        canvas.background(0);
        let mut canvas = canvas.get_context();

        const SCALE: i32 = 10;
    
        let (width, height) = canvas.size_i32();
    
        let w: i32 = width / SCALE;
        let h: i32 = height / SCALE;
        let cols: i32 = 5;
        let rows: i32 = 5;
    
        let mut inner = CanvasImpl::new(w as usize, h as usize);
        let mut buffer = ContextImpl::new_canvas(&mut inner);
        buffer.line(10, 20, 20, 20, RED);
        *canvas.view_mut().transform_mut() = None;
    
    
        // Draw grid
        for x in 0..cols {
            canvas.line(x * (w + 2), 0, x * (w + 2), height, GRAY);
            canvas.line(x * (w + 2) + w + 1, 0, x * (w + 2) + w + 1, height, GRAY);
        }
    
        for y in 0..rows {
            canvas.line(0, y * (h + 2), width, y * (h + 2), GRAY);
            canvas.line(0, y * (h + 2) + h + 1, width, y * (h + 2) + h + 1, GRAY);
        }
    
    
        for x in 0..cols {
            for y in 0..rows {
                buffer.background(0);
    
                buffer.line(0, 0, x * 5, y * 5, WHITE);
    
                let xp = x * (w + 2) + 1;
                let yp = y * (h + 2) + 1;
    
                let t = if x == 4 && y == 4 {
                    buffer.marker(Line::new(0, 0, x * 5, y * 5), RED);
                    Some(Transform2D::new(Vec2::new(-xp * 4, -yp * 4), Vec2::new(5, 5)))
                } else {
                    None
                };
    
                *canvas.view_mut().transform_mut() = t;
    
                canvas.image(&buffer, xp , yp, 1);
                buffer.markers_mut().clear();
    
                *canvas.view_mut().transform_mut() = None;
    
            }
        }
    }
}