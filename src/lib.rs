#![deny(clippy::all)]

mod webp;


#[macro_use]
extern crate napi_derive;

#[napi]
fn image_to_webp(path: String, quality_factor: f64) -> bool {
  webp::image_to_webp(path).unwrap()
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
