use frender::prelude::*;

mod lines;
use lines::LinesApp;

mod builder;
use builder::BuilderApp;

mod fill;
use fill::FillApp;

const WIDTH: usize = 1280;
const HEIGHT: usize = 720;

struct AllApp {
    demos: Vec<Box<dyn App>>,
    current: usize,
}

impl App for AllApp {
    fn event(&mut self, window: &mut Window, event: Event) {
        self.demos[self.current].event(window, event);
    }

    fn update(&mut self, window: &mut Window) {
        self.demos[self.current].update(window);
    }

    fn render(&mut self, window: &mut Window, canvas: &mut dyn Canvas) {
        self.demos[self.current].render(window, canvas);

        window.render(canvas);

        if window.key_just_pressed(Key::S) {
            canvas.save_image_path("out/test.png");
        }

        if window.key_just_pressed(Key::N) {
            self.current += 1;
            if self.current >= self.demos.len() {
                self.current = 0;
            }
        }
    }
}

fn main() {
    let app = AllApp {
        demos: vec![
            Box::new(LinesApp::new()), 
            Box::new(FillApp::new()),
            Box::new(BuilderApp::new()),
        ],
        current: 2,
    };

    Window::new("Test", WIDTH, HEIGHT, app);
}