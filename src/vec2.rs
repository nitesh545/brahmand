#[derive(Debug, Clone)]
pub struct Vec2 {
    pub(crate) x: f32,
    pub(crate) y: f32,
}

impl std::ops::Add for Vec2 {
    type Output = Vec2;

    fn add(self, other: Vec2) -> Vec2 {
        Vec2 {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl std::ops::Sub for Vec2 {
    type Output = Vec2;

    fn sub(self, other: Vec2) -> Vec2 {
        Vec2 {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Vec2 {
    fn add(self, other: Vec2) -> Vec2 {
        Vec2 {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
    fn sub(self, other: Vec2) -> Vec2 {
        Vec2 {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
    fn mul(self, scale: f32) -> Vec2 {
        Vec2 {
            x: self.x * scale,
            y: self.y * scale,
        }
    }
}

impl PartialEq for Vec2 {
    fn eq(&self, other: &Vec2) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Eq for Vec2 {}

impl Vec2 {
    pub fn new(x: f32, y: f32) -> Vec2 {
        Vec2 { x: (x), y: (y) }
    }

    pub fn normalize(self) -> Vec2 {
        let a: f32 = self.x;
        let b: f32 = self.y;
        let len: f32 = self.length();
        Vec2 {
            x: a / len,
            y: b / len,
        }
    }

    pub fn length(self) -> f32 {
        f32::sqrt((self.x * self.x) + (self.y + self.y)).round()
    }
}
