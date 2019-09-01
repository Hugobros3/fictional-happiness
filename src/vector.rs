use std::ops;

pub struct Vector2D {
    pub x: f32,
    pub y: f32
}

impl Vector2D {
    pub fn to_string(self) -> String {
        format!("Vector2D(x: {}, y: {})", self.x, self.y)
    }
}

impl ops::Add<Vector2D> for Vector2D {
    type Output = Vector2D;

    fn add(self, _rhs: Vector2D) -> Vector2D {
        return Vector2D{x: self.x + _rhs.x, y: self.y + _rhs.y};
    }
}

