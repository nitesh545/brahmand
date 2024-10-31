use crate::{components, entity, entity_manager};
use egui_sfml::SfEgui;
use sfml::graphics::{CircleShape, Color, Font, RenderTarget, RenderWindow, Shape, Transformable};
use sfml::window::{Event, Key, Style, VideoMode};
use rand::Rng;
use crate::components::Position;

const WINDOW_WIDTH: u32 = 1280;
const WINDOW_HEIGHT: u32 = 720;

pub struct Game {
    window: sfml::cpp::FBox<RenderWindow>,
    em: entity_manager::EntityManager,
    pause: bool,
    running: bool,
}

impl Game {
    pub fn new() -> Game {
        let win = RenderWindow::new(
            VideoMode::new(WINDOW_WIDTH, WINDOW_HEIGHT, 32),
            "Brahmand",
            Style::DEFAULT,
            &Default::default(),
        )
        .unwrap();
        let ntiti_mngr = entity_manager::EntityManager::new();
        Game {
            window: win,
            pause: false,
            running: true,
            em: ntiti_mngr,
        }
    }

    pub fn input(&mut self) {
        while let Some(event) = self.window.poll_event() {
            match event {
                Event::Closed => {self.running = false; self.window.close();},
                Event::KeyPressed {code, ..} => {
                    match code {
                        Key::Escape => {self.running = false; self.window.close();},
                        // TODO - fix creation of enemies: random position, radius and vertices
                        Key::K => {
                            println!("enemy spawned");
                            let mut en: &mut entity::Entity = self.em.add_entity();
                            en.add_component(components::Position{x: 1000.0, y: 0.0});
                            en.add_component(components::Shape::init());
                            en.add_component(components::Enemy{health: 100.0});
                        },
                        Key::W => {
                            println!("Move forward");
                        }
                        _ => (),
                    }
                }
                _ => (),
            }
        }
    }

    pub fn begin_play(&mut self) {
        let mut en = self.em.add_entity();
        en.add_component(components::Position{x: (WINDOW_WIDTH / 2) as f32, y: (WINDOW_HEIGHT/2) as f32 });
        en.add_component(components::Shape::init());
        en.add_component(components::Velocity{x: 50.0, y: 50.0});
        en.add_component(components::Player{health: 100.0});
    }
    
    pub fn update(&mut self) {
        self.input();
    }

    pub fn render_players(&mut self) {
        // render player
        let players = self.em.get_entities_by_component::<components::Player>();
        for mut entity in players {
            let mut cir = CircleShape::new(0.0, 0);
            match entity.get_component::<components::Shape>() {
                Some(&ref shape) => {
                    cir = CircleShape::new(shape.radius, shape.points);
                    cir.set_fill_color(Color::WHITE);
                    cir.set_position((WINDOW_WIDTH as f32/2.0, WINDOW_HEIGHT as f32/2.0));
                    cir.set_origin((cir.radius()/2.0, cir.radius()/2.0));
                },
                None => (),
            };
            self.window.draw(&cir);
        }
    }
    
    pub fn render_enemies(&mut self) {
        // render enemies
        let enemies = self.em.get_entities_by_component::<components::Enemy>();
        for mut entity in enemies {
            let mut cir = CircleShape::new(0.0, 0);
            match entity.get_component::<components::Shape>() {
                Some(shape) => {
                    cir = CircleShape::new(shape.radius, shape.points-26);
                    cir.set_fill_color(Color::RED);
                    cir.set_origin((cir.radius()/2.0, cir.radius()/2.0));
                },
                None => (),
            };
            match entity.get_component::<components::Position>() {
                Some(position) => {
                    cir.set_position((position.x, position.y));
                },
                None => (),
            };
            self.window.draw(&cir);
        }
    }
    
    pub fn render(&mut self) {
        self.window.clear(Color::BLACK);
        
        self.render_players();
        self.render_enemies();
        
        self.window.display();
    }

    pub fn run(&mut self) {
        self.begin_play();
        while self.running {
            self.update();
            self.render();
        }
    }
}
