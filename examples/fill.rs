use frender::prelude::*;

#[path = "common/mod.rs"]
mod common;
use common::App10x;

const WIDTH: usize = 1280;
const HEIGHT: usize = 720;

#[allow(unused)]
fn main() {
    Frender::new("Test", WIDTH, HEIGHT, FillApp::new());
}

pub struct FillApp {}

impl FillApp {
    pub fn new() -> App10x<Self> {
        App10x::new(Self {})
    }
}

impl App for FillApp {
    fn render(&mut self, _window: &mut Window, canvas: &mut Canvas) {
        let (w, h) = canvas.size_i32();

        canvas.circle(5, 20, 5, RED);
        
        let line = Line::new(1, 1, w - 2, h - 2);
        canvas.draw(line, WHITE);
        canvas.marker(line, RED.middle());
    
        let line = Line::new(1, h - 2, w - 2, 1);
        canvas.draw(line, WHITE);
        canvas.marker(line, RED.middle());
    }
}