use std::collections::HashMap;

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
    step: usize,
    width: usize,
    height: usize,
    cells: Vec<Element>,
    dirty_rect: DirtyRect,
    rules: HashMap<u32, u32>,
}

impl Chunk {
    pub fn new(width: usize, height: usize) -> Self {
        // Pre-computed air-sand rules
        let rules = HashMap::from([
            gen_rule(
                (Element::Air, Element::Sand, Element::Air, Element::Air),
                (Element::Air, Element::Air, Element::Air, Element::Sand),
            ),
            gen_rule(
                (Element::Air, Element::Sand, Element::Air, Element::Sand),
                (Element::Air, Element::Air, Element::Sand, Element::Sand),
            ),
            gen_rule(
                (Element::Air, Element::Sand, Element::Sand, Element::Air),
                (Element::Air, Element::Air, Element::Sand, Element::Sand),
            ),
            gen_rule(
                (Element::Sand, Element::Air, Element::Air, Element::Air),
                (Element::Air, Element::Air, Element::Sand, Element::Air),
            ),
            gen_rule(
                (Element::Sand, Element::Air, Element::Air, Element::Sand),
                (Element::Air, Element::Air, Element::Sand, Element::Sand),
            ),
            gen_rule(
                (Element::Sand, Element::Air, Element::Sand, Element::Air),
                (Element::Air, Element::Air, Element::Sand, Element::Sand),
            ),
            gen_rule(
                (Element::Sand, Element::Sand, Element::Air, Element::Air),
                (Element::Air, Element::Air, Element::Sand, Element::Sand),
            ),
            gen_rule(
                (Element::Sand, Element::Sand, Element::Air, Element::Sand),
                (Element::Air, Element::Sand, Element::Sand, Element::Sand),
            ),
            gen_rule(
                (Element::Sand, Element::Sand, Element::Sand, Element::Air),
                (Element::Sand, Element::Air, Element::Sand, Element::Sand),
            ),
        ]);

        Self {
            step: 0,
            width,
            height,
            cells: vec![Element::Air; width * height],
            dirty_rect: DirtyRect::default(),
            rules,
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

    pub fn get_cell(&self, x: usize, y: usize) -> Option<Element> {
        if x >= self.width || y >= self.height {
            return None;
        }

        Some(self.cells[x + y * self.width])
    }

    pub fn get_rule_result(&self, input: u32) -> u32 {
        match self.rules.get(&input) {
            Some(&result) => result,
            None => input,
        }
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

    // Determine which Margolus neighbourhood offset we're using this update
    let offset = if chunk.step == 0 {
        (0, 0)
    } else if chunk.step == 1 {
        (1, 1)
    } else if chunk.step == 2 {
        (0, 1)
    } else {
        (1, 0)
    };
    chunk.step = (chunk.step + 1) % 4;

    // We're operating on 2x2 blocks of cells
    for block_y in 0..(chunk.height / 2) {
        let y = block_y * 2 + offset.1;
        for block_x in 0..(chunk.width / 2) {
            let x = block_x * 2 + offset.0;

            // Get all the cells in our block and convert them to a rule state for lookup
            // Because our offset can cause cell look-ups to go ourside of the grid we have
            // a default `Element::None`
            // Cells are obtained in the order top-left, top-right, bottom-left, bottom-right
            let start_state = to_rule_state((
                chunk.get_cell(x, y + 1).unwrap_or(Element::None),
                chunk.get_cell(x + 1, y + 1).unwrap_or(Element::None),
                chunk.get_cell(x, y).unwrap_or(Element::None),
                chunk.get_cell(x + 1, y).unwrap_or(Element::None),
            ));
            let end_state = chunk.get_rule_result(start_state);

            // We only need to actually update things if the state changed
            // Same ordering as above.
            if start_state != end_state {
                if (start_state & 0xFF000000) != (end_state & 0xFF000000) {
                    chunk.set_cell(x, y + 1, Element::from((end_state >> 24) & 0xFF));
                }
                if (start_state & 0x00FF0000) != (end_state & 0x00FF0000) {
                    chunk.set_cell(x + 1, y + 1, Element::from((end_state >> 16) & 0xFF));
                }
                if (start_state & 0x0000FF00) != (end_state & 0x0000FF00) {
                    chunk.set_cell(x, y, Element::from((end_state >> 8) & 0xFF));
                }
                if (start_state & 0x000000FF) != (end_state & 0x000000FF) {
                    chunk.set_cell(x + 1, y, Element::from(end_state & 0xFF));
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
                        _ => {}
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
    None,
    Air,
    Sand,
}

impl From<u32> for Element {
    fn from(value: u32) -> Self {
        match value {
            x if x == Element::Air as u32 => Element::Air,
            x if x == Element::Sand as u32 => Element::Sand,
            _ => Element::None,
        }
    }
}

fn gen_rule(
    input: (Element, Element, Element, Element),
    output: (Element, Element, Element, Element),
) -> (u32, u32) {
    (to_rule_state(input), to_rule_state(output))
}

fn to_rule_state(input: (Element, Element, Element, Element)) -> u32 {
    ((input.0 as u32) << 24) + ((input.1 as u32) << 16) + ((input.2 as u32) << 8) + input.3 as u32
}
