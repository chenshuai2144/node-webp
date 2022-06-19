use std::u8;

use libwebp_sys::WebPEncodeLosslessRGBA;

pub fn read_image(image_bytes: Vec<u8>, width: u32, height: u32, stride: u32, quality_factor: f32) {
  WebPEncodeLosslessRGBA(
    &[
      255, 255, 255, 255, // white
      255, 0, 0, 255, // red
      0, 255, 0, 255, // green
      0, 0, 255, 255, // blue
    ],
    width,
    height,
    stride,
  )
  .unwrap();
}
