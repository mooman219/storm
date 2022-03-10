// The MIT License (MIT)
//
// Copyright (c) 2014 PistonDevelopers
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.
//
// Below is from https://github.com/image-rs/image/blob/master/src/imageops/sample.rs

use crate::image::Image;
use crate::{color::RGBA8, math::Float};
use alloc::vec::Vec;

impl Image<RGBA8> {
    /// Interpret a slice of bytes as a PNG and decodes it into an RGBA image.
    pub fn from_png(bytes: &[u8]) -> Image<RGBA8> {
        crate::image::png::read_png(bytes)
    }

    pub fn resize(&self, width: u32, height: u32) -> Image<RGBA8> {
        let image = vertical_sample(self, height);
        horizontal_sample(&image, width)
    }
}

#[inline]
fn clamp<N: PartialOrd>(a: N, min: N, max: N) -> N {
    if a < min {
        min
    } else if a > max {
        max
    } else {
        a
    }
}

fn sinc(t: f32) -> f32 {
    let a = t * core::f32::consts::PI;

    if t == 0.0 {
        1.0
    } else {
        a.sin_rad_fast() / a
    }
}

fn lanczos3(x: f32) -> f32 {
    const WINDOW: f32 = 3.0;
    if x.abs() < WINDOW {
        sinc(x) * sinc(x / WINDOW)
    } else {
        0.0
    }
}

fn vertical_sample(image: &Image<RGBA8>, new_height: u32) -> Image<RGBA8> {
    const SUPPORT: f32 = 3.0;
    let (width, height) = (image.width(), image.height());
    let mut out = Image::from_color(RGBA8::MAGENTA, width, new_height);
    let mut ws = Vec::new();

    let ratio = height as f32 / new_height as f32;
    let sratio = if ratio < 1.0 {
        1.0
    } else {
        ratio
    };
    let src_support = SUPPORT * sratio;

    for outy in 0..new_height {
        let inputy = (outy as f32 + 0.5) * ratio;

        let left = (inputy - src_support).floor() as i64;
        let left = clamp(left, 0, <i64 as From<_>>::from(height) - 1) as u32;

        let right = (inputy + src_support).ceil() as i64;
        let right = clamp(right, <i64 as From<_>>::from(left) + 1, <i64 as From<_>>::from(height)) as u32;

        let inputy = inputy - 0.5;

        ws.clear();
        let mut sum = 0.0;
        for i in left..right {
            let w = lanczos3((i as f32 - inputy) / sratio);
            ws.push(w);
            sum += w;
        }
        ws.iter_mut().for_each(|w| *w /= sum);

        for x in 0..width {
            let mut t = (0.0, 0.0, 0.0, 0.0);

            for (i, w) in ws.iter().enumerate() {
                let p = image.get(x, left + i as u32);
                let vec: (f32, f32, f32, f32) = p.into();
                t.0 += vec.0 * w;
                t.1 += vec.1 * w;
                t.2 += vec.2 * w;
                t.3 += vec.3 * w;
            }

            let t = RGBA8::from_f32(t.0, t.1, t.2, t.3);

            out.set(x, outy, t);
        }
    }

    out
}

fn horizontal_sample(image: &Image<RGBA8>, new_width: u32) -> Image<RGBA8> {
    const SUPPORT: f32 = 3.0;
    let (width, height) = (image.width(), image.height());
    let mut out = Image::from_color(RGBA8::MAGENTA, new_width, height);
    let mut ws = Vec::new();

    let ratio = width as f32 / new_width as f32;
    let sratio = if ratio < 1.0 {
        1.0
    } else {
        ratio
    };
    let src_support = SUPPORT * sratio;

    for outx in 0..new_width {
        // Find the point in the input image corresponding to the centre
        // of the current pixel in the output image.
        let inputx = (outx as f32 + 0.5) * ratio;

        // Left and right are slice bounds for the input pixels relevant
        // to the output pixel we are calculating.  Pixel x is relevant
        // if and only if (x >= left) && (x < right).

        // Invariant: 0 <= left < right <= width

        let left = (inputx - src_support).floor() as i64;
        let left = clamp(left, 0, <i64 as From<_>>::from(width) - 1) as u32;

        let right = (inputx + src_support).ceil() as i64;
        let right = clamp(right, <i64 as From<_>>::from(left) + 1, <i64 as From<_>>::from(width)) as u32;

        // Go back to left boundary of pixel, to properly compare with i
        // below, as the kernel treats the centre of a pixel as 0.
        let inputx = inputx - 0.5;

        ws.clear();
        let mut sum = 0.0;
        for i in left..right {
            let w = lanczos3((i as f32 - inputx) / sratio);
            ws.push(w);
            sum += w;
        }
        ws.iter_mut().for_each(|w| *w /= sum);

        for y in 0..height {
            let mut t = (0.0, 0.0, 0.0, 0.0);

            for (i, w) in ws.iter().enumerate() {
                let p = image.get(left + i as u32, y);
                let vec: (f32, f32, f32, f32) = p.into();
                t.0 += vec.0 * w;
                t.1 += vec.1 * w;
                t.2 += vec.2 * w;
                t.3 += vec.3 * w;
            }

            let t = RGBA8::from_f32(
                clamp(t.0, 0.0, 1.0),
                clamp(t.1, 0.0, 1.0),
                clamp(t.2, 0.0, 1.0),
                clamp(t.3, 0.0, 1.0),
            );

            out.set(outx, y, t);
        }
    }

    out
}
