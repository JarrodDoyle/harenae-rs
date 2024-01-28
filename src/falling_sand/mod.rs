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
use rand::Rng;

pub struct FallingSandPlugin;

impl Plugin for FallingSandPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup, setup);
        app.add_systems(
            Update,
            (simulate_chunk_system, update_chunk_texture_system).chain(),
        );
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
            cells: vec![Elements::Air; 256 * 256],
        })
        .insert(SpriteBundle {
            sprite: Sprite {
                flip_y: true,
                ..default()
            },
            texture: image_handle,
            transform: Transform::from_translation(Vec3::new(256., 0., 0.))
                .with_scale(Vec3::new(2., 2., 0.)),
            ..default()
        });
}

#[derive(Component)]
pub struct Chunk {
    width: usize,
    height: usize,
    cells: Vec<Elements>,
}

pub fn simulate_chunk_system(mut chunk: Query<&mut Chunk>) {
    // We know for now there's only one chunk
    let chunk = chunk.get_single_mut();
    if chunk.is_err() {
        return;
    }

    let mut query = chunk.unwrap();
    let chunk = query.as_mut();

    // Place sand
    let frac = chunk.width / 2;
    let x = (chunk.width - frac) / 2 + rand::thread_rng().gen_range(0..frac);
    let y = chunk.height - 1;
    chunk.cells[x + y * chunk.width] = Elements::Sand;

    // Simulate sand
    for y in 0..chunk.height {
        for x in 0..chunk.width {
            let index = x + y * chunk.width;
            let element = chunk.cells.get(index).unwrap();
            match element {
                Elements::Air => {}
                Elements::Sand => {
                    if y != 0 {
                        let b_index = index - chunk.width;
                        let bottom = chunk.cells.get(b_index).unwrap();
                        if *bottom == Elements::Air {
                            chunk.cells.swap(index, b_index);
                            continue;
                        }

                        if x != 0 {
                            let bl_index = b_index - 1;
                            let bottom_left = chunk.cells.get(bl_index).unwrap();
                            if *bottom_left == Elements::Air {
                                chunk.cells.swap(index, bl_index);
                                continue;
                            }
                        }

                        if x != chunk.width - 1 {
                            let br_index = b_index + 1;
                            let bottom_right = chunk.cells.get(br_index).unwrap();
                            if *bottom_right == Elements::Air {
                                chunk.cells.swap(index, br_index);
                                continue;
                            }
                        }
                    }
                }
            }
        }
    }
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
    if let Some(image) = images.get_mut(image_handle) {
        for y in 0..chunk.height {
            for x in 0..chunk.width {
                let mut colour = (0, 0, 0);
                if let Some(element) = chunk.cells.get(x + y * chunk.width) {
                    match element {
                        Elements::Air => colour = (25, 24, 26),
                        Elements::Sand => colour = (255, 216, 102),
                    }
                }

                let index = (x + y * chunk.width) * 4;
                image.data[index] = colour.0;
                image.data[index + 1] = colour.1;
                image.data[index + 2] = colour.2;
                image.data[index + 3] = 255;
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Elements {
    Air,
    Sand,
}
