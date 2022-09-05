use crate::color::Color;
use crate::shapes::DrawType;
use crate::{Coord, Graphics};
use mint::Point2;

pub fn triangle_contains(points: [Coord; 3], point: Coord) -> bool {
    let p1 = Coord::new(points[1].x - points[0].x, points[1].y - points[0].y);
    let p2 = Coord::new(points[2].x - points[0].x, points[2].y - points[0].y);
    let q = Coord::new(point.x - points[0].x, point.y - points[0].y);

    let s = q.cross_product(p2) as f32 / p1.cross_product(p2) as f32;
    let t = p1.cross_product(q) as f32 / p1.cross_product(p2) as f32;

    s >= 0.0 && t >= 0.0 && (s + t) <= 1.0
}

pub fn triangle_draw(graphics: &mut Graphics, points: [Coord; 3], draw_type: DrawType) {
    let color = draw_type.color();
    graphics.draw_line(points[0], points[1], color);
    graphics.draw_line(points[1], points[2], color);
    graphics.draw_line(points[0], points[2], color);
    if let DrawType::Fill(_) = draw_type {
        let points = [
            Point2 {
                x: points[0].x as f32,
                y: points[0].y as f32,
            },
            Point2 {
                x: points[1].x as f32,
                y: points[1].y as f32,
            },
            Point2 {
                x: points[2].x as f32,
                y: points[2].y as f32,
            },
        ];
        if points[1].y == points[2].y {
            draw_flat_bottom(graphics, points, color);
        } else if points[0].y == points[1].y {
            draw_flat_top(graphics, points, color);
        } else {
            let p = Point2 {
                x: points[0].x
                    + ((points[1].y - points[0].y) / (points[2].y - points[0].y))
                        * (points[2].x - points[0].x),
                y: points[1].y,
            };
            draw_flat_bottom(graphics, [points[0], points[1], p], color);
            draw_flat_top(graphics, [points[1], p, points[2]], color);
        }
    }
}

pub fn draw_flat_bottom(graphics: &mut Graphics, points: [Point2<f32>; 3], color: Color) {
    let slope1 = (points[1].x - points[0].x) / (points[1].y - points[0].y);
    let slope2 = (points[2].x - points[0].x) / (points[2].y - points[0].y);
    let mut x1 = points[0].x;
    let mut x2 = points[0].x;
    for y in (points[0].y as usize)..(points[1].y as usize) {
        graphics.draw_line((x1 as usize, y), (x2 as usize + 1, y), color);
        x1 += slope1;
        x2 += slope2;
    }
}

pub fn draw_flat_top(graphics: &mut Graphics, points: [Point2<f32>; 3], color: Color) {
    let slope1 = (points[2].x - points[0].x) / (points[2].y - points[0].y);
    let slope2 = (points[2].x - points[1].x) / (points[2].y - points[1].y);
    let mut x1 = points[2].x;
    let mut x2 = points[2].x;
    for y in ((points[0].y as usize)..(points[2].y as usize)).rev() {
        graphics.draw_line((x1 as usize, y), (x2 as usize + 1, y), color);
        x1 -= slope1;
        x2 -= slope2;
    }
}
