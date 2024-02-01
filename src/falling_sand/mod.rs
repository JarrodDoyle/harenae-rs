mod element;
mod rules;
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
use rand::Rng;

use crate::util::DirtyRect;

use self::{element::Element, rules::FallingSandRules};

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

#[derive(Component)]
pub struct Chunk {
    step: usize,
    width: usize,
    height: usize,
    cells: Vec<Element>,
    dirty_rect: DirtyRect,
}

impl Chunk {
    pub fn new(width: usize, height: usize) -> Self {
        let mut initial = Self {
            step: 0,
            width,
            height,
            cells: vec![Element::Air; width * height],
            dirty_rect: DirtyRect::default(),
        };

        let max_y = height / rand::thread_rng().gen_range(2..10);
        for y in 0..=max_y {
            for x in 0..width {
                initial.set_cell(x, y, Element::Water);
            }
        }
        initial
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

    pub fn get_cell(&self, x: usize, y: usize) -> Option<Element> {
        if x >= self.width || y >= self.height {
            return None;
        }

        Some(self.cells[x + y * self.width])
    }
}
