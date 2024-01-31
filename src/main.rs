mod falling_sand;
mod util;

use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
    window::PresentMode,
};

fn main() {
    let window_plugin = WindowPlugin {
        primary_window: Some(Window {
            title: "Harenae".into(),
            resolution: (1280., 720.).into(),
            present_mode: PresentMode::AutoVsync,
            ..default()
        }),
        ..default()
    };

    App::new()
        .add_plugins(
            DefaultPlugins
                .set(window_plugin)
                .set(ImagePlugin::default_nearest()),
        )
        .add_plugins(FrameTimeDiagnosticsPlugin)
        .add_plugins(LogDiagnosticsPlugin::default())
        .add_plugins(falling_sand::FallingSandPlugin)
        .add_systems(Startup, setup)
        .insert_resource(ClearColor(Color::rgb_u8(45, 42, 46)))
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
