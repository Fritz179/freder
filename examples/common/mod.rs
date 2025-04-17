use frender::prelude::*;

pub struct ScaledApp<T: App> {
    inner: T,
    scale: i32,
}

impl<T: App> ScaledApp<T> {
    pub fn new(inner: T, scale: i32) -> Self {
        Self { inner, scale }
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
        canvas.background(0);

        let scale = self.scale;
        let (width, height) = canvas.size_i32();
    
        let w = width / scale;
        let h = height / scale;
    
        let mut buffer = ImageImpl::new(w as usize, h as usize);
        buffer.background(0);
    

        // Render 10x
        self.inner.render(window, &mut buffer);
    
    
        canvas.image(&buffer, 0, 0, scale as i32);
    
        for x in 0..w {
            let x1 = (x * scale) as i32;
            let x2 = x1 + scale as i32 - 1;
    
            canvas.line(x1, 0, x1, height as i32, GRAY);
            canvas.line(x2, 0, x2, height as i32, GRAY);
        }
    
        for y in 0..h {
            let y1 = (y * scale) as i32;
            let y2 = y1 + scale as i32 - 1;
    
            canvas.line(0, y1, width as i32, y1, GRAY);
            canvas.line(0, y2, width as i32, y2, GRAY);
        }
    }
}