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
    saved: bool,
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

    fn render(&mut self, window: &mut Window, canvas: &mut Canvas) {
        self.demos[self.current].render(window, canvas);

        window.render(canvas);

        if window.is_key_down(Key::S) != self.saved {
            self.saved = !self.saved;

            if self.saved {
                canvas.save_image_path("out/test.png");
            }
        }
    }
}

fn main() {
    let app = AllApp {
        saved: false,
        demos: vec![
            Box::new(LinesApp::new()), 
            Box::new(FillApp::new()),
            Box::new(BuilderApp::new()),
        ],
        current: 2,
    };

    Frender::new("Test", WIDTH, HEIGHT, app);
}