use std::ops::{Index, IndexMut, Range};

use crate::pixel_formats::*;

#[derive(Clone, Debug, PartialEq)]
pub struct PixelRow<T: PixelChunk> {
    // TODO change pixel_chunks vector to be a kind of reference
    // This will allow cheap cloning of the row
    pixel_chunks: Vec<T>,
    // Amount of pixels to ignore at the start of the row
    pad_left: usize,
    // Amount of pixels to ignore at the end of the row
    pad_right: usize,
}

pub struct PixelRowIterator<'a, T: PixelChunk> {
    row: &'a PixelRow<T>,
    pixel_index: usize,
    chunk_index: usize,
    chunk_iterator: <T as std::iter::IntoIterator>::IntoIter,
}

impl<'a, T: PixelChunk> Iterator for PixelRowIterator<'a, T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pixel_index >= (self.row.pixel_chunks.len() * T::pixels()) - self.row.pad_right {
            return None;
        }

        let mut pixel = self.chunk_iterator.next();
        self.pixel_index += 1;
        if pixel.is_none() {
            self.chunk_index += 1;
            if self.chunk_index < self.row.pixel_chunks.len() {
                self.chunk_iterator = self.row.pixel_chunks[self.chunk_index].into_iter();
                pixel = self.chunk_iterator.next();
            }
        }

        pixel
    }
}

impl<T: PixelChunk<PixelType = T>> PixelRow<T> {
    pub fn new(size: usize) -> PixelRow<T> {
        Self::new_with(size, Default::default())
    }

    pub fn new_with(size: usize, pixel: T) -> PixelRow<T> {
        let pad = size % T::pixels();
        let real_size = size / T::pixels() + if pad > 0 { 1 } else { 0 };
        PixelRow {
            pixel_chunks: vec![pixel; real_size],
            pad_left: 0,
            pad_right: pad,
        }
    }

    // is this wanted/needed?
    pub fn from_vec(pixel_chunks: Vec<T>) -> PixelRow<T> {
        PixelRow {
            pixel_chunks,
            pad_left: 0,
            pad_right: 0,
        }
    }

    pub fn width(&self) -> usize {
        self.pixel_chunks.len() * T::pixels() - self.pad_left - self.pad_right
    }

    pub fn pixel(&self, index: usize) -> Option<T> {
        let actual_index = index + self.pad_left;
        let chunk = self.pixel_chunks.get(actual_index / T::pixels())?;
        chunk.get_pixel(actual_index % T::pixels())
    }

    // set a pixel - the `pixel` value will use the "default" pixel in the provided pixel chunk
    pub fn set_pixel(&mut self, index: usize, pixel: T) {
        let actual_index = index + self.pad_left;
        let chunk = self
            .pixel_chunks
            .get_mut(actual_index / T::pixels())
            .unwrap();
        chunk.set_pixel(actual_index % T::pixels(), pixel);
    }

    pub fn fill_range(&mut self, range: Range<usize>, pixel: T) {
        if T::pixels() == 1 || range.len() < (T::pixels() * 2) {
            // fill can be simplistic
            for i in range {
                self.set_pixel(i, pixel);
            }
        } else {
            // we can optimise our fill by using a filled chunk
            self.fill_range_with_chunk(range, T::filled_pixel(pixel));
        }
    }

    pub fn fill_range_with_chunk(&mut self, range: Range<usize>, chunk: T) {
        // fills a range with given chunk, where fill is aligned by chunk
        // Work out how many pixels we need to skip at front to align with chunk
        let skip_front = (range.start + self.pad_left) % T::pixels();
        if skip_front > 0 {
            // fill the front of the range
            for i in range.start..range.start + skip_front {
                let px = chunk.get_pixel(i % T::pixels()).unwrap();
                self.set_pixel(i, px);
            }
        }
        // Similarly work out what pixels at end need to be set to align with chunk
        let skip_back = (self.pad_left + range.end) % T::pixels();
        if skip_back > 0 {
            // fill the back of the range
            for i in range.end - skip_back..range.end {
                let px = chunk.get_pixel(i % T::pixels()).unwrap();
                self.set_pixel(i, px);
            }
        }
        for i in range.start + skip_front..range.end - skip_back {
            self.pixel_chunks[i / T::pixels()] = chunk;
        }
    }

    pub fn fill_range_with(&mut self, range: Range<usize>, new_pixels: &[T]) {
        // TODO think this through properly
        // for now, whilst new_pixels is a slice of chunks, we'll treat each chunk as a single pixel

        // // TODO this implementation feels wrong...  as it can't easily be optimized
        // this _might_ be able to use copy_from_slice, combined with split_at_mut
        for (i, pixel) in range.zip(new_pixels.iter()) {
            self.set_pixel(i, *pixel);
        }
    }

    pub fn fill_range_with_chunks(&mut self, _range: Range<usize>, _new_chunks: &[T]) {
        // TODO think this through properly
        // what do we do about alignment?
        // should we have two variants? one that aligns, and another that doesn't?
    }

    pub fn len(&self) -> usize {
        self.pixel_chunks.len()
    }

    pub fn is_empty(&self) -> bool {
        self.pixel_chunks.is_empty()
    }
}

impl<'a, T: PixelChunk> IntoIterator for &'a PixelRow<T> {
    type Item = T;
    type IntoIter = PixelRowIterator<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        let mut iterator = PixelRowIterator {
            row: self,
            pixel_index: 0,
            chunk_index: 0,
            chunk_iterator: self.pixel_chunks[0].into_iter(),
        };
        // skip any padding set by pad_left
        let mut counter = self.pad_left;
        while counter > 0 {
            iterator.next();
            counter -= 1;
        }
        iterator
    }
}

// generic index implementation
// NB this fetches a pixel chunk, so shouldn't be used to reference individual pixels
impl<T: PixelChunk> Index<usize> for PixelRow<T> {
    type Output = T;

    fn index(&self, index: usize) -> &T {
        &self.pixel_chunks[index]
    }
}

// NB this is fetching a pixel chunk...
impl<T: PixelChunk> IndexMut<usize> for PixelRow<T> {
    fn index_mut(&mut self, index: usize) -> &mut T {
        &mut self.pixel_chunks[index]
    }
}

// generic range index implementation
// this should also use a different implementations for different sizes of pixels
// for efficiency, we'd likely want to somehow return range based on bytes with info on pixels to ignore
impl<T: PixelChunk> Index<Range<usize>> for PixelRow<T> {
    type Output = [T];

    fn index(&self, range: Range<usize>) -> &[T] {
        &self.pixel_chunks[range]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_create_pixel_row() {
        let row: PixelRow<Pixel8> = PixelRow::new(3);
        assert_eq!(row.pixel_chunks.len(), 3);
        for pixel in row.pixel_chunks.iter() {
            assert_eq!(pixel.value, 0);
        }
        for pixel in &row.pixel_chunks {
            let test_px: u8 = (*pixel).into();
            assert_eq!(test_px, 0);
        }
    }

    #[test]
    fn can_create_pixel_row_with_pixel() {
        // let row: PixelRow<Pixel8> = PixelRow::new_with(3, Pixel8 { value: 5 });
        let row: PixelRow<Pixel8> = PixelRow::new_with(3, 5.into());
        for i in 0..row.len() {
            assert_eq!(row.pixel_chunks[i].value, 5);
        }
    }

    #[test]
    fn can_create_pixel_row_from_vector() {
        // let row: PixelRow<Pixel8> = PixelRow::from_vec(vec![0, 1, 2]);
        let pixel_vec: Vec<Pixel8> = vec![
            Pixel8 { value: 0 },
            Pixel8 { value: 1 },
            Pixel8 { value: 2 },
        ];
        // let pixel_vec: Vec<Pixel8> = vec![0.into(), 1.into(), 2.into()];
        // let pixel_vec: Vec<Pixel8> = vec![0, 1, 2];
        // let row: PixelRow<Pixel8> = PixelRow::from_vec(vec![0, 1, 2]);
        let row: PixelRow<Pixel8> = PixelRow::from_vec(pixel_vec);
        assert_eq!(row.pixel_chunks[0].value, 0);
        assert_eq!(row.pixel_chunks[1].value, 1);
        assert_eq!(row.pixel_chunks[2].value, 2);
    }

    #[test]
    fn can_get_pixel() {
        let row: PixelRow<Pixel8> = PixelRow::from_vec(vec![0.into(), 1.into(), 2.into()]);
        assert_eq!(row.pixel(0), Some(0.into()));
        assert_eq!(row.pixel(1), Some(1.into()));
        assert_eq!(row.pixel(2), Some(2.into()));
        assert_eq!(row.pixel(3), None);
    }

    #[test]
    fn can_get_pixel_chunks_with_index() {
        let row: PixelRow<Pixel8> =
            PixelRow::from_vec(vec![0.into(), 1.into(), 2.into(), 3.into(), 4.into()]);
        assert_eq!(row[0], 0.into());
        assert_eq!(row[1], 1.into());
        assert_eq!(row[2], 2.into());
    }

    #[test]
    fn can_get_slice_from_pixelrow() {
        let row: PixelRow<Pixel8> =
            PixelRow::from_vec(vec![0.into(), 1.into(), 2.into(), 3.into(), 4.into()]);
        assert_eq!(row[1..4], [1.into(), 2.into(), 3.into()]);
    }

    #[test]
    fn can_set_a_pixel_in_a_pixelrow() {
        let mut row: PixelRow<Pixel8> = PixelRow::from_vec(vec![0.into(), 1.into(), 2.into()]);
        row.set_pixel(0, 3.into());
        row.set_pixel(1, 4.into());
        row.set_pixel(2, 5.into());
        assert_eq!(row.pixel_chunks[0], 3.into());
        assert_eq!(row.pixel_chunks[1], 4.into());
        assert_eq!(row.pixel_chunks[2], 5.into());

        row[0] = 6.into();
        assert_eq!(row.pixel_chunks[0], 6.into());
    }

    #[test]
    fn can_fill_range_of_a_pixelrow() {
        let mut row: PixelRow<Pixel4> = PixelRow::new(7);
        for i in 0..7 {
            row.set_pixel(i, (i as u8).into());
        }

        row.fill_range(2..4, 13.into());
        assert_eq!(row.pixel(0), Some(0.into()));
        assert_eq!(row.pixel(1), Some(1.into()));
        assert_eq!(row.pixel(2), Some(13.into()));
        assert_eq!(row.pixel(3), Some(13.into()));
        assert_eq!(row.pixel(4), Some(4.into()));
        assert_eq!(row.pixel(5), Some(5.into()));
        assert_eq!(row.pixel(6), Some(6.into()));

        row.fill_range(1..5, 9.into());
        assert_eq!(row.pixel(0), Some(0.into()));
        assert_eq!(row.pixel(1), Some(9.into()));
        assert_eq!(row.pixel(2), Some(9.into()));
        assert_eq!(row.pixel(3), Some(9.into()));
        assert_eq!(row.pixel(4), Some(9.into()));
        assert_eq!(row.pixel(5), Some(5.into()));
        assert_eq!(row.pixel(6), Some(6.into()));
        assert_eq!(row.pad_right, 1);
    }

    #[test]
    fn can_get_width_of_pixelrow() {
        let row: PixelRow<Pixel8> = PixelRow::new(3);
        assert_eq!(row.width(), 3);

        let mut row: PixelRow<Pixel4> = PixelRow::new(7);
        assert_eq!(row.width(), 7);
        assert_eq!(row.pad_left, 0);
        assert_eq!(row.pad_right, 1);
        assert_eq!(row.pixel_chunks.len(), 4);

        row.pad_left = 1;
        assert_eq!(row.width(), 6);
        assert_eq!(row.pixel_chunks.len(), 4);
    }

    #[test]
    fn can_adjust_padding() {
        let mut row: PixelRow<Pixel4> = PixelRow::new(7);
        for i in 0..7 {
            row.set_pixel(i, (i as u8).into());
        }
        assert_eq!(row.pad_left, 0);
        assert_eq!(row.pad_right, 1);
        assert_eq!(row.width(), 7);

        row.pad_left = 1;
        assert_eq!(row.pixel(0), Some(1.into()));
        assert_eq!(row.width(), 6);
        row.pad_left = 4;
        assert_eq!(row.pixel(0), Some(4.into()));
        assert_eq!(row.width(), 3);

        // Remove our right padding, which should increase the width
        row.pad_right = 0;
        assert_eq!(row.width(), 4);
        // new last pixel will be default (a zero)
        assert_eq!(row.pixel(3), Some(0.into()));

        // Adjust our current zero indexed pixel to be 7
        row.set_pixel(0, 7.into());

        row.pad_left = 0;
        assert_eq!(row.width(), 8);
        assert_eq!(row.pixel(0), Some(0.into()));
        assert_eq!(row.pixel(4), Some(7.into()));
    }

    #[test]
    fn can_fill_range_with_multiple_pixels() {
        let mut row: PixelRow<Pixel8> =
            PixelRow::from_vec(vec![0.into(), 1.into(), 2.into(), 3.into(), 4.into()]);
        row.fill_range_with(1..4, &[5.into(), 6.into(), 7.into()]);
        assert_eq!(row.pixel(0), Some(0.into()));
        assert_eq!(row.pixel(1), Some(5.into()));
        assert_eq!(row.pixel(2), Some(6.into()));
        assert_eq!(row.pixel(3), Some(7.into()));
        assert_eq!(row.pixel(4), Some(4.into()));

        // TODO test with pixels slice that's short, long, etc
        // and define what that behaviour should be
        // let mut row: PixelRow<u8> = PixelRow::from_vec(vec![0, 1, 2, 3, 4]);
        // row.fill_range_with(1..4, &[5, 6]);
        // assert_eq!(row.pixel_chunks[0], 0);
        // assert_eq!(row.pixel_chunks[1], 5);
        // assert_eq!(row.pixel_chunks[2], 6);
        // assert_eq!(row.pixel_chunks[3], 3);
        // assert_eq!(row.pixel_chunks[4], 4);
    }

    #[test]
    fn can_iterate_over_row() {
        let row: PixelRow<Pixel8> = PixelRow::from_vec(vec![0.into(), 1.into(), 2.into()]);
        let mut iter: PixelRowIterator<Pixel8> = row.into_iter();
        assert_eq!(iter.next(), Some(0.into()));
        assert_eq!(iter.next(), Some(1.into()));
        assert_eq!(iter.next(), Some(2.into()));
        assert_eq!(iter.next(), None);

        // ensure that row hasn't been consumed
        let mut iter: PixelRowIterator<Pixel8> = row.into_iter();
        assert_eq!(iter.next(), Some(0.into()));
        assert_eq!(iter.next(), Some(1.into()));
        assert_eq!(iter.next(), Some(2.into()));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn can_iterate_over_padded_row() {
        let mut row: PixelRow<Pixel4> = PixelRow::new(14);
        row.pad_left = 3;
        row.pad_right = 1;
        assert_eq!(row.width(), 10);
        for i in 0..10 {
            row.set_pixel(i, (i as u8).into());
        }

        let mut i = 0;
        for pixel in &row {
            assert_eq!(pixel.value, i);
            i += 1;
        }
        assert_eq!(i, 10);
    }
}
