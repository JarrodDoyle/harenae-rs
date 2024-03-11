use bevy::{
    asset::{Assets, Handle},
    ecs::system::{Query, Res, ResMut},
    render::texture::Image,
};
use rand::Rng;

use super::{element::Element, rules::FallingSandRules, Chunk};

pub fn place_sand(mut query: Query<&mut Chunk>) {
    for mut chunk in &mut query {
        let frac = chunk.width / 2;
        let x = (chunk.width - frac) / 2 + rand::thread_rng().gen_range(0..frac);
        let y = chunk.height - 1;
        chunk.set_cell(x, y, Element::Sand);
    }
}

pub fn simulate_chunk(rules: Res<FallingSandRules>, mut query: Query<&mut Chunk>) {
    for mut chunk in &mut query {
        chunk.update(&rules);
    }
}

pub fn update_chunk_texture(
    mut images: ResMut<Assets<Image>>,
    mut query: Query<(&mut Chunk, &Handle<Image>)>,
) {
    for (mut chunk, image_handle) in &mut query {
        if !chunk.dirty_rect.is_dirty() {
            return;
        }

        if let Some(image) = images.get_mut(image_handle) {
            for y in chunk.dirty_rect.range_y() {
                for x in chunk.dirty_rect.range_x() {
                    let mut colour = (0, 0, 0);
                    if let Some(element) = chunk.get_cell(x, y) {
                        match element {
                            Element::Air => colour = (25, 24, 26),
                            Element::Sand => colour = (255, 216, 102),
                            Element::Water => colour = (120, 220, 232),
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
}
