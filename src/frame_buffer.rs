use std::ops::{Index, Range};

use crate::pixel_row::PixelRow;

#[allow(dead_code)]
pub struct FrameBuffer<T> {
    width: usize,
    height: usize,
    rows: Vec<PixelRow<T>>,
}

impl<T: Copy + Default> FrameBuffer<T> {
    pub fn new(width: usize, height: usize) -> FrameBuffer<T> {
        FrameBuffer {
            width,
            height,
            rows: vec![PixelRow::new(width); height],
        }
    }

    pub fn new_with(width: usize, height: usize, pixel: T) -> FrameBuffer<T> {
        FrameBuffer {
            width,
            height,
            rows: vec![PixelRow::new_with(width, pixel); height],
        }
    }

    pub fn row(&self, y: usize) -> Option<&PixelRow<T>> {
        self.rows.get(y)
    }

    pub fn pixel(&self, x: usize, y: usize) -> Option<&T> {
        self.rows.get(y).and_then(|row| row.pixel(x))
    }
}

impl<T> Index<usize> for FrameBuffer<T> {
    type Output = PixelRow<T>;

    fn index(&self, index: usize) -> &PixelRow<T> {
        &self.rows[index]
    }
}

// impl<T> IndexMut<usize> for FrameBuffer<T> {
//     fn index_mut(&mut self, index: usize) -> &mut PixelRow<T> {
//         &mut self.rows[index]
//     }
// }

impl<T> Index<Range<usize>> for FrameBuffer<T> {
    type Output = [PixelRow<T>];

    fn index(&self, range: Range<usize>) -> &[PixelRow<T>] {
        &self.rows[range]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_create_frame_buffer() {
        let buffer: FrameBuffer<u8> = FrameBuffer::new(7, 3);
        assert_eq!(buffer.rows.len(), 3);
        assert_eq!(buffer.rows.len(), buffer.height);
        for row in buffer.rows.iter() {
            assert_eq!(row.len(), 7);
            assert_eq!(row.len(), buffer.width);
        }
    }

    #[test]
    fn can_create_frame_buffer_with_pixel() {
        let buffer: FrameBuffer<u8> = FrameBuffer::new_with(3, 3, 5);
        for row in buffer.rows.iter() {
            for i in 0..row.len() {
                assert_eq!(row[i], 5);
            }
        }
    }

    #[test]
    fn can_get_row() {
        let buffer: FrameBuffer<u8> = FrameBuffer::new(3, 3);
        assert_eq!(buffer.row(0).unwrap().len(), 3);
        assert_eq!(buffer.row(1).unwrap().len(), 3);
        assert_eq!(buffer.row(2).unwrap().len(), 3);
        assert_eq!(buffer.row(3), None);
    }

    #[test]
    fn can_get_pixel() {
        let buffer: FrameBuffer<u8> = FrameBuffer::new(3, 3);
        assert_eq!(buffer.pixel(0, 0), Some(&0));
        assert_eq!(buffer.pixel(1, 1), Some(&0));
        assert_eq!(buffer.pixel(2, 2), Some(&0));
        assert_eq!(buffer.pixel(3, 3), None);
    }
}
