mod utils;

use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

#[wasm_bindgen]
impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Point {x, y}
    }
}

#[wasm_bindgen]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Rectangle {
    pub center: Point,
    pub width: i32,
    pub height: i32,
}

#[wasm_bindgen]
impl Rectangle {
    pub fn new(center: Point, width: i32, height: i32) -> Self {
        Rectangle {
            center,
            width,
            height
        }
    }

    pub fn contains(&self, point: &Point) -> bool {
        point.x >= self.center.x - self.width / 2 &&
        point.x <= self.center.x + self.width / 2 &&
        point.y >= self.center.y - self.height / 2 &&
        point.y <= self.center.y + self.height / 2
    }

    pub fn intersects(&self, range: &Rectangle) -> bool {
        !(range.center.x - range.width / 2 > self.center.x + self.width / 2 ||
          range.center.x + range.width / 2 < self.center.x - self.width / 2 ||
          range.center.y - range.height / 2 > self.center.y + self.height / 2 ||
          range.center.y + range.height / 2 < self.center.y - self.height / 2)
    }
}

#[wasm_bindgen]
pub struct QuadTree {
    boundary: Rectangle,
    capacity: usize,
    points: Vec<Point>,
    divided: bool,
    northeast: Option<Box<QuadTree>>,
    northwest: Option<Box<QuadTree>>,
    southeast: Option<Box<QuadTree>>,
    southwest: Option<Box<QuadTree>>,
}

#[wasm_bindgen]
impl QuadTree {
    pub fn subdivide(&mut self) {
        let half_width = self.boundary.width / 2;
        let half_height = self.boundary.height / 2;
        let quarter_width = half_width / 2;
        let quarter_height = half_height / 2;

        let ne = Rectangle {
            center: Point { x: self.boundary.center.x + quarter_width, y: self.boundary.center.y - quarter_height },
            width: half_width,
            height: half_height,
        };
        let nw = Rectangle {
            center: Point { x: self.boundary.center.x - quarter_width, y: self.boundary.center.y - quarter_height },
            width: half_width,
            height: half_height,
        };
        let se = Rectangle {
            center: Point { x: self.boundary.center.x + quarter_width, y: self.boundary.center.y + quarter_height },
            width: half_width,
            height: half_height,
        };
        let sw = Rectangle {
            center: Point { x: self.boundary.center.x - quarter_width, y: self.boundary.center.y + quarter_height },
            width: half_width,
            height: half_height,
        };

        self.northeast = Some(Box::new(QuadTree::new(ne, self.capacity)));
        self.northwest = Some(Box::new(QuadTree::new(nw, self.capacity)));
        self.southeast = Some(Box::new(QuadTree::new(se, self.capacity)));
        self.southwest = Some(Box::new(QuadTree::new(sw, self.capacity)));

    }

    fn query(&self, range: &Rectangle, found_points: &mut Vec<Point>) {
        if !self.boundary.intersects(range) {
            return;
        }

        for point in &self.points {
            if range.contains(point) {
                found_points.push(*point);
            }
        }

        // If this quad can't be divided further, then return
        if !self.divided {
            return;
        }

        // Otherwise, recurse into the children
        if let Some(northeast) = &self.northeast {
            northeast.query(range, found_points);
        }

        if let Some(northwest) = &self.northwest {
            northwest.query(range, found_points);
        }

        if let Some(southeast) = &self.southeast {
            southeast.query(range, found_points);
        }

        if let Some(southwest) = &self.southwest {
            southwest.query(range, found_points);
        }

    }

    fn query_rects(&self, found_rects: &mut Vec<Rectangle>) {
        found_rects.push(self.boundary);

        // If this quad can't be divided further, then return
        if !self.divided {
            return;
        }

        // Otherwise, recurse into the children
        if let Some(northeast) = &self.northeast {
            northeast.query_rects(found_rects);
        }

        if let Some(northwest) = &self.northwest {
            northwest.query_rects(found_rects);
        }

        if let Some(southeast) = &self.southeast {
            southeast.query_rects(found_rects);
        }

        if let Some(southwest) = &self.southwest {
            southwest.query_rects(found_rects);
        }

    }
}

#[wasm_bindgen]
impl QuadTree {
   pub fn new(boundary: Rectangle, capacity: usize) -> Self {
        QuadTree {
            boundary,
            capacity,
            points: Vec::with_capacity(capacity),
            divided: false,
            northeast: None,
            northwest: None,
            southeast: None,
            southwest: None,
        }
   }
    pub fn insert(&mut self, point: Point) -> bool {
            if !self.boundary.contains(&point) {
                return false;
            }

            if self.points.len() < self.capacity {
                self.points.push(point);
                true
            } else {
                if !self.divided {
                    self.subdivide();
                    self.divided = true;
                }

                self.northeast.as_mut().unwrap().insert(point) ||
                self.northwest.as_mut().unwrap().insert(point) ||
                self.southeast.as_mut().unwrap().insert(point) ||
                self.southwest.as_mut().unwrap().insert(point)
            }
        }

    pub fn query_for_js(&self, range: &Rectangle) -> String {
        let mut found_points: Vec<Point> = Vec::new();
        self.query(range, &mut found_points);
        serde_json::to_string(&found_points).unwrap_or_else(|_| "[]".to_string())
    }

    pub fn query_rects_for_js(&self) -> String {
        let mut found_rects: Vec<Rectangle> = Vec::new();
        self.query_rects(&mut found_rects);
        serde_json::to_string(&found_rects).unwrap_or_else(|_| "[]".to_string())
    }
}
