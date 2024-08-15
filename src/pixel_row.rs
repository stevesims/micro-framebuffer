use std::ops::{Index, IndexMut, Range};

#[derive(Clone, Debug, PartialEq)]
pub struct PixelRow<T> {
    pixels: Vec<T>,
}

impl<T: Copy + Default> PixelRow<T> {
    pub fn new(size: usize) -> PixelRow<T> {
        PixelRow {
            pixels: vec![Default::default(); size],
        }
    }

    pub fn new_with(size: usize, pixel: T) -> PixelRow<T> {
        PixelRow {
            pixels: vec![pixel; size],
        }
    }

    pub fn from_vec(pixels: Vec<T>) -> PixelRow<T> {
        PixelRow { pixels }
    }

    pub fn pixel(&self, index: usize) -> Option<&T> {
        self.pixels.get(index)
    }

    pub fn set_pixel(&mut self, index: usize, pixel: T) {
        if let Some(p) = self.pixels.get_mut(index) {
            *p = pixel;
        }
    }

    pub fn fill_range(&mut self, range: Range<usize>, pixel: T) {
        for i in range {
            self.set_pixel(i, pixel);
        }
    }

    pub fn fill_range_with(&mut self, range: Range<usize>, new_pixels: &[T]) {
        // // TODO this implementation feels wrong...  as it can't easily be optimized
        // this _might_ be able to use copy_from_slice, combined with split_at_mut
        for (i, pixel) in range.zip(new_pixels.iter()) {
            self.set_pixel(i, *pixel);
        }
    }

    pub fn len(&self) -> usize {
        self.pixels.len()
    }

    pub fn is_empty(&self) -> bool {
        self.pixels.is_empty()
    }
}

// generic index implementation
// we'd have different ones for different sizes of pixels
impl<T> Index<usize> for PixelRow<T> {
    type Output = T;

    fn index(&self, index: usize) -> &T {
        &self.pixels[index]
    }
}

// TODO consider if this can be supported when we're looking at pixels within a struct (partial-byte pixels)
impl<T> IndexMut<usize> for PixelRow<T> {
    fn index_mut(&mut self, index: usize) -> &mut T {
        &mut self.pixels[index]
    }
}

// generic range index implementation
// this should also use a different implementations for different sizes of pixels
// for efficiency, we'd likely want to somehow return range based on bytes with info on pixels to ignore
impl<T> Index<Range<usize>> for PixelRow<T> {
    type Output = [T];

    fn index(&self, range: Range<usize>) -> &[T] {
        &self.pixels[range]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_create_pixel_row() {
        let row: PixelRow<u8> = PixelRow::new(3);
        assert_eq!(row.pixels[0], 0);
        assert_eq!(row.pixels[1], 0);
        assert_eq!(row.pixels[2], 0);
    }

    #[test]
    fn can_create_pixel_row_with_pixel() {
        let row: PixelRow<u8> = PixelRow::new_with(3, 5);
        for i in 0..row.len() {
            assert_eq!(row.pixels[i], 5);
        }
        assert_eq!(row.pixels[0], 5);
        assert_eq!(row.pixels[1], 5);
        assert_eq!(row.pixels[2], 5);
    }

    #[test]
    fn can_create_pixel_row_from_vector() {
        let row: PixelRow<u8> = PixelRow::from_vec(vec![0, 1, 2]);
        assert_eq!(row.pixels[0], 0);
        assert_eq!(row.pixels[1], 1);
        assert_eq!(row.pixels[2], 2);
    }

    #[test]
    fn can_get_pixel() {
        let row: PixelRow<u8> = PixelRow::from_vec(vec![0, 1, 2]);
        assert_eq!(row.pixel(0), Some(&0));
        assert_eq!(row.pixel(1), Some(&1));
        assert_eq!(row.pixel(2), Some(&2));
        assert_eq!(row.pixel(3), None);
    }

    #[test]
    fn can_get_pixel_with_index() {
        let row: PixelRow<u8> = PixelRow::from_vec(vec![0, 1, 2, 3, 4]);
        assert_eq!(row[0], 0);
        assert_eq!(row[1], 1);
        assert_eq!(row[2], 2);

        assert_eq!(row[1..4], [1, 2, 3]);
    }

    #[test]
    fn can_get_slice_from_pixelrow() {
        let row: PixelRow<u8> = PixelRow::from_vec(vec![0, 1, 2, 3, 4]);
        assert_eq!(row[1..4], [1, 2, 3]);
    }

    #[test]
    fn can_set_a_pixel_in_a_pixelrow() {
        let mut row: PixelRow<u8> = PixelRow::from_vec(vec![0, 1, 2]);
        row.set_pixel(0, 3);
        row.set_pixel(1, 4);
        row.set_pixel(2, 5);
        assert_eq!(row.pixels[0], 3);
        assert_eq!(row.pixels[1], 4);
        assert_eq!(row.pixels[2], 5);

        row[0] = 6;
        assert_eq!(row.pixels[0], 6);
    }

    #[test]
    fn can_fill_range_of_a_pixelrow() {
        let mut row: PixelRow<u8> = PixelRow::from_vec(vec![0, 1, 2, 3, 4]);
        row.fill_range(1..4, 5);
        assert_eq!(row.pixels[0], 0);
        assert_eq!(row.pixels[1], 5);
        assert_eq!(row.pixels[2], 5);
        assert_eq!(row.pixels[3], 5);
        assert_eq!(row.pixels[4], 4);
    }

    #[test]
    fn can_fill_range_with_multiple_pixels() {
        let mut row: PixelRow<u8> = PixelRow::from_vec(vec![0, 1, 2, 3, 4]);
        row.fill_range_with(1..4, &[5, 6, 7]);
        assert_eq!(row.pixels[0], 0);
        assert_eq!(row.pixels[1], 5);
        assert_eq!(row.pixels[2], 6);
        assert_eq!(row.pixels[3], 7);
        assert_eq!(row.pixels[4], 4);

        // TODO test with pixels slice that's short, long, etc
        // and define what that behaviour should be
        let mut row: PixelRow<u8> = PixelRow::from_vec(vec![0, 1, 2, 3, 4]);
        row.fill_range_with(1..4, &[5, 6]);
        assert_eq!(row.pixels[0], 0);
        assert_eq!(row.pixels[1], 5);
        assert_eq!(row.pixels[2], 6);
        assert_eq!(row.pixels[3], 3);
        assert_eq!(row.pixels[4], 4);
    }
}
