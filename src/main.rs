pub mod components;
pub mod plugins;
pub mod resources;
pub mod systems;

use bevy::{prelude::*, render::pass::ClearColor};

// const WINDOW_SIZE: (u32, u32) = (1920, 1080);
const WINDOW_SIZE: (u32, u32) = (2430, 1620);

fn main() {
    App::build()
        .add_resource(WindowDescriptor {
            title: "Dunge".to_string(),
            width: WINDOW_SIZE.0,
            height: WINDOW_SIZE.1,
            vsync: true,
            ..Default::default()
        })
        .add_default_plugins()
        .add_plugin(plugins::window_resize::WindowResizePlugin)
        .add_plugin(GamePlugin)
        .run();
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(systems::setup.system())
            .add_resource(resources::InputState::default())
            .add_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
            .add_system(systems::input.system())
            .add_system(systems::physics.system())
            .add_system(systems::player.system())
            // .add_system(systems::debug.system())
        ;
    }
}
