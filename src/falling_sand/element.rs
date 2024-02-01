#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Element {
    None,
    Air,
    Sand,
    Water,
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
