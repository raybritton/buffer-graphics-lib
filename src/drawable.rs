use crate::color::Color;
use graphics_shapes::coord::Coord;
use graphics_shapes::Shape;
#[cfg(feature = "serde_derive")]
use serde::{Deserialize, Serialize};
use crate::shapes::CreateDrawable;

#[cfg_attr(feature = "serde_derive", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum DrawType {
    Stroke(Color),
    Fill(Color),
}

impl DrawType {
    #[inline]
    pub fn color(&self) -> Color {
        *match self {
            DrawType::Stroke(c) => c,
            DrawType::Fill(c) => c,
        }
    }
}

#[inline]
pub fn fill(color: Color) -> DrawType {
    DrawType::Fill(color)
}

#[inline]
pub fn stroke(color: Color) -> DrawType {
    DrawType::Stroke(color)
}

#[derive(Debug)]
pub struct Drawable<T: Clone> {
    obj: T,
    draw_type: DrawType,
    drawing_points: Vec<Coord>,
}

impl<T: Clone> Drawable<T> {
    #[inline]
    pub fn obj(&self) -> &T {
        &self.obj
    }
    #[inline]
    pub fn draw_type(&self) -> DrawType {
        self.draw_type
    }
    #[inline]
    pub fn drawing_points(&self) -> &Vec<Coord> {
        &self.drawing_points
    }
}

impl<T: Clone> Drawable<T> {
    #[inline]
    pub fn new(obj: T, draw_type: DrawType, drawing_points: Vec<Coord>) -> Drawable<T> {
        Self {
            obj,
            draw_type,
            drawing_points,
        }
    }
}

impl<T: Clone> Drawable<T> {
    pub fn with_draw_type(&self, draw_type: DrawType) -> Drawable<T> {
        Drawable::new(self.obj.clone(), draw_type, self.drawing_points.clone())
    }
}

impl<T> Drawable<T> where Self: CreateDrawable<T>, T: Shape + Clone {
    pub fn with_translation<P: Into<Coord>>(&self, delta: P) -> Drawable<T> {
        let moved = self.obj.translate_by(delta);
        Drawable::from_obj(moved, self.draw_type)
    }

    pub fn with_move<P: Into<Coord>>(&self, xy: P) -> Drawable<T> {
        let moved = self.obj.move_to(xy);
        Drawable::from_obj(moved, self.draw_type)
    }

    pub fn with_rotation<P: Into<Coord>>(&self, degrees: isize) -> Drawable<T> {
        let rotated = self.obj.rotate(degrees);
        Drawable::from_obj(rotated, self.draw_type)
    }
}