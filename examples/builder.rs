use frender::prelude::*;

#[path = "common/mod.rs"]
mod common;
use common::App10x;

const WIDTH: usize = 1280;
const HEIGHT: usize = 720;

#[allow(unused)]
fn main() {
    Frender::new("Test", WIDTH, HEIGHT, BuilderApp::new());
}

pub struct BuilderApp {
    data: Vec<Box<dyn Render>>,
    builder: Option<Box<dyn ShapeBuilder>>,
    pressed: bool,
}

impl BuilderApp {
    pub fn new() -> App10x<Self> {
        App10x::new(Self {
            data: vec![],
            builder: None,
            pressed: false,
        })
    }
}

impl App for BuilderApp {
    fn render(&mut self, window: &mut Window, canvas: &mut Canvas) {
        let mouse_pos = window.get_mouse_pos().unwrap();
        let mouse_down = window.is_mouse_down();

        self.builder.get_or_insert(Builder::new([mouse_pos]));

        if let Some(mut current) = self.builder.take() {
            if mouse_down && !self.pressed {
                match current.commit() {
                    BuildDecision::Done(command) => {
                        self.data.push(command);
                    }
                    BuildDecision::Continue(new) => {
                        self.builder = Some(new);
                    }
                }
            } else {
                current.update(mouse_pos);
                current.render(canvas);
                self.builder.replace(current);
            }
        }

        self.pressed = mouse_down;

        for x in self.data.iter() {
            x.render(canvas);
        }
    }
}

#[derive(Debug)]
struct Builder<const N: usize> {
    points: [Vec2<i32>; N],
}

impl<const N: usize> Builder<N> {
    fn new(points: [Vec2; N]) -> Box<Self> {
        Box::new(Self {
            points,
        })
    }
}

impl Render for Builder<1> {
    fn render(&self, canvas: &mut Canvas) {
        canvas.draw(Line::new_vec(self.points[0], self.points[0]), RED)
    }
}

impl ShapeBuilder for Builder<1> {
    fn commit(self: Box<Self>) -> BuildDecision {
        BuildDecision::Continue(Builder::new([self.points[0], self.points[0]]))
    }

    fn update(&mut self, position: Vec2) {
        self.points[0] = position;
    }
}


impl Render for Builder<2> {
    fn render(&self, canvas: &mut Canvas) {
        canvas.draw(Line::new_slice(self.points), RED)
    }
}

impl ShapeBuilder for Builder<2> {
    fn commit(self: Box<Self>) -> BuildDecision {
        BuildDecision::Done(self)
    }

    fn update(&mut self, position: Vec2) {
        self.points[1] = position;
    }
}

enum BuildDecision {
    Done(Box<dyn Render>),
    Continue(Box<dyn ShapeBuilder>),
}

trait ShapeBuilder: Render {
    fn update(&mut self, position: Vec2);
    fn commit(self: Box<Self>) -> BuildDecision;
}