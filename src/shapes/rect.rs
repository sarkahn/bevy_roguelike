use std::fmt::Display;

use bevy::math::UVec2;

/// A rectangle on a grid.
///
/// Points contained in the rect can be iterated over.
pub struct Rect {
    pub min: UVec2,
    pub max: UVec2,
}

impl Rect {
    /// Construct a rect from it's position and size.
    pub fn from_position_size(pos: (u32, u32), size: (u32, u32)) -> Self {
        let pos = UVec2::from(pos);
        let size = UVec2::from(size);
        Rect {
            min: pos,
            max: pos + size,
        }
    }

    /// Construct a rect from it's min and max extents.
    pub fn from_extents(min: (u32, u32), max: (u32, u32)) -> Self {
        Rect {
            min: UVec2::from(min),
            max: UVec2::from(max),
        }
    }

    pub fn size(&self) -> (u32, u32) {
        (self.max - self.min).into()
    }

    pub fn width(&self) -> u32 {
        self.max.x - self.min.x
    }

    pub fn height(&self) -> u32 {
        self.max.y - self.min.y
    }

    pub fn set_size(&mut self, new_size: (u32, u32)) {
        let new_size = UVec2::from(new_size);
        self.max = self.min + new_size;
    }

    pub fn position(&self) -> (u32, u32) {
        self.min.into()
    }

    pub fn set_position(&mut self, new_pos: (u32, u32)) {
        let new_pos = UVec2::from(new_pos);
        let size = UVec2::from(self.size());
        self.min = new_pos;
        self.max = new_pos + size;
    }

    pub fn center(&self) -> (u32, u32) {
        let size = UVec2::from(self.size());
        (self.min + size / 2).into()
    }

    /// Move the rect's center without affecting it's position or size.
    pub fn set_center(&mut self, pos: (u32, u32)) {
        let pos = UVec2::from(pos);
        let size = UVec2::from(self.size());
        let pos = pos - size / 2;
        self.set_position(pos.into());
    }

    pub fn overlaps(&self, other: &Rect) -> bool {
        let min = self.min;
        let max = self.max;
        !(max.cmplt(other.min).any() || min.cmpgt(other.max).any())
    }

    /// An iterator over all grid positions contained in the rect.
    pub fn iter(&self) -> RectIterator {
        RectIterator::from_rect(self)
    }
}

impl Display for Rect {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let w = self.width();
        let h = self.height();
        write!(
            f,
            "Rect[pos({},{}) size({},{})]",
            self.min.x, self.min.y, w, h
        )
    }
}

pub struct RectIterator {
    min: UVec2,
    width: u32,
    current: u32,
    length: u32,
}

impl RectIterator {
    pub fn from_rect(rect: &Rect) -> Self {
        RectIterator {
            min: rect.min,
            width: rect.width(),
            current: 0,
            length: rect.width() * rect.height(),
        }
    }
}

impl Iterator for RectIterator {
    type Item = (u32, u32);

    fn next(&mut self) -> Option<Self::Item> {
        for i in self.current..self.length {
            self.current += 1;

            let xy = UVec2::new(i % self.width, i / self.width);

            return Some((self.min + xy).into());
        }
        None
    }
}

#[cfg(test)]
mod test {
    use super::Rect;

    #[test]
    fn init() {
        let rect = Rect::from_position_size((5, 5), (5, 5));
        assert_eq!((5, 5), rect.position());
        assert_eq!((5, 5), rect.size());
    }

    #[test]
    fn iterator() {
        let size = 10;
        let rect = Rect::from_position_size((0, 0), (size, size));

        let points: Vec<(u32, u32)> = rect.iter().collect();

        for x in 0..size {
            for y in 0..size {
                assert!(points.contains(&(x, y)));
            }
        }

        assert_eq!(size * size, points.len() as u32);
    }

    #[test]
    fn overlap() {
        let r1 = Rect::from_extents((0, 0), (10, 10));
        let r2 = Rect::from_extents((5, 5), (10, 10));
        let r3 = Rect::from_extents((100, 100), (10, 10));

        assert!(r1.overlaps(&r2));
        assert!(!r1.overlaps(&r3));
        assert!(r1.overlaps(&r1));

        let r1 = Rect::from_extents((0, 0), (5, 5));
        let r2 = Rect::from_extents((6, 6), (10, 10));

        assert!(!r1.overlaps(&r2));

        let r1 = Rect::from_position_size((24, 12), (6, 8));
        let r2 = Rect::from_position_size((6, 31), (9, 7));

        assert!(!r1.overlaps(&r2));
    }

    #[test]
    fn set_center() {
        let mut r = Rect::from_position_size((0, 0), (10, 10));

        r.set_center((30, 30));

        assert_eq!((30, 30), r.center());
        assert_eq!((25, 25), r.min.into());
        assert_eq!((35, 35), r.max.into());
        assert_eq!((10, 10), r.size());
    }
}
