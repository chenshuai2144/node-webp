#![deny(clippy::all)]

use std::{fs, io::Write};
mod webp;
use image::io::Reader as ImageReader;
use reqwest;

#[macro_use]
extern crate napi_derive;

#[napi]
fn image_to_webp(path: String, dist_path: String) -> bool {
  // Open path as DynamicImage
  let file_path = path;
  let image = ImageReader::open(file_path)
    .unwrap()
    .with_guessed_format()
    .unwrap()
    .decode()
    .unwrap();
  let webp_file_content = webp::image_to_webp(image).unwrap();
  let mut webp_file = fs::File::create(dist_path).unwrap();
  webp_file.write_all(&webp_file_content.to_vec()).unwrap();
  true
}

#[napi]
fn web_image_to_webp(url: String, dist_path: String) -> bool {
  let response = reqwest::blocking::get(url.clone()).unwrap();

  let content_type = response
    .headers()
    .get("content-type")
    .unwrap()
    .to_str()
    .unwrap();

  if !content_type.starts_with("image") {
    return false;
  }

  let img_bytes = response.bytes().unwrap();

  let image = image::load_from_memory(&img_bytes).unwrap();
  let webp_file_content = webp::image_to_webp(image).unwrap();
  let mut webp_file = fs::File::create(dist_path).unwrap();
  webp_file.write_all(&webp_file_content.to_vec()).unwrap();
  true
}

#[cfg(test)]
mod tests {

  use crate::image_to_webp;
  use crate::web_image_to_webp;

  #[test]
  fn png_to_webp() {
    image_to_webp("test.png".to_string(), "test.webp".to_string());
  }
  #[test]
  fn web_png_to_webp() {
    web_image_to_webp(
      "https://pic1.zhimg.com/v2-f2124f32356af878afa887652bfe082e_1440w.jpg?source=172ae18b"
        .to_string(),
      "test_web.webp".to_string(),
    );
  }
}
