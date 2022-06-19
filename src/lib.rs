#![deny(clippy::all)]

use std::{fs, io::Write};

mod webp;

#[macro_use]
extern crate napi_derive;

#[napi]
fn image_to_webp(path: String, dist_path: String) -> bool {
  let webp_file_content = webp::image_to_webp(path).unwrap();
  let mut webp_file = fs::File::create(dist_path).unwrap();
  webp_file.write_all(&webp_file_content.to_vec()).unwrap();
  true
}

#[cfg(test)]
mod tests {

  use crate::image_to_webp;

  #[test]
  fn png_to_webp() {
    image_to_webp("test.png".to_string(), "test.webp".to_string());
  }
}
