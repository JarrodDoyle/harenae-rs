use super::rules::RuleBuilder;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Element {
    None,
    Air,
    Sand,
    Water,
    ElementCount,
}

impl From<u32> for Element {
    fn from(value: u32) -> Self {
        match value {
            x if x == Element::Air as u32 => Element::Air,
            x if x == Element::Sand as u32 => Element::Sand,
            x if x == Element::Water as u32 => Element::Water,
            _ => Element::None,
        }
    }
}

pub fn update_sand(block: &mut RuleBuilder, me: Element) {
    let swap_elements = [Element::Air, Element::Water];

    let bottom = block.get(0, -1);
    let bottom_left = block.get(-1, -1);
    let bottom_right = block.get(1, -1);

    if swap_elements.contains(&bottom) {
        block.set(0, 0, bottom);
        block.set(0, -1, me);
    } else if swap_elements.contains(&bottom_left) {
        block.set(0, 0, bottom_left);
        block.set(-1, -1, me);
    } else if swap_elements.contains(&bottom_right) {
        block.set(0, 0, bottom_right);
        block.set(1, -1, me);
    }
}

pub fn update_water(block: &mut RuleBuilder, me: Element) {
    let swap_elements = [Element::Air];

    let bottom = block.get(0, -1);
    let bottom_left = block.get(-1, -1);
    let bottom_right = block.get(1, -1);
    let left = block.get(-1, 0);
    let right = block.get(1, 0);

    if swap_elements.contains(&bottom) {
        block.set(0, 0, bottom);
        block.set(0, -1, me);
    } else if swap_elements.contains(&bottom_left) {
        block.set(0, 0, bottom_left);
        block.set(-1, -1, me);
    } else if swap_elements.contains(&bottom_right) {
        block.set(0, 0, bottom_right);
        block.set(1, -1, me);
    } else if swap_elements.contains(&left) {
        block.set(0, 0, left);
        block.set(-1, 0, me);
    } else if swap_elements.contains(&right) {
        block.set(0, 0, right);
        block.set(1, 0, me);
    }
}
