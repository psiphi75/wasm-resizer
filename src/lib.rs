extern crate cfg_if;
extern crate wasm_bindgen;

use png;
use resize::Pixel;
use resize::Type::*;
use rgb::FromSlice;
use std::io::Cursor;

mod utils;

use cfg_if::cfg_if;
use wasm_bindgen::prelude::*;

cfg_if! {
    // When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
    // allocator.
    if #[cfg(feature = "wee_alloc")] {
        extern crate wee_alloc;
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }
}

#[wasm_bindgen]
pub fn resize(buf: &[u8]) -> Vec<u8> {
    let decoder = png::Decoder::new(Cursor::new(buf));
    let (info, mut reader) = decoder.read_info().unwrap();

    let mut src = vec![0; info.buffer_size()];
    reader.next_frame(&mut src).unwrap();

    let (w1, h1) = (info.width as usize, info.height as usize);
    let (w2, h2) = (100, 100);
    let mut dst = vec![0u8; w2 * h2 * info.color_type.samples()];
    match info.color_type {
        png::ColorType::Grayscale => resize::new(w1, h1, w2, h2, Pixel::Gray8, Triangle)
            .unwrap()
            .resize(src.as_gray(), dst.as_gray_mut())
            .unwrap(),
        png::ColorType::RGB => resize::new(w1, h1, w2, h2, Pixel::RGB8, Triangle)
            .unwrap()
            .resize(src.as_rgb(), dst.as_rgb_mut())
            .unwrap(),
        png::ColorType::Indexed => unimplemented!(),
        png::ColorType::GrayscaleAlpha => unimplemented!(),
        png::ColorType::RGBA => resize::new(w1, h1, w2, h2, Pixel::RGBA8, Triangle)
            .unwrap()
            .resize(src.as_rgba(), dst.as_rgba_mut())
            .unwrap(),
    };

    let mut result = Vec::new();
    let outfh = Cursor::new(&mut result);
    let mut encoder = png::Encoder::new(outfh, w2 as u32, h2 as u32);
    encoder.set_color(info.color_type);
    encoder.set_depth(info.bit_depth);
    encoder
        .write_header()
        .unwrap()
        .write_image_data(&dst)
        .unwrap();

    result
}
