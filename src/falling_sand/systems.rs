use bevy::{
    asset::{Assets, Handle},
    ecs::system::{Query, Res, ResMut},
    render::texture::Image,
};
use rand::Rng;

use super::{
    element::Element,
    rules::{to_rule_state, FallingSandRules},
    Chunk,
};

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
                let end_state = rules.get_result(start_state);

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
                    if let Some(element) = chunk.cells.get(x + y * chunk.width) {
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
