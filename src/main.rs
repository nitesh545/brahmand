mod game;
use bevy_ecs::prelude::*;
use winit::event_loop::{ActiveEventLoop, EventLoop};

#[derive(Component, Debug)]
struct Name {
    name: String,
}

fn print_name(mut q: Query<(&mut Name)>) {
    for (n) in &mut q {
        println!("{:?}", n);
    }
}

fn main() {
    pollster::block_on(game::run());
    let mut w = World::default();
    let e = w
        .spawn(
            (Name {
                name: "Nitesh".to_string(),
            }),
        )
        .id();
    let mut sch = Schedule::default();
    sch.add_systems(print_name);
    sch.run(&mut w);
}
