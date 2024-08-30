use std::ops::{Index, IndexMut, Range};

use crate::pixel_formats::*;

#[derive(Clone, Debug, PartialEq)]
pub struct PixelRow<T: PixelChunk> {
    // TODO change pixels vector to be a kind of reference
    // This will allow cheap cloning of the row
    pixels: Vec<T>,
    pad_left: usize,
    pad_right: usize,
}

impl<T: PixelChunk<PixelType = T>> PixelRow<T> {
    pub fn new(size: usize) -> PixelRow<T> {
        Self::new_with(size, Default::default())
    }

    pub fn new_with(size: usize, pixel: T) -> PixelRow<T> {
        let pad = size % T::pixels();
        let real_size = size / T::pixels() + if pad > 0 { 1 } else { 0 };
        PixelRow {
            pixels: vec![pixel; real_size],
            pad_left: 0,
            pad_right: pad,
        }
    }

    // is this wanted/needed?
    pub fn from_vec(pixels: Vec<T>) -> PixelRow<T> {
        PixelRow {
            pixels,
            pad_left: 0,
            pad_right: 0,
        }
    }

    pub fn pixel(&self, index: usize) -> Option<T> {
        // TODO should this adjust index with pad_left?
        let chunk = self.pixels.get(index / T::pixels())?;
        chunk.get_pixel(index % T::pixels())
    }

    // set a pixel - the `pixel` value will use the "default" pixel in the provided pixel chunk
    pub fn set_pixel(&mut self, index: usize, pixel: T) {
        let chunk = self.pixels.get_mut(index / T::pixels()).unwrap();
        chunk.set_pixel(index % T::pixels(), pixel);
    }

    pub fn fill_range(&mut self, range: Range<usize>, pixel: T) {
        if T::pixels() == 1 || range.len() < (T::pixels() * 2) {
            // fill can be simplistic
            for i in range {
                self.set_pixel(i, pixel);
            }
        } else {
            // we can optimise our fill by using a filled chunk
            let filled_chunk = T::filled_pixel(pixel);
            let skip_front = range.start % T::pixels();
            if skip_front > 0 {
                // fill the front of the range
                for i in range.start..range.start + skip_front {
                    self.set_pixel(i, pixel);
                }
            }
            let skip_back = range.end % T::pixels();
            if skip_back > 0 {
                // fill the back of the range
                for i in range.end - skip_back..range.end {
                    self.set_pixel(i, pixel);
                }
            }
            for i in range.start + skip_front..range.end - skip_back {
                self.pixels[i / T::pixels()] = filled_chunk;
            }
        }
    }

    pub fn fill_range_with_chunk(&mut self, range: Range<usize>, chunk: T) {
        // fills a range with given chunk, where fill is aligned by chunk
        let skip_front = range.start % T::pixels();
        if skip_front > 0 {
            // fill the front of the range
            for i in range.start..range.start + skip_front {
                let px = chunk.get_pixel(i % T::pixels()).unwrap();
                self.set_pixel(i, px);
            }
        }
        let skip_back = range.end % T::pixels();
        if skip_back > 0 {
            // fill the back of the range
            for i in range.end - skip_back..range.end {
                let px = chunk.get_pixel(i % T::pixels()).unwrap();
                self.set_pixel(i, px);
            }
        }
        for i in range.start + skip_front..range.end - skip_back {
            self.pixels[i / T::pixels()] = chunk;
        }
    }

    pub fn fill_range_with(&mut self, _range: Range<usize>, _new_pixels: &[T]) {
        // TODO think this through properly
        // for now, whilst new_pixels is a slice of chunks, we'll treat each chunk as a single pixel

        // // TODO this implementation feels wrong...  as it can't easily be optimized
        // this _might_ be able to use copy_from_slice, combined with split_at_mut
        // for (i, pixel) in range.zip(new_pixels.iter()) {
        //     self.set_pixel(i, *pixel);
        // }
    }

    pub fn fill_range_with_chunks(&mut self, _range: Range<usize>, _new_chunks: &[T]) {
        // TODO think this through properly
        // what do we do about alignment?
        // should we have two variants? one that aligns, and another that doesn't?
    }

    pub fn len(&self) -> usize {
        self.pixels.len()
    }

    pub fn is_empty(&self) -> bool {
        self.pixels.is_empty()
    }
}

// generic index implementation
// NB this fetches a pixel chunk, so shouldn't be used to reference individual pixels
impl<T: PixelChunk> Index<usize> for PixelRow<T> {
    type Output = T;

    fn index(&self, index: usize) -> &T {
        &self.pixels[index]
    }
}

// NB this is fetching a pixel chunk...
impl<T: PixelChunk> IndexMut<usize> for PixelRow<T> {
    fn index_mut(&mut self, index: usize) -> &mut T {
        &mut self.pixels[index]
    }
}

// generic range index implementation
// this should also use a different implementations for different sizes of pixels
// for efficiency, we'd likely want to somehow return range based on bytes with info on pixels to ignore
impl<T: PixelChunk> Index<Range<usize>> for PixelRow<T> {
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
        let row: PixelRow<Pixel8> = PixelRow::new(3);
        assert_eq!(row.pixels.len(), 3);
        // let test_px: Pixel8 = Pixel8 { value: 0 };
        for pixel in row.pixels.iter() {
            let test_px: u8 = (*pixel).into();
            assert_eq!(test_px, 0);
        }
    }

    #[test]
    fn can_create_pixel_row_with_pixel() {
        // let row: PixelRow<Pixel8> = PixelRow::new_with(3, Pixel8 { value: 5 });
        let row: PixelRow<Pixel8> = PixelRow::new_with(3, 5.into());
        for i in 0..row.len() {
            assert_eq!(row.pixels[i].value, 5);
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
        assert_eq!(row.pixels[0].value, 0);
        assert_eq!(row.pixels[1].value, 1);
        assert_eq!(row.pixels[2].value, 2);
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
        assert_eq!(row.pixels[0], 3.into());
        assert_eq!(row.pixels[1], 4.into());
        assert_eq!(row.pixels[2], 5.into());

        row[0] = 6.into();
        assert_eq!(row.pixels[0], 6.into());
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

    // #[test]
    // fn can_fill_range_with_multiple_pixels() {
    //     let mut row: PixelRow<u8> = PixelRow::from_vec(vec![0, 1, 2, 3, 4]);
    //     row.fill_range_with(1..4, &[5, 6, 7]);
    //     assert_eq!(row.pixels[0], 0);
    //     assert_eq!(row.pixels[1], 5);
    //     assert_eq!(row.pixels[2], 6);
    //     assert_eq!(row.pixels[3], 7);
    //     assert_eq!(row.pixels[4], 4);

    //     // TODO test with pixels slice that's short, long, etc
    //     // and define what that behaviour should be
    //     let mut row: PixelRow<u8> = PixelRow::from_vec(vec![0, 1, 2, 3, 4]);
    //     row.fill_range_with(1..4, &[5, 6]);
    //     assert_eq!(row.pixels[0], 0);
    //     assert_eq!(row.pixels[1], 5);
    //     assert_eq!(row.pixels[2], 6);
    //     assert_eq!(row.pixels[3], 3);
    //     assert_eq!(row.pixels[4], 4);
    // }
}
