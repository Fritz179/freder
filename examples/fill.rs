use frender::prelude::*;

#[path = "common/mod.rs"]
mod common;
use common::ScaledApp;

const WIDTH: usize = 1280;
const HEIGHT: usize = 720;

#[allow(unused)]
fn main() {
    Window::new("Test", WIDTH, HEIGHT, FillApp::new());
}

pub struct FillApp {}

impl FillApp {
    pub fn new() -> ScaledApp<Self> {
        ScaledApp::new(Self {}, 16)
    }
}

impl App for FillApp {
    fn render(&mut self, _window: &mut Window, canvas: &mut dyn Canvas) {
        let (w, h) = canvas.size_i32();

        canvas.circle(5, 20, 5, RED);
        canvas.circle(60, 15, 10, WHITE.fill(RED));
        
        let line = Line::new(1, 1, w - 2, h - 2);
        // canvas.draw(line, WHITE);

        // Disgusting
        canvas.draw(line, WHITE);
        canvas.marker(line, RED.middle());
    
        // let line = Line::new(1, h - 2, w - 2, 1);
        // canvas.draw(line, WHITE);
        // canvas.marker(line, RED.middle());
    }
}