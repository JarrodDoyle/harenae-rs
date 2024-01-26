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
use rand::random;

pub struct FallingSandPlugin;

impl Plugin for FallingSandPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup, setup);
        app.add_systems(Update, update_chunk_texture_system);
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

    let image_handle = images.add(image);

    commands
        .spawn(Chunk {
            width: 256,
            height: 256,
        })
        .insert(SpriteBundle {
            texture: image_handle,
            transform: Transform::from_translation(Vec3::new(256., 0., 0.)),
            ..default()
        });
}

#[derive(Component)]
pub struct Chunk {
    width: usize,
    height: usize,
}

pub fn update_chunk_texture_system(
    mut images: ResMut<Assets<Image>>,
    mut chunk: Query<(&mut Chunk, &Handle<Image>)>,
) {
    // We know for now there's only one chunk
    let chunk = chunk.get_single_mut();
    if chunk.is_err() {
        return;
    }

    let (chunk, image_handle) = chunk.unwrap();
    let image = images.get_mut(image_handle).unwrap();
    for y in 0..chunk.height {
        for x in 0..chunk.width {
            // Just set each pixel to random colours for now
            let index = (x + y * chunk.width) * 4;
            image.data[index] = random::<u8>();
            image.data[index + 1] = random::<u8>();
            image.data[index + 2] = random::<u8>();
            image.data[index + 3] = 255;
        }
    }
}
