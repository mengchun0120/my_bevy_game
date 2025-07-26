use bevy::prelude::*;
use bevy::reflect::TypePath;
use serde::Deserialize;
use bevy_common_assets::json::JsonAssetPlugin;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            JsonAssetPlugin::<GameConfig>::new(&["json"]),
        ))
        .init_state::<AppState>()
        .add_systems(Startup, setup)
        .add_systems(Update, load_menu.run_if(in_state(AppState::Loading)))
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let game_config = GameConfigHandle(asset_server.load("boxes.json"));
    commands.insert_resource(game_config);
}

fn load_menu(
    game_config: Res<GameConfigHandle>,
    mut game_configs: ResMut<Assets<GameConfig>>,
    mut state: ResMut<NextState<AppState>>,
    mut window_query: Query<&mut Window>,
) {
    if let Some(config) = game_configs.remove(game_config.0.id()) {
        info!("Config loaded successfully");
        if let Ok(mut window) = window_query.single_mut() {
            window.resolution.set(config.screen.width, config.screen.height);
        }
        state.set(AppState::Menu);
    }
}

#[derive(Deserialize, Debug)]
struct BoxConfig {
    bitmaps: [[[i32; 4]; 4]; 4],
    level: i32,
    color: [f32; 4],
}

#[derive(Deserialize, Debug)]
struct ScreenConfig {
    width: f32,
    height: f32,
}

#[derive(Deserialize, Asset, TypePath, Debug)]
struct GameConfig {
    screen: ScreenConfig,
    boxes: Vec<BoxConfig>,
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
enum AppState {
    #[default]
    Loading,
    Menu,
}

#[derive(Resource)]
struct GameConfigHandle(Handle<GameConfig>);
