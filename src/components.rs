use crate::vec2;
use sfml::graphics::{CircleShape, Color, Transformable};
use sfml::system::Vector2f;

#[derive(Clone, Debug)]
pub struct Transform {
    pub(crate) position: vec2::Vec2,
    speed: vec2::Vec2,
    angle: f32,
}

impl Transform {
    pub fn new() -> Transform {
        Transform {
            position: vec2::Vec2::new(0.0, 0.0),
            speed: vec2::Vec2::new(0.0, 0.0),
            angle: 0.0,
        }
    }

    pub fn new_with_value(pos: vec2::Vec2, spd: vec2::Vec2, ngl: f32) -> Transform {
        Transform {
            position: pos,
            speed: spd,
            angle: ngl,
        }
    }
    
    pub fn set_position(&mut self, x: f32, y: f32) {
        self.position = vec2::Vec2::new(x, y);
    }
}

#[derive(Clone, Debug)]
pub struct Lifespan {
    lifespan: f32,
    remaining: f32,
}

impl Lifespan {
    pub fn new() -> Lifespan {
        Lifespan {
            lifespan: 5.0,
            remaining: 5.0,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Input {
    up: bool,
    down: bool,
    left: bool,
    right: bool,
    shoot: bool,
    special: bool,
}

impl Input {
    pub fn new() -> Input {
        Input {
            up: false,
            down: false,
            left: false,
            right: false,
            shoot: false,
            special: false,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Shape {
    pub(crate) radius: f32,
    pub(crate) points: usize,
    pub(crate) origin: vec2::Vec2,
    pub(crate) color: Color,
}

impl Shape {
    pub fn init() -> Shape {
        Shape {
            radius: 50.0,
            points: 32,
            origin: vec2::Vec2::new(0.0, 0.0),
            color: Color::WHITE,
        }
    }
    
    pub fn set_radius(&mut self, radius: f32) {
        self.radius = radius;
    }
    
    pub fn set_points(&mut self, points: usize) {
        self.points = points;
    }
}

#[derive(Clone, Debug)]
pub struct Score {
    score: u32,
}

impl Score {
    pub fn new() -> Score {
        Score { score: 0 }
    }

    pub fn get_score(&self) -> u32 {
        self.score
    }

    pub fn set_score(&mut self, score: u32) {
        self.score = score
    }

    pub fn add_score(&mut self, score: u32) {
        self.score += score
    }
}

pub struct Position {
    pub(crate) x: f32,
    pub(crate) y: f32,
}

pub struct Velocity {
    pub(crate) x: f32,
    pub(crate) y: f32,
}

pub struct Player {
    pub(crate) health: f32,
}

pub struct Enemy {
    pub(crate) health: f32,
}
