pub struct Point {
    pub x: i32,
    pub y: i32,
}

pub struct Nomad {
    position: Point,
}

impl Nomad {
    pub fn new() -> Self {
        return Nomad {
            position: Point { x: 0, y: 0 },
        };
    }

    pub fn get_position(&self) -> Point {
        return Point {
            x: self.position.x,
            y: self.position.y,
        };
    }
}

pub struct Animal {
    position: Point,
}

impl Animal {
    pub fn new() -> Self {
        return Animal {
            position: Point { x: 0, y: 0 },
        };
    }

    pub fn get_position(&self) -> Point {
        return Point {
            x: self.position.x,
            y: self.position.y,
        };
    }
}
