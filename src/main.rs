use bevy::{prelude::*, window::PresentMode};

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
        .add_systems(Startup, setup)
        .insert_resource(ClearColor(Color::rgb_u8(45, 42, 46)))
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(0.25, 0.25, 0.75),
            custom_size: Some(Vec2::new(50.0, 100.0)),
            ..default()
        },
        transform: Transform::from_translation(Vec3::new(-50., 0., 0.)),
        ..default()
    });
}
