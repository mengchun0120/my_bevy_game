use crate::game_lib::*;
use crate::game_panel::*;
use crate::play_box::*;
use crate::utils::*;
use bevy::prelude::*;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum AppState {
    #[default]
    Loading,
    Playing,
}

pub fn play_box_active(play_box: Res<PlayBoxRecord>) -> bool {
    play_box.0.is_some()
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

    let window_size = &game_lib.config.window_size;
    window
        .resolution
        .set(window_size.width as f32, window_size.height as f32);

    commands.spawn(Camera2d);

    let game_panel = GamePanel::new(
        &mut commands,
        &game_lib,
        meshes.as_mut(),
        materials.as_mut(),
    );

    let box_config = &game_lib.config.box_config;

    commands.insert_resource(IndexGen::new(
        box_config.play_box_type_count(),
        PLAY_BOX_ROTATE_COUNT,
    ));
    commands.insert_resource(DropDownTimer(Timer::from_seconds(
        game_lib.config.drop_down_interval,
        TimerMode::Repeating,
    )));
    commands.insert_resource(game_lib);
    commands.insert_resource(game_panel);
    commands.insert_resource(PlayBoxRecord(None));

    next_state.set(AppState::Playing);

    info!("Finished setting up game");
}

pub fn reset_play_box(
    mut commands: Commands,
    game_lib: Res<GameLib>,
    game_panel: Res<GamePanel>,
    mut play_box: ResMut<PlayBoxRecord>,
    mut index_gen: ResMut<IndexGen>,
    mut timer: ResMut<DropDownTimer>,
) {
    play_box.0 = PlayBox::new(
        index_gen.as_mut(),
        game_lib.as_ref(),
        &mut commands,
        game_panel.as_ref(),
    );
    timer.0.unpause();
}

pub fn process_input(
    game_lib: Res<GameLib>,
    game_panel: Res<GamePanel>,
    mut play_box: ResMut<PlayBoxRecord>,
    keys: Res<ButtonInput<KeyCode>>,
    mut active_boxes: Query<
        (Entity, &mut Transform, &mut Visibility, &mut BoxPos),
        With<ActiveBox>,
    >,
) {
    let b = play_box.0.as_mut().unwrap();
    if keys.just_pressed(KeyCode::ArrowLeft) {
        try_move_left(b, game_lib.as_ref(), game_panel.as_ref(), &mut active_boxes);
    } else if keys.just_pressed(KeyCode::ArrowRight) {
        try_move_right(b, game_lib.as_ref(), game_panel.as_ref(), &mut active_boxes);
    } else if keys.just_pressed(KeyCode::ArrowUp) {
        try_rotate(b, game_lib.as_ref(), game_panel.as_ref(), &mut active_boxes);
    }
}

pub fn drop_down_play_box(
    mut commands: Commands,
    game_lib: Res<GameLib>,
    mut game_panel: ResMut<GamePanel>,
    mut play_box: ResMut<PlayBoxRecord>,
    time: Res<Time>,
    mut timer: ResMut<DropDownTimer>,
    mut active_boxes: Query<
        (Entity, &mut Transform, &mut Visibility, &mut BoxPos),
        With<ActiveBox>,
    >,
) {
    timer.0.tick(time.delta());

    if timer.0.is_finished() {
        let b = play_box.0.as_mut().unwrap();
        let dest = BoxPos::new(b.pos.row - 1, b.pos.col);
        let index = b.index.clone();
        if game_panel.can_move_to(&dest, &index, game_lib.as_ref()) {
            b.reset(
                &dest,
                &index,
                game_lib.as_ref(),
                game_panel.as_ref(),
                &mut active_boxes,
            );
        } else {
            game_panel.put_down_boxes(&mut commands, &active_boxes);
            timer.0.pause();
            play_box.0 = None;
        }
    }
}

fn try_move_left(
    play_box: &mut PlayBox,
    game_lib: &GameLib,
    game_panel: &GamePanel,
    active_boxes: &mut Query<
        (Entity, &mut Transform, &mut Visibility, &mut BoxPos),
        With<ActiveBox>,
    >,
) {
    let dest = BoxPos::new(play_box.pos.row, play_box.pos.col - 1);
    let index = play_box.index.clone();
    if game_panel.can_move_to(&dest, &index, game_lib) {
        play_box.reset(&dest, &index, game_lib, game_panel, active_boxes);
    }
}

fn try_move_right(
    play_box: &mut PlayBox,
    game_lib: &GameLib,
    game_panel: &GamePanel,
    active_boxes: &mut Query<
        (Entity, &mut Transform, &mut Visibility, &mut BoxPos),
        With<ActiveBox>,
    >,
) {
    let dest = BoxPos::new(play_box.pos.row, play_box.pos.col + 1);
    let index = play_box.index.clone();
    if game_panel.can_move_to(&dest, &index, game_lib) {
        play_box.reset(&dest, &index, game_lib, game_panel, active_boxes);
    }
}

fn try_rotate(
    play_box: &mut PlayBox,
    game_lib: &GameLib,
    game_panel: &GamePanel,
    active_boxes: &mut Query<
        (Entity, &mut Transform, &mut Visibility, &mut BoxPos),
        With<ActiveBox>,
    >,
) {
    let dest = play_box.pos.clone();
    let index = play_box.index.rotate();
    if game_panel.can_move_to(&dest, &index, game_lib) {
        play_box.reset(&dest, &index, game_lib, game_panel, active_boxes);
    }
}
