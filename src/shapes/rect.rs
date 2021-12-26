use std::fmt::Display;

use bevy::math::{IVec2};

/// A rectangle on a grid.
///
/// Points contained in the rect can be iterated over.
pub struct Rect {
    pub min: IVec2,
    pub max: IVec2,
}

impl Rect {
    /// Construct a rect from it's position and size.
    pub fn from_position_size(pos: (i32, i32), size: (i32, i32)) -> Self {
        let pos = IVec2::from(pos);
        let size = IVec2::from(size);
        Rect {
            min: pos,
            max: pos + size,
        }
    }

    #[allow(dead_code)]
    /// Construct a rect from it's min and max extents.
    pub fn from_extents(min: (i32, i32), max: (i32, i32)) -> Self {
        Rect {
            min: IVec2::from(min),
            max: IVec2::from(max),
        }
    }

    pub fn size(&self) -> IVec2 {
        self.max - self.min
    }

    pub fn width(&self) -> i32 {
        self.max.x - self.min.x
    }

    pub fn height(&self) -> i32 {
        self.max.y - self.min.y
    }

    #[allow(dead_code)]
    pub fn set_size(&mut self, new_size: (i32, i32)) {
        let new_size = IVec2::from(new_size);
        self.max = self.min + new_size;
    }

    #[allow(dead_code)]
    pub fn position(&self) -> IVec2 {
        self.min
    }

    #[allow(dead_code)]
    pub fn set_position(&mut self, new_pos: (i32, i32)) {
        let new_pos = IVec2::from(new_pos);
        let size = self.size();
        self.min = new_pos;
        self.max = new_pos + size;
    }

    pub fn center(&self) -> IVec2 {
        let size = self.size();
        self.min + size / 2
    }

    #[allow(dead_code)]
    /// Move the rect's center without affecting it's position or size.
    pub fn set_center(&mut self, pos: (i32, i32)) {
        let pos = IVec2::from(pos);
        let size = self.size();
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
    min: IVec2,
    width: i32,
    current: i32,
    length: i32,
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
    type Item = IVec2;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(i) = (self.current..self.length).next() {
            let xy = IVec2::new(i % self.width, i / self.width);
            return Some(self.min + xy);
        }
        None
    }
}

#[cfg(test)]
mod test {
    use bevy::math::IVec2;

    use super::Rect;

    #[test]
    fn init() {
        let rect = Rect::from_position_size((5, 5), (5, 5));
        assert_eq!((5, 5), rect.position().into());
        assert_eq!((5, 5), rect.size().into());
    }

    #[test]
    fn iterator() {
        let size = 10;
        let rect = Rect::from_position_size((0, 0), (size, size));

        let points: Vec<_> = rect.iter().collect();

        for x in 0..size {
            for y in 0..size {
                assert!(points.contains(&IVec2::new(x, y)));
            }
        }

        assert_eq!(size * size, points.len() as i32);
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

        assert_eq!((30, 30), r.center().into());
        assert_eq!((25, 25), r.min.into());
        assert_eq!((35, 35), r.max.into());
        assert_eq!((10, 10), r.size().into());
    }
}
