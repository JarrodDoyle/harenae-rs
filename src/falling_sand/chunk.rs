use bevy::ecs::{component::Component, system::Res};
use ndarray::Array2;
use rand::Rng;

use crate::util::DirtyRect;

use super::{element::Element, rules::FallingSandRules};

#[derive(Component)]
pub struct Chunk {
    step: usize,
    pub width: usize,
    pub height: usize,
    cells: Array2<Element>,
    pub dirty_rect: DirtyRect,
}

impl Chunk {
    pub fn new(width: usize, height: usize) -> Self {
        let mut initial = Self {
            step: 0,
            width,
            height,
            cells: Array2::from_elem((width + 2, height + 2), Element::None),
            dirty_rect: DirtyRect::default(),
        };

        // Set Main area to air
        for y in 0..height {
            for x in 0..width {
                initial.set_cell(x, y, Element::Air);
            }
        }

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

        let x = x + 1;
        let y = y + 1;

        self.cells[(x, y)] = element;
        self.dirty_rect.add_point(x - 1, y - 1);
    }

    pub fn get_cell(&self, x: usize, y: usize) -> Option<Element> {
        if x >= self.width || y >= self.height {
            return None;
        }

        let x = x + 1;
        let y = y + 1;

        Some(self.cells[(x, y)])
    }

    pub fn update(&mut self, rules: &Res<FallingSandRules>) {
        // We operate on 2x2 blocks of cells. Each update we offset the blocks using a
        // modified Margolus neighbourhood.
        let offsets = [(0, 1), (1, 1), (0, 0), (1, 0)];
        let block_offset = [(0, 0), (1, 1), (0, 1), (1, 0)][self.step];
        self.step = (self.step + 1) % 4;

        // Faster to reuse rather than keep remaking it within the loops
        // Also we directly access the cells here rather than using the helper function
        // because we know the values will never be out of bounds
        let mut in_elements: [Element; 4] = [Element::None; 4];
        for block_y in 0..(self.height / 2) {
            let y = block_y * 2 + block_offset.1 + 1;
            for block_x in 0..(self.width / 2) {
                let x = block_x * 2 + block_offset.0 + 1;

                // Get all the cells in our block and convert them to a rule state for lookup
                // Because our offset can cause cell look-ups to go ourside of the grid we have
                // a default `Element::None`
                for i in 0..offsets.len() {
                    let o = offsets[i];
                    in_elements[i] = self.cells[(x + o.0, y + o.1)];
                }
                let out_elements = rules.get_result(&in_elements);

                // We only need to actually update things if the state changed
                for i in 0..offsets.len() {
                    let o = offsets[i];
                    if in_elements[i] != out_elements[i] {
                        let pos = (x + o.0, y + o.1);
                        self.cells[pos] = out_elements[i];
                        self.dirty_rect.add_point(pos.0 - 1, pos.1 - 1);
                    }
                }
            }
        }
    }
}
