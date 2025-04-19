use frender::{canvas::draw_commands::{CloneCommand, Command}, prelude::*};

pub struct ScaledApp<T: App> {
    inner: T,
    scale: i32,
    testing: bool,
}

impl<T: App> ScaledApp<T> {
    pub fn new(inner: T, scale: i32) -> Self {
        Self {
            inner,
            scale,
            testing: true,
        }
    }
}

#[derive(Debug)]
struct TestingCanvas {
    canvas: CanvasImpl,
    data: Vec<Vec<Color>>
}

impl TestingCanvas {
    fn new(width: usize, height: usize) -> Self {
        let canvas = CanvasImpl::new(width, height);
        let data = vec![vec![]; width * height];
        Self { canvas, data }
    }

    fn move_data(&mut self) {
        let (w, h) = self.canvas.size_i32();

        for x in 0..w {
            for y in 0..h {
                let i = self.canvas.index(x, y).unwrap();
                let color = self.canvas.pixel_mut(x, y).unwrap();
                
                if !color.is_transparent() {
                    self.data[i].push(*color);
                    *color = TRANSPARENT;
                }
            }
        }
    }
}

impl Canvas for TestingCanvas {
    fn buffer(&self) -> &[Color] { self.canvas.buffer() }
    fn buffer_mut(&mut self) -> &mut [Color] { self.move_data(); self.canvas.buffer_mut() }

    fn size(&self) -> (usize, usize) { self.canvas.size() }
    fn size_i32(&self) -> (i32, i32) { self.canvas.size_i32() }

    fn draw_dyn(&mut self, command: &mut dyn Command) {
        command.render_canvas(self);
    }

    fn marker_dyn(&mut self, marker: Box<dyn CloneCommand>) {
        self.canvas.markers_mut().push(marker);
    }

    fn markers_mut(&mut self) -> &mut Vec<Box<dyn CloneCommand>> { self.canvas.markers_mut() }
    fn markers(&self) -> &Vec<Box<dyn CloneCommand>> { self.canvas.markers() }

    fn render_markers(&mut self) {
        let markers = self.markers_mut().clone();

        for mut marker in markers {
            marker.render_canvas(self);
        }

        self.markers_mut().clear();
    }

    fn get_context(&mut self) -> ContextImpl {
        ContextImpl::new_canvas(self)
    }
}

impl<T: App> App for ScaledApp<T> {
    fn event(&mut self, window: &mut Window, event: Event) {
        self.inner.event(window, event);
    }

    fn update(&mut self, window: &mut Window) {
        self.inner.update(window);
    }

    fn render(&mut self, window: &mut Window, canvas: &mut dyn Canvas) {
        if window.key_just_pressed(Key::T) {
            self.testing = !self.testing;
        }

        canvas.background(0);

        let scale = self.scale;
        let (width, height) = canvas.size_i32();
    
        let w = width / scale;
        let h = height / scale;
    
        if !self.testing {
            let mut inner = CanvasImpl::new(w as usize, h as usize);
            let mut buffer = ContextImpl::new_canvas(&mut inner);
            self.inner.render(window, &mut buffer);
            canvas.image(&buffer, 0, 0, scale);
        } else {
            let mut inner = TestingCanvas::new(w as usize, h as usize);
            let mut buffer = ContextImpl::new_canvas(&mut inner);
            self.inner.render(window, &mut buffer);

            inner.move_data();

            // Draw each color in a pixel, separated by dotted lines
            for x in 0..w {
                for y in 0..h {
                    let i = inner.canvas.index(x, y).unwrap();
                    let n = inner.data[i].len() as i32;

                    // No colors
                    if n == 0 {
                        continue;
                    }

                    // Space for each color
                    let availabe = scale - 2;
                    let each = availabe / n;
                    assert!(each > 0, "Each is 0");
                    assert!(each * n <= scale, "Each is too big");

                    // Draw each color
                    for c in 0..n {
                        let color = inner.data[i][c as usize];
                        let x1 = x * scale + c * each + 1;
                        let y1 = y * scale + c * each + 1;
                        let x2 = x1 + each;
                        let y2 = y1 + each;

                        for y in y1..y2 {
                            canvas.pixels_mut(x1..x2, y).color(color);
                        }
                    }
                }
            }
        }
    
        // Lines
        for x in 0..w {
            let x1 = x * scale;
            let x2 = x1 + scale - 1;
    
            canvas.line(x1, 0, x1, height, GRAY);
            canvas.line(x2, 0, x2, height, GRAY);
        }
    
        for y in 0..h {
            let y1 = y * scale;
            let y2 = y1 + scale - 1;
    
            canvas.line(0, y1, width, y1, GRAY);
            canvas.line(0, y2, width, y2, GRAY);
        }
    }
}