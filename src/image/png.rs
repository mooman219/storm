use super::Image;
use crate::color::RGBA8;
use png::{ColorType, Decoder};

/// Interpret a slice of bytes as a PNG and decodes it into an RGBA image.
pub fn read_png(bytes: &[u8]) -> Image<RGBA8> {
    let decoder = Decoder::new(bytes);
    let (info, mut reader) = decoder.read_info().expect("Unable to read PNG info.");
    let mut input = vec![0; info.buffer_size()];
    reader.next_frame(&mut input).expect("Unable to read PNG payload.");

    match info.color_type {
        ColorType::RGB => {
            let mut output = Vec::with_capacity((input.len() / 3) * 4);
            for rgb in input.chunks_exact(3) {
                output.push(RGBA8::new(rgb[0], rgb[1], rgb[2], 255));
            }
            Image::from_vec(output, info.width, info.height)
        }
        ColorType::RGBA => {
            let mut output = Vec::with_capacity(input.len());
            for rgba in input.chunks_exact(4) {
                output.push(RGBA8::new(rgba[0], rgba[1], rgba[2], rgba[3]));
            }
            Image::from_vec(output, info.width, info.height)
        }
        ColorType::Grayscale => {
            let mut output = Vec::with_capacity(input.len() * 4);
            for g in input {
                output.push(RGBA8::new(g, g, g, 255));
            }
            Image::from_vec(output, info.width, info.height)
        }
        ColorType::GrayscaleAlpha => {
            let mut output = Vec::with_capacity(input.len() * 2);
            for ga in input.chunks_exact(2) {
                output.push(RGBA8::new(ga[0], ga[0], ga[0], ga[1]));
            }
            Image::from_vec(output, info.width, info.height)
        }
        ColorType::Indexed => panic!("PNG Indexed color type is unsupported."),
    }
}
