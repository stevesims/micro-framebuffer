// use std::ops::Index;

pub trait PixelChunk {
    type PixelType; // Add an associated type for the pixel type.

    fn pixels() -> usize;

    fn get_pixel(&self, index: usize) -> Option<Self>
    where
        Self: Sized; // Use the associated type for the return type.

    fn set_pixel(&mut self, index: usize, pixel: Self::PixelType);
}

#[derive(Debug, PartialEq, Clone, Copy, Default)]
pub struct Pixel8 {
    pub value: u8,
}

impl PixelChunk for Pixel8 {
    type PixelType = Pixel8;

    #[inline]
    fn pixels() -> usize {
        1
    }

    fn get_pixel(&self, index: usize) -> Option<Self> {
        match index {
            0 => Some(*self),
            _ => None,
        }
    }

    fn set_pixel(&mut self, _index: usize, pixel: Self::PixelType) {
        *self = pixel;
    }
}

impl From<u8> for Pixel8 {
    #[inline]
    fn from(value: u8) -> Self {
        Pixel8 { value }
    }
}

impl From<Pixel8> for u8 {
    #[inline]
    fn from(pixel: Pixel8) -> u8 {
        pixel.value
    }
}

// Pixel4 contains 2 pixels in a single byte
// the first pixel is the high 4 bits, the second pixel is the low 4 bits
// when used as an argument as a singular pixel, the pixel in the lowest 4 bits is used
#[derive(Debug, PartialEq, Clone)]
pub struct Pixel4 {
    pub value: u8,
}

impl PixelChunk for Pixel4 {
    type PixelType = Pixel4;

    #[inline]
    fn pixels() -> usize {
        2
    }

    fn get_pixel(&self, index: usize) -> Option<Self> {
        match index {
            0 => Some((self.value >> 4).into()),
            1 => Some((self.value & 0xF).into()),
            _ => None,
        }
    }

    fn set_pixel(&mut self, index: usize, pixel: Self::PixelType) {
        // NB we use the lowest bits in `pixel` as our source pixel (which is at index 1)
        match index {
            0 => {
                self.value = (self.value & 0xF) | (pixel.value << 4);
            }
            1 => {
                self.value = (self.value & 0xF0) | (pixel.value & 0xF);
            }
            _ => {}
        }
    }
}

impl From<u8> for Pixel4 {
    #[inline]
    fn from(value: u8) -> Self {
        // TODO is this correct?  arguably this should be value >> 4
        Pixel4 { value }
    }
}

impl From<Pixel4> for u8 {
    #[inline]
    fn from(pixel: Pixel4) -> u8 {
        pixel.value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_create_pixel4() {
        let pixel = Pixel4 { value: 0xF0 };

        assert_eq!(pixel.get_pixel(0), Some(Pixel4 { value: 0xF }));
        assert_eq!(pixel.value, 0xF0);
    }

    #[test]
    fn can_get_pixels_from_pixel4() {
        let pixel = Pixel4 { value: 0xFA };
        assert_eq!(pixel.get_pixel(0).unwrap().value, 0xF);
        assert_eq!(pixel.get_pixel(1).unwrap().value, 0xA);
        assert_eq!(pixel.get_pixel(2), None);
        assert_eq!(pixel.get_pixel(42), None);
    }

    #[test]
    fn check_alignment_and_size() {
        assert_eq!(std::mem::align_of::<Pixel4>(), 1);
        assert_eq!(std::mem::size_of::<Pixel4>(), 1);

        assert_eq!(std::mem::size_of::<Vec<Pixel4>>(), 24);

        let px: Pixel4 = 0xF0.into();
        let mut val = vec![px.clone(), px.clone(), px.clone()];
        for _i in 0..24 {
            val.push(px.clone());
        }

        assert_eq!(std::mem::size_of_val(&val), 24);

        // from this we learn that a Vec is 24 bytes, even if it contains more data
        // which makes sense, as it's contents are stored elsewhere
    }

    #[test]
    fn can_set_pixel4() {
        let mut pixel = Pixel4 { value: 0xA0 };
        pixel.set_pixel(1, Pixel4 { value: 0xB });
        assert_eq!(pixel.value, 0xAB);
    }

    #[test]
    fn can_set_pixel4_with_u8() {
        let mut pixel: Pixel4 = 0xA1.into();
        assert_eq!(pixel.get_pixel(0).unwrap().value, 0xA);
        assert_eq!(pixel.get_pixel(1).unwrap().value, 0x1);
        pixel.set_pixel(1, 0xB.into());
        assert_eq!(pixel.value, 0xAB);
    }

    #[test]
    fn can_get_u8_from_pixel4() {
        let pixel = Pixel4 { value: 0xAB };
        assert_eq!(u8::from(pixel), 0xAB);
    }
}
