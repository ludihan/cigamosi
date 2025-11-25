use bevy::prelude::*;

mod editor;
mod game;
mod menu;

const FONT: &[u8] = include_bytes!("../assets/fonts/Iosevka.ttc");

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
enum GameState {
    #[default]
    Menu,
    Editor,
    Level,
    Settings,
    CustomLevel,
    Exit,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(MeshPickingPlugin)
        .add_plugins(game::GamePlugin)
        .run();
}
