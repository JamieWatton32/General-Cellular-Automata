use crate::logic::Cell;



pub const RADIUS: i32 = 4;
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct UserData<T> {
    pub data: T,
}

#[derive(Debug, Clone, PartialEq, Copy)]
pub struct Point2D {
    pub x: i32,
    pub y: i32,
    pub user_data: UserData<Cell>,
}

impl Point2D{
    pub fn intersects(&self, other:&Point2D)-> bool{
        let d:f32 = ((((self.x - other.x)).pow(2) + ((self.y - other.y) ).pow(2)) as f32).sqrt();
        (d as i32) < RADIUS * 2

    }
    
}

#[derive(Debug, Clone, Copy)]
pub struct Rectangle {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}


impl Rectangle {
    pub fn contains(&self, point: &Point2D) -> bool {
        point.x >= self.x - self.width / 2 &&
        point.x <= self.x + self.width / 2 &&
        point.y >= self.y - self.height / 2 &&
        point.y <= self.y + self.height / 2
    }

    pub fn intersects(&self, range: &Rectangle) -> bool {
        !(range.x - range.width / 2 > self.x + self.width / 2 ||
          range.x + range.width / 2 < self.x - self.width / 2 ||
          range.y - range.height / 2 > self.y + self.height / 2 ||
          range.y + range.height / 2 < self.y - self.height / 2)
    }
}

pub struct QuadTree {
    pub boundary: Rectangle,
    capacity: usize,
    points: Vec<Point2D>,
    pub divided: bool,
    pub north_east: Option<Box<QuadTree>>,
    pub north_west: Option<Box<QuadTree>>,
    pub south_east: Option<Box<QuadTree>>,
    pub south_west: Option<Box<QuadTree>>,
}

impl QuadTree {
    pub fn new(boundary: Rectangle, capacity: usize) -> Self {
        QuadTree {
            boundary,
            capacity,
            points: Vec::new(),
            divided: false,
            north_east: None,
            north_west: None,
            south_east: None,
            south_west: None,
        }
    }

    pub fn insert(&mut self, point: Point2D) -> bool {
        if !self.boundary.contains(&point) {
            return false;
        }

        if self.points.len() < self.capacity {
            self.points.push(point);
            return true;
        }

        if !self.divided {
            self.subdivide();
        } else{

            if let Some(ne) = &mut self.north_east {
                if ne.insert(point) {
                    return true;
                }
            }
            if let Some(nw) = &mut self.north_west {
                if nw.insert(point) {
                    return true;
                }
            }
            if let Some(se) = &mut self.south_east {
                if se.insert(point) {
                    return true;
                }
            }
            if let Some(sw) = &mut self.south_west {
                if sw.insert(point) {
                    return true;
                }
            }
        }
        false
    }

    fn subdivide(&mut self) {
        let x = self.boundary.x;
        let y = self.boundary.y;
        let w = self.boundary.width / 2;
        let h = self.boundary.height / 2;

        let ne = Rectangle { x: x + w / 2, y: y - h / 2, width: w, height: h };
        let nw = Rectangle { x: x - w / 2, y: y - h / 2, width: w, height: h };
        let se = Rectangle { x: x + w / 2, y: y + h / 2, width: w, height: h };
        let sw = Rectangle { x: x - w / 2, y: y + h / 2, width: w, height: h };

        self.north_east = Some(Box::new(QuadTree::new(ne, self.capacity)));
        self.north_west = Some(Box::new(QuadTree::new(nw, self.capacity)));
        self.south_east = Some(Box::new(QuadTree::new(se, self.capacity)));
        self.south_west = Some(Box::new(QuadTree::new(sw, self.capacity)));

        self.divided = true;
    }

    pub fn query(&self, range: &Rectangle) -> Vec<Point2D> {
        let mut found = Vec::new();

        if !self.boundary.intersects(range) {
            return found;
        }

        for point in &self.points {
            if range.contains(point) {
                found.push(*point);
            }
        } 

        if self.divided {
            if let Some(ne) = &self.north_east {
                found.append(&mut ne.query(range));
            }
            if let Some(nw) = &self.north_west {
                found.append(&mut nw.query(range));
            }
            if let Some(se) = &self.south_east {
                found.append(&mut se.query(range));
            }
            if let Some(sw) = &self.south_west {
                found.append(&mut sw.query(range));
            }
        }

        found
    }

    pub fn points(&self) -> Vec<Point2D> {
        let mut all_points = self.points.clone();

        if self.divided {
            if let Some(ne) = &self.north_east {
                all_points.append(&mut ne.points());
            }
            if let Some(nw) = &self.north_west {
                all_points.append(&mut nw.points());
            }
            if let Some(se) = &self.south_east {
                all_points.append(&mut se.points());
            }
            if let Some(sw) = &self.south_west {
                all_points.append(&mut sw.points());
            }
        }

        all_points
    }
}
