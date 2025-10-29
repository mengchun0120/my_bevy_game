mod game_lib;
mod game_panel;
mod my_error;
mod play_box;
mod preview;
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
        .add_systems(
            Update,
            reset_play_box.run_if(in_state(AppState::Playing).and(not(play_box_active))),
        )
        .add_systems(
            Update,
            (process_input, drop_down_play_box)
                .run_if(in_state(AppState::Playing).and(play_box_active)),
        )
        .add_systems(
            Update,
            fast_move_down.run_if(in_state(AppState::FastDown).and(play_box_active)),
        )
        .add_systems(Update, flash_full_rows.run_if(in_state(AppState::Flashing)))
        .run();
}
