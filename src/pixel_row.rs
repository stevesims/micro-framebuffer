use std::ops::{Index, IndexMut, Range};

use crate::pixel_formats::*;

#[derive(Clone, Debug, PartialEq)]
pub struct PixelRow<T: PixelChunk> {
    pixels: Vec<T>,
    pad_left: usize,
    pad_right: usize,
}

impl<T: Copy + Default + PixelChunk> PixelRow<T> {
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

    pub fn set_pixel(&mut self, index: usize, pixel: <T as PixelChunk>::PixelType) {
        // TODO this needs to change to handle pixel chunks
        let chunk = self.pixels.get_mut(index / T::pixels()).unwrap();
        chunk.set_pixel(index % T::pixels(), pixel);
    }

    pub fn fill_range(&mut self, _range: Range<usize>, _pixel: T) {
        // this needs changing so that it understands pixel chunks
        // it should fill as if the chunk is a single pixel (using only the first pixel in chunk)
        // and can optimise by using a filled chunk
        // for i in range {
        //     self.set_pixel(i, pixel);
        // }
    }

    pub fn fill_range_with_chunk(&mut self, _range: Range<usize>, _chunk: T) {
        // fill should get aligned to chunk size
        // should set individual pixels to align
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

    // fill with pixel, and fill with chunk
    pub fn fill_range_with_chunks(&mut self, _range: Range<usize>, _new_chunks: &[T]) {
        // TODO think this through properly
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

        assert_eq!(row[1..4], [1.into(), 2.into(), 3.into()]);
    }

    // #[test]
    // fn can_get_slice_from_pixelrow() {
    //     let row: PixelRow<u8> = PixelRow::from_vec(vec![0, 1, 2, 3, 4]);
    //     assert_eq!(row[1..4], [1, 2, 3]);
    // }

    // #[test]
    // fn can_set_a_pixel_in_a_pixelrow() {
    //     let mut row: PixelRow<u8> = PixelRow::from_vec(vec![0, 1, 2]);
    //     row.set_pixel(0, 3);
    //     row.set_pixel(1, 4);
    //     row.set_pixel(2, 5);
    //     assert_eq!(row.pixels[0], 3);
    //     assert_eq!(row.pixels[1], 4);
    //     assert_eq!(row.pixels[2], 5);

    //     row[0] = 6;
    //     assert_eq!(row.pixels[0], 6);
    // }

    // #[test]
    // fn can_fill_range_of_a_pixelrow() {
    //     let mut row: PixelRow<u8> = PixelRow::from_vec(vec![0, 1, 2, 3, 4]);
    //     row.fill_range(1..4, 5);
    //     assert_eq!(row.pixels[0], 0);
    //     assert_eq!(row.pixels[1], 5);
    //     assert_eq!(row.pixels[2], 5);
    //     assert_eq!(row.pixels[3], 5);
    //     assert_eq!(row.pixels[4], 4);
    // }

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
