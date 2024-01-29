#[derive(Debug, Default)]
pub struct DirtyRect {
    dirty: bool,
    rect: (usize, usize, usize, usize),
}

impl DirtyRect {
    pub fn new() -> Self {
        Self {
            dirty: false,
            rect: (usize::MAX, usize::MAX, usize::MIN, usize::MIN),
        }
    }

    pub fn range_x(&self) -> std::ops::RangeInclusive<usize> {
        self.rect.0..=self.rect.2
    }

    pub fn range_y(&self) -> std::ops::RangeInclusive<usize> {
        self.rect.1..=self.rect.3
    }

    pub fn reset(&mut self) {
        self.dirty = false;
        self.rect = (usize::MAX, usize::MAX, usize::MIN, usize::MIN);
    }

    pub fn add_point(&mut self, x: usize, y: usize) {
        if x < self.rect.0 {
            self.rect.0 = x;
            self.dirty = true;
        }
        if x > self.rect.2 {
            self.rect.2 = x;
            self.dirty = true;
        }
        if y < self.rect.1 {
            self.rect.1 = y;
            self.dirty = true;
        }
        if y > self.rect.3 {
            self.rect.3 = y;
            self.dirty = true;
        }
    }

    pub fn is_dirty(&self) -> bool {
        self.dirty
    }
}
