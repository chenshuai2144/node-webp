#![deny(clippy::all)]

use std::os::raw::c_int;

use image::EncodableLayout;
use libwebp_sys::{WebPEncodeLosslessRGBA, WebPEncodeRGBA};

#[macro_use]
extern crate napi_derive;

#[napi]
fn image_to_webp(path: String, quality_factor: f64) -> bool {
  let buf: &[u8] = &[
    255, 255, 255, 255, // white
    255, 0, 0, 255, // red
    0, 255, 0, 255, // green
    0, 0, 255, 255, // blue
  ];

  let width: c_int = 2;
  let height: c_int = 2;
  let stride: c_int = 8;
  let output: *mut *mut u8 = 8 as *mut u8 as *mut *mut u8;
  unsafe {
    let data = WebPEncodeRGBA(buf.as_ptr(), width, height, stride, 75.0, output);
    println!("{}", data);
  }

  true
}

#[cfg(test)]
mod tests {

  use crate::image_to_webp;

  #[test]
  fn png_to_webp() {
    image_to_webp("C:\\rust\\node-webp\\test.png".to_string(), 100.0);
  }
}
