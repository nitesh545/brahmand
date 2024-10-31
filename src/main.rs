/*mod vec2;
mod entity_manager;
mod entity;
mod Components;
mod Game;

use egui_sfml::{egui, DrawInput, SfEgui};
use sfml::graphics::{
    CircleShape, Color, Font, RectangleShape, RenderTarget, RenderWindow, Shape, Text,
    Transformable,
};
use sfml::window::{Event, Key, Style, VideoMode};
use sfml::cpp::FBox;
use std::fmt::Debug;
use std::fs;

#[derive(Debug, Clone)]
struct SShapes {
    text: String,
    pos_x: f32,
    pos_y: f32,
    speed_x: f32,
    speed_y: f32,
    r: u8,
    g: u8,
    b: u8,
    width: f32,
    height: f32,
}

fn get_params_from_file(file_path: String) -> Vec<String> {
    let params = fs::read_to_string(file_path).unwrap();
    let params_array = params.split("\n").collect::<Vec<&str>>();
    let params_strings = params_array
        .into_iter()
        .map(|x| x.to_string().trim().to_string())
        .collect::<Vec<String>>();
    params_strings
}

fn get_entities_from_params(params_strings: Vec<String>) -> Vec<SShapes> {
    let mut entities: Vec<SShapes> = Vec::new();
    for i in 2..params_strings.len() {
        let param_string = params_strings[i]
            .clone()
            .split(' ')
            .collect::<Vec<&str>>()
            .into_iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>();
        if param_string[0] == "Circle" {
            let entity: SShapes = SShapes {
                text: param_string[1].clone(),
                pos_x: param_string[2].parse::<f32>().unwrap(),
                pos_y: param_string[3].parse::<f32>().unwrap(),
                speed_x: param_string[4].parse::<f32>().unwrap(),
                speed_y: param_string[5].parse::<f32>().unwrap(),
                r: param_string[6].parse::<u8>().unwrap(),
                g: param_string[7].parse::<u8>().unwrap(),
                b: param_string[8].parse::<u8>().unwrap(),
                width: param_string[9].parse::<f32>().unwrap(),
                height: 0.0,
            };
            entities.push(entity);
        }
        if param_string[0] == "Rectangle" {
            let entity: SShapes = SShapes {
                text: param_string[1].clone(),
                pos_x: param_string[2].parse::<f32>().unwrap(),
                pos_y: param_string[3].parse::<f32>().unwrap(),
                speed_x: param_string[4].parse::<f32>().unwrap(),
                speed_y: param_string[5].parse::<f32>().unwrap(),
                r: param_string[6].parse::<u8>().unwrap(),
                g: param_string[7].parse::<u8>().unwrap(),
                b: param_string[8].parse::<u8>().unwrap(),
                width: param_string[9].parse::<f32>().unwrap(),
                height: param_string[10].parse::<f32>().unwrap(),
            };
            entities.push(entity);
        }
    }
    entities
}

fn init_shapes(
    entities: Vec<SShapes>,
    font: &FBox<Font>,
) -> (Vec<CircleShape>, Vec<RectangleShape>, Vec<Text>) {
    let mut circles: Vec<CircleShape> = Vec::new();
    let mut rects: Vec<RectangleShape> = Vec::new();
    let mut texts: Vec<Text> = Vec::new();
    for entity in entities.iter() {
        if entity.height == 0.0 {
            let mut circle = CircleShape::new(entity.width, 32);
            circle.set_position((entity.pos_x, entity.pos_y));
            circle.set_fill_color(Color::rgb(entity.r, entity.g, entity.b));
            circles.push(circle);
        } else {
            let mut rect = RectangleShape::new();
            rect.set_size((entity.width, entity.height));
            rect.set_position((entity.pos_x, entity.pos_y));
            rect.set_fill_color(Color::rgb(entity.r, entity.g, entity.b));
            rects.push(rect);
        }
        let text = Text::new(&entity.text, font, 24);
        texts.push(text);
    }
    (circles, rects, texts)
}

fn input(window: &mut FBox<RenderWindow>, sf_egui: &mut SfEgui) {
    while let Some(event) = window.poll_event() {
        sf_egui.add_event(&event);

        if event == Event::Closed {
            window.close();
        }

        if Key::Escape.is_pressed() {
            window.close();
        }
    }
}

fn debug_tools(
    mut window: &mut FBox<RenderWindow>,
    sf_egui: &mut SfEgui,
    entities: &Vec<SShapes>,
) -> DrawInput {
    sf_egui
        .run(&mut window, |_render_window, ctx| {
            egui::Window::new("Debug Tools")
                .default_pos([20.0, 20.0])
                .default_size([300.0, 200.0])
                .show(ctx, |ui| {
                    ui.heading("Debug Information");
                    ui.label(format!("Number of entities: {}", entities.len()));
                    if ui.button("Reset Entities").clicked() {
                        println!("Button Clicked");
                        // Add reset logic here
                    }
                    for entity in entities.iter() {
                        ui.label(format!("{:?}", entity.text));
                        ui.label(format!("speedX: {:?}, speedY: {:?}", entity.speed_x, entity.speed_y));
                        ui.label(format!("posX: {:?}, posY: {:?}", entity.pos_x, entity.pos_y));
                    }
                    // Add more debug UI elements here
                });
        })
        .expect("sf_egui not working nicely")
}

fn render_entities(
    window: &mut FBox<RenderWindow>,
    entities: &mut Vec<SShapes>,
    circles: &mut Vec<CircleShape>,
    rects: &mut Vec<RectangleShape>,
    texts: &Vec<Text>,
    width: u32,
    height: u32,
) {
    for circle in circles.iter() {
        window.draw(circle);
    }

    for rect in rects.iter() {
        window.draw(rect);
    }

    for i in 0..circles.len() {
        let x = circles[i].position().x;
        let y = circles[i].position().y;
        let speed_x = entities[i].speed_x * 10.0;
        let speed_y = entities[i].speed_y * 10.0;
        let mut text = texts[i].clone();
        circles[i].set_position((x + speed_x, y + speed_y));

        text.set_position((
            circles[i].position().x + circles[i].radius() / 4.0,
            circles[i].position().y
                + circles[i].radius() / 2.0
                + (text.character_size() / 2) as f32,
        ));
        window.draw(&text);

        if x < 0.0 {
            entities[i].speed_x *= -1.0;
            circles[i].set_position((1.0 + speed_x, y + speed_y));
        }

        if x >= width as f32 - (2.0 * circles[i].radius()) {
            entities[i].speed_x *= -1.0;
            let radius = circles[i].radius();
            circles[i].set_position((width as f32 - (2.0 * radius) - 1.0, y + speed_y));
        }

        if y < 0.0 {
            entities[i].speed_y *= -1.0;
            circles[i].set_position((x + speed_x, 1.0 + speed_y));
        }

        if y >= height as f32 - (2.0 * circles[i].radius()) {
            entities[i].speed_y *= -1.0;
            let radius = circles[i].radius();
            circles[i].set_position((x + speed_x, height as f32 - (2.0 * radius) - 1.0 + speed_y));
        }
    }

    for i in 0..rects.len() {
        let x = rects[i].position().x;
        let y = rects[i].position().y;
        let speed_x = entities[i + circles.len()].speed_x * 10.0;
        let speed_y = entities[i + circles.len()].speed_y * 10.0;
        let mut text = texts[i + circles.len()].clone();
        rects[i].set_position((x + speed_x, y + speed_y));

        text.set_position((
            rects[i].position().x + rects[i].size().x / 4.0,
            rects[i].position().y + rects[i].size().y / 2.0 - (text.character_size() / 2) as f32,
        ));
        window.draw(&text);

        if x < 0.0 {
            entities[i + circles.len()].speed_x *= -1.0;
            rects[i].set_position((5.0 + speed_x, y + speed_y));
        }

        if x >= width as f32 - (1.0 * rects[i].size().x) {
            entities[i + circles.len()].speed_x *= -1.0;
            let w = rects[i].size().x;
            rects[i].set_position((width as f32 - (2.0 * w) - 1.0, y + speed_y));
        }

        if y < 0.0 {
            entities[i + circles.len()].speed_y *= -1.0;
            rects[i].set_position((x + speed_x, 5.0 + speed_y));
        }

        if y >= height as f32 - (1.0 * rects[i].size().y) {
            entities[i + circles.len()].speed_y *= -1.0;
            let h = rects[i].size().y;
            rects[i].set_position((x + speed_x, height as f32 - (2.0 * h) - 1.0 + speed_y));
        }
    }
}

fn main() {
    let params_strings = get_params_from_file("shapes.txt".to_string());

    let window_params = params_strings[0]
        .split(' ')
        .collect::<Vec<&str>>()
        .into_iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>();

    let width = window_params[1].parse::<u32>().unwrap();
    let height = window_params[2].parse::<u32>().unwrap();

    let pixels = 32;

    let mut entities: Vec<SShapes> = get_entities_from_params(params_strings);

    let mut window = RenderWindow::new(
        VideoMode::new(width, height, pixels),
        "Brahmand",
        Style::DEFAULT,
        &Default::default(),
    )
    .unwrap();

    let mut sf_egui = SfEgui::new(&window);

    window.set_framerate_limit(60);

    let font = Font::from_file("C:/Windows/Fonts/Arial.ttf").unwrap();

    let (mut circles, mut rects, texts) = init_shapes(entities.clone(), &font);

    while window.is_open() {
        input(&mut window, &mut sf_egui);

        let di = debug_tools(&mut window, &mut sf_egui, &entities);

        window.clear(Color::BLACK);

        render_entities(
            &mut window,
            &mut entities,
            &mut circles,
            &mut rects,
            &texts,
            width,
            height,
        );

        sf_egui.draw(di, &mut window, None);
        window.display();
    }
}
*/
mod components;
mod entity;
mod entity_manager;
mod game;
mod vec2;
mod component_manager;

fn main() {
    let mut game = game::Game::new();
    game.run();
}
