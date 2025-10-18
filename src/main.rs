mod game_lib;
mod game_panel;
mod my_error;
mod play_box;
mod systems;
mod utils;

use crate::systems::*;
use crate::utils::*;
use bevy::{log::LogPlugin, prelude::*};
use clap::Parser;

fn main() {
    let args = Args::parse();

    let _guard = setup_log(&args.log_path);

    App::new()
        .add_plugins(DefaultPlugins.build().disable::<LogPlugin>())
        .insert_resource(args)
        .init_state::<AppState>()
        .add_systems(Startup, setup_game)
        .add_systems(Update, play_game.run_if(in_state(AppState::Playing)))
        .run();
}
