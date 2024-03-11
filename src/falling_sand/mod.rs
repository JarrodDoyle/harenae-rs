mod chunk;
mod element;
pub mod rules;
mod systems;

use bevy::{
    app::{Plugin, Startup},
    asset::Assets,
    ecs::system::{Commands, ResMut},
    math::Vec3,
    prelude::*,
    render::{
        render_resource::{Extent3d, TextureDimension, TextureFormat},
        texture::Image,
    },
    sprite::SpriteBundle,
    transform::components::Transform,
};

use self::{chunk::Chunk, element::Element, rules::FallingSandRules};

pub struct FallingSandPlugin;

impl Plugin for FallingSandPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup, setup);
        app.add_systems(
            Update,
            (
                systems::place_sand,
                systems::simulate_chunk,
                systems::update_chunk_texture,
            )
                .chain(),
        );
        app.init_resource::<FallingSandRules>();
    }
}

fn setup(mut commands: Commands, mut images: ResMut<Assets<Image>>) {
    let image = Image::new_fill(
        Extent3d {
            width: 256,
            height: 256,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        &[25, 24, 26, 255],
        TextureFormat::Rgba8UnormSrgb,
    );

    let sprite_bundle1 = SpriteBundle {
        sprite: Sprite {
            flip_y: true,
            ..default()
        },
        texture: images.add(image.clone()),
        transform: Transform::from_translation(Vec3::new(256., 0., 0.))
            .with_scale(Vec3::new(2., 2., 0.)),
        ..default()
    };
    let mut sprite_bundle2 = sprite_bundle1.clone();
    sprite_bundle2.texture = images.add(image.clone());
    sprite_bundle2.transform =
        Transform::from_translation(Vec3::new(-256., 0., 0.)).with_scale(Vec3::new(2., 2., 0.));

    commands.spawn(Chunk::new(256, 256)).insert(sprite_bundle1);
    commands.spawn(Chunk::new(256, 256)).insert(sprite_bundle2);
}
