use crate::game_lib::*;
use crate::game_panel::*;
use crate::play_box::*;
use crate::preview::*;
use crate::utils::*;
use bevy::prelude::*;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum AppState {
    #[default]
    Loading,
    InitBox,
    Playing,
    FastDown,
    Flashing,
    Stopped,
}

pub fn setup_game(
    mut next_state: ResMut<NextState<AppState>>,
    mut commands: Commands,
    args: Res<Args>,
    mut exit_app: MessageWriter<AppExit>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut window: Single<&mut Window>,
) {
    let game_lib = match GameLib::new(
        args.config_path.as_path(),
        meshes.as_mut(),
        materials.as_mut(),
    ) {
        Ok(lib) => lib,
        Err(err) => {
            error!("Failed to initialize GameLib {}", err);
            exit_app.write(AppExit::error());
            return;
        }
    };

    let config = &game_lib.config;

    let window_size = &config.window_size;
    window
        .resolution
        .set(window_size.width as f32, window_size.height as f32);

    commands.spawn(Camera2d);

    let cmd = &mut commands;
    let game_panel = GamePanel::new(cmd, &game_lib, meshes.as_mut(), materials.as_mut());
    let preview = Preview::new(cmd, &game_lib, meshes.as_mut(), materials.as_mut());
    let box_config = &config.box_config;

    commands.insert_resource(IndexGen::new(
        box_config.play_box_type_count(),
        PLAY_BOX_ROTATE_COUNT,
    ));
    commands.insert_resource(DropDownTimer(repeat_timer(config.drop_down_interval)));
    commands.insert_resource(FastDownTimer(CountDownTimer::new(
        config.fast_down_interval,
        config.fast_down_max_steps,
    )));
    commands.insert_resource(FlashFullLineTimer(CountDownTimer::new(
        config.flash_full_line_interval,
        config.flash_full_line_max_count,
    )));
    commands.insert_resource(game_lib);
    commands.insert_resource(game_panel);
    commands.insert_resource(PlayBox::default());
    commands.insert_resource(preview);

    next_state.set(AppState::InitBox);

    info!("Finished setting up game");
}

pub fn reset_play_box(
    mut next_state: ResMut<NextState<AppState>>,
    mut commands: Commands,
    game_lib: Res<GameLib>,
    game_panel: Res<GamePanel>,
    mut play_box: ResMut<PlayBox>,
    mut index_gen: ResMut<IndexGen>,
    mut preview: ResMut<Preview>,
    mut drop_down_timer: ResMut<DropDownTimer>,
) {
    if play_box.is_valid() {
        return;
    }

    if let Some(index) = preview.play_box.index() {
        if let Some(new_pos) = game_panel.init_pos(index, game_lib.as_ref()) {
            play_box.transfer(
                &mut preview.play_box,
                new_pos,
                &game_panel.play_region,
                &mut commands,
                game_lib.as_ref(),
            );
            preview.reset_box(index_gen.as_mut(), &mut commands, game_lib.as_ref());
        } else {
            next_state.set(AppState::Stopped);
            return;
        }
    } else {
        let index = index_gen.rand_box();
        if let Some(new_pos) = game_panel.init_pos(&index, game_lib.as_ref()) {
            play_box.init(
                index,
                new_pos,
                &game_panel.play_region,
                game_lib.as_ref(),
                &mut commands,
            );
            preview.reset_box(index_gen.as_mut(), &mut commands, game_lib.as_ref());
        } else {
            next_state.set(AppState::Stopped);
            return;
        }
    }

    drop_down_timer.0.unpause();
    next_state.set(AppState::Playing);
}

pub fn process_input(
    mut next_state: ResMut<NextState<AppState>>,
    mut commands: Commands,
    game_lib: Res<GameLib>,
    game_panel: Res<GamePanel>,
    mut play_box: ResMut<PlayBox>,
    keys: Res<ButtonInput<KeyCode>>,
    mut fast_down_timer: ResMut<FastDownTimer>,
) {
    if keys.just_pressed(KeyCode::ArrowLeft) {
        try_move_left(
            play_box.as_mut(),
            &mut commands,
            game_lib.as_ref(),
            game_panel.as_ref(),
        );
    } else if keys.just_pressed(KeyCode::ArrowRight) {
        try_move_right(
            play_box.as_mut(),
            &mut commands,
            game_lib.as_ref(),
            game_panel.as_ref(),
        );
    } else if keys.just_pressed(KeyCode::ArrowUp) {
        try_rotate(
            play_box.as_mut(),
            &mut commands,
            game_lib.as_ref(),
            game_panel.as_ref(),
        );
    } else if keys.just_pressed(KeyCode::ArrowDown) {
        start_fast_down(
            next_state.as_mut(),
            play_box.as_mut(),
            game_panel.as_ref(),
            game_lib.as_ref(),
            fast_down_timer.as_mut(),
        );
    }
}

pub fn drop_down_play_box(
    mut next_state: ResMut<NextState<AppState>>,
    mut commands: Commands,
    game_lib: Res<GameLib>,
    mut game_panel: ResMut<GamePanel>,
    mut play_box: ResMut<PlayBox>,
    time: Res<Time>,
    mut drop_down_timer: ResMut<DropDownTimer>,
    mut flash_full_line_timer: ResMut<FlashFullLineTimer>,
) {
    if !play_box.is_valid() {
        return;
    }

    drop_down_timer.0.tick(time.delta());
    if drop_down_timer.0.is_finished() {
        let index = play_box.index().unwrap();
        let new_pos = BoxPos::new(play_box.pos().row - 1, play_box.pos().col);

        if game_panel.can_move_to(&new_pos, index, game_lib.as_ref()) {
            play_box.move_to(
                new_pos,
                &game_panel.play_region,
                game_lib.as_ref(),
                &mut commands,
            );
        } else {
            game_panel.put_down_play_box(play_box.as_mut(), game_lib.as_ref());
            drop_down_timer.0.pause();

            if game_panel.has_full_lines() {
                next_state.set(AppState::Flashing);
                flash_full_line_timer.0.start();
            } else if game_panel.reach_top() {
                next_state.set(AppState::Stopped);
            } else {
                next_state.set(AppState::InitBox);
            }
        }
    }
}

pub fn fast_move_down(
    mut next_state: ResMut<NextState<AppState>>,
    mut commands: Commands,
    game_lib: Res<GameLib>,
    game_panel: Res<GamePanel>,
    mut play_box: ResMut<PlayBox>,
    mut fast_down_timer: ResMut<FastDownTimer>,
    time: Res<Time>,
) {
    if !play_box.is_valid() {
        fast_down_timer.0.stop();
        return;
    }

    let mut stop = false;
    if fast_down_timer.0.update(time.as_ref()) {
        let new_pos = BoxPos::new(play_box.pos().row - 1, play_box.pos().col);
        let index = play_box.index().unwrap();

        if game_panel.can_move_to(&new_pos, index, game_lib.as_ref()) {
            play_box.move_to(
                new_pos,
                &game_panel.play_region,
                game_lib.as_ref(),
                &mut commands,
            );
        } else {
            stop = true;
        }
    }

    if stop || fast_down_timer.0.is_finished() {
        next_state.set(AppState::Playing);
        fast_down_timer.0.stop();
    }
}

pub fn flash_full_rows(
    mut commands: Commands,
    mut next_state: ResMut<NextState<AppState>>,
    mut game_panel: ResMut<GamePanel>,
    game_lib: Res<GameLib>,
    mut flash_full_line_timer: ResMut<FlashFullLineTimer>,
    time: Res<Time>,
) {
    if flash_full_line_timer.0.update(time.as_ref()) {
        game_panel.toggle_full_rows_visibility(&mut commands);
    }

    if flash_full_line_timer.0.is_finished() {
        game_panel.remove_full_rows(&mut commands, game_lib.as_ref());
        flash_full_line_timer.0.stop();
        next_state.set(AppState::InitBox);
    }
}

fn try_move_left(
    play_box: &mut PlayBox,
    commands: &mut Commands,
    game_lib: &GameLib,
    game_panel: &GamePanel,
) {
    if !play_box.is_valid() {
        return;
    }

    let index = play_box.index().unwrap();
    let new_pos = BoxPos::new(play_box.pos().row, play_box.pos().col - 1);

    if game_panel.can_move_to(&new_pos, index, game_lib) {
        play_box.move_to(new_pos, &game_panel.play_region, game_lib, commands);
    }
}

fn try_move_right(
    play_box: &mut PlayBox,
    commands: &mut Commands,
    game_lib: &GameLib,
    game_panel: &GamePanel,
) {
    if !play_box.is_valid() {
        return;
    }

    let index = play_box.index().unwrap();
    let new_pos = BoxPos::new(play_box.pos().row, play_box.pos().col + 1);

    if game_panel.can_move_to(&new_pos, index, game_lib) {
        play_box.move_to(new_pos, &game_panel.play_region, game_lib, commands);
    }
}

fn try_rotate(
    play_box: &mut PlayBox,
    commands: &mut Commands,
    game_lib: &GameLib,
    game_panel: &GamePanel,
) {
    if !play_box.is_valid() {
        return;
    }

    let mut new_index = play_box.index().unwrap().clone();
    new_index.rotate();

    if game_panel.can_move_to(play_box.pos(), &new_index, game_lib) {
        play_box.rotate(&game_panel.play_region, game_lib, commands);
    }
}

fn start_fast_down(
    next_state: &mut NextState<AppState>,
    play_box: &PlayBox,
    game_panel: &GamePanel,
    game_lib: &GameLib,
    fast_down_timer: &mut FastDownTimer,
) {
    if !play_box.is_valid() {
        return;
    }

    let index = play_box.index().unwrap();
    let new_pos = BoxPos::new(play_box.pos().row - 1, play_box.pos().col);
    if game_panel.can_move_to(&new_pos, index, game_lib) {
        next_state.set(AppState::FastDown);
        fast_down_timer.0.start();
    }
}
