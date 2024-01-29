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

use crate::util::DirtyRect;

pub struct FallingSandPlugin;

impl Plugin for FallingSandPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup, setup);
        app.add_systems(
            Update,
            (
                place_sand_system,
                simulate_chunk_system,
                update_chunk_texture_system,
            )
                .chain(),
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

    commands.spawn(Chunk::new(256, 256)).insert(SpriteBundle {
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
    cells: Vec<Element>,
    dirty_rect: DirtyRect,
}

impl Chunk {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            cells: vec![Element::Air; width * height],
            dirty_rect: DirtyRect::default(),
        }
    }

    pub fn set_cell(&mut self, x: usize, y: usize, element: Element) {
        if x >= self.width || y >= self.height {
            return;
        }

        self.cells[x + y * self.width] = element;
        self.dirty_rect.add_point(x, y);
    }

    pub fn swap_cells(&mut self, x0: usize, y0: usize, x1: usize, y1: usize) {
        if x0 >= self.width || y0 >= self.height || x1 >= self.width || y1 >= self.height {
            return;
        }

        let i0 = x0 + y0 * self.width;
        let i1 = x1 + y1 * self.width;
        self.cells.swap(i0, i1);
        self.dirty_rect.add_point(x0, y0);
        self.dirty_rect.add_point(x1, y1);
    }

    pub fn get_cell(&self, x: usize, y: usize) -> Option<&Element> {
        if x >= self.width || y >= self.height {
            return None;
        }

        Some(&self.cells[x + y * self.width])
    }
}

pub fn place_sand_system(mut chunk: Query<&mut Chunk>) {
    // We know for now there's only one chunk
    let chunk = chunk.get_single_mut();
    if chunk.is_err() {
        return;
    }

    let mut chunk = chunk.unwrap();
    let frac = chunk.width / 2;
    let x = (chunk.width - frac) / 2 + rand::thread_rng().gen_range(0..frac);
    let y = chunk.height - 1;
    chunk.set_cell(x, y, Element::Sand);
}

pub fn simulate_chunk_system(mut chunk: Query<&mut Chunk>) {
    // We know for now there's only one chunk
    let chunk = chunk.get_single_mut();
    if chunk.is_err() {
        return;
    }

    let mut chunk = chunk.unwrap();

    // Simulate sand
    for y in 0..chunk.height {
        for x in 0..chunk.width {
            let element = chunk.get_cell(x, y).unwrap();
            match element {
                Element::Air => {}
                Element::Sand => {
                    if y == 0 {
                        continue;
                    }

                    // Bottom
                    if *chunk.get_cell(x, y - 1).unwrap() == Element::Air {
                        chunk.swap_cells(x, y, x, y - 1);
                        continue;
                    }

                    // Bottom left
                    if x != 0 && *chunk.get_cell(x - 1, y - 1).unwrap() == Element::Air {
                        chunk.swap_cells(x, y, x - 1, y - 1);
                        continue;
                    }

                    // Bottom right
                    if x != chunk.width - 1
                        && *chunk.get_cell(x + 1, y - 1).unwrap() == Element::Air
                    {
                        chunk.swap_cells(x, y, x + 1, y - 1);
                        continue;
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

    let (mut chunk, image_handle) = chunk.unwrap();
    if !chunk.dirty_rect.is_dirty() {
        return;
    }

    if let Some(image) = images.get_mut(image_handle) {
        for y in chunk.dirty_rect.range_y() {
            for x in chunk.dirty_rect.range_x() {
                let mut colour = (0, 0, 0);
                if let Some(element) = chunk.cells.get(x + y * chunk.width) {
                    match element {
                        Element::Air => colour = (25, 24, 26),
                        Element::Sand => colour = (255, 216, 102),
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

    chunk.dirty_rect.reset();
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Element {
    Air,
    Sand,
}
