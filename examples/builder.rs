use frender::prelude::*;

#[path = "common/mod.rs"]
mod common;
use common::ScaledApp;

const WIDTH: usize = 1280;
const HEIGHT: usize = 720;

#[allow(unused)]
fn main() {
    Window::new("Test", WIDTH, HEIGHT, BuilderApp::new());
}

pub struct BuilderApp {
    data: Vec<Box<dyn Render>>,
    builder: Option<Box<dyn ShapeBuilder>>,
}

impl BuilderApp {
    pub fn new() -> ScaledApp<Self> {
        ScaledApp::new(Self {
            data: vec![],
            builder: None,
        }, 20)
    }
}

impl App for BuilderApp {
    fn render(&mut self, window: &mut Window, canvas: &mut dyn Canvas) {
        let mouse_pos = window.mouse_pos();

        self.builder.get_or_insert(Builder::new([mouse_pos]));

        if let Some(mut current) = self.builder.take() {
            if window.mouse_just_pressed(MouseButton::Left) {
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
                current.render_context(canvas);
                self.builder.replace(current);
            }
        }

        for x in self.data.iter() {
            x.render_context(canvas);
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
    fn render_context(&self, canvas: &mut dyn Canvas) {
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
    fn render_context(&self, canvas: &mut dyn Canvas) {
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