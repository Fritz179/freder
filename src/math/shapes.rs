use crate::prelude::*;

pub mod point;
pub mod line;
pub mod rect;
pub mod circle;
pub mod triangle;

// TODO: Ellipse?

pub trait Overlaps {
    fn overlaps(&self, other: &dyn Overlaps) -> bool;

    fn overlaps_with_line(&self, rect: &Line) -> bool;
    fn overlaps_with_rect(&self, rect: &Rect) -> bool;
}

pub trait Encloses {
    fn encloses(&self, other: &dyn Encloses) -> bool;

    fn encloses_line(&self, rect: &Line) -> bool;
    fn encloses_rect(&self, rect: &Rect) -> bool;
}

pub trait Clip {
    fn clip_rect(self, rect: &Rect) -> Option<Self> where Self: Sized;
}

impl Clip for Line {
    fn clip_rect(self, _rect: &Rect) -> Option<Self> {
        unimplemented!()
    }
}