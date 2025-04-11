use frender::prelude::*;

pub struct App10x<T: App> {
    inner: T
}

impl<T: App> App10x<T> {
    pub fn new(inner: T) -> Self {
        Self { inner }
    }
}

impl<T: App> App for App10x<T> {
    fn event(&mut self, window: &mut Window, event: Event) {
        self.inner.event(window, event);
    }

    fn update(&mut self, window: &mut Window) {
        self.inner.update(window);
    }

    fn render(&mut self, window: &mut Window, canvas: &mut Canvas) {
        canvas.background(0);

        const SCALE: i32 = 20;
        let (width, height) = canvas.size_i32();
    
        let w = width / SCALE;
        let h = height / SCALE;
    
        let mut buffer = Canvas::new(w as usize, h as usize);
        buffer.background(0);
    

        // Render 10x
        self.inner.render(window, &mut buffer);
    
    
        canvas.image(&buffer, 0, 0, SCALE as i32);
    
        for x in 0..w {
            let x1 = (x * SCALE) as i32;
            let x2 = x1 + SCALE as i32 - 1;
    
            canvas.line(x1, 0, x1, height as i32, GRAY);
            canvas.line(x2, 0, x2, height as i32, GRAY);
        }
    
        for y in 0..h {
            let y1 = (y * SCALE) as i32;
            let y2 = y1 + SCALE as i32 - 1;
    
            canvas.line(0, y1, width as i32, y1, GRAY);
            canvas.line(0, y2, width as i32, y2, GRAY);
        }
    }
}