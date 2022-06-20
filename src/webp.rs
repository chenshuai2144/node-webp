use image::DynamicImage;
use webp::{Encoder, WebPMemory};

use std::error::Error;

/// Converts an image from any `image`-supported format to WEBP.
/// # Parameters
/// * `file_path`: Path to the image to convert.
/// # Returns
/// `Result<PathBuf, io::Error>`: Path of the WEBP-image as a `PathBuf` when
///  succesful, otherwise an `Error`.
pub fn image_to_webp(
  image: DynamicImage,
) -> Result<WebPMemory, Box<dyn Error + Send + Sync + 'static>> {
  // Make webp::Encoder from DynamicImage.
  let encoder: Encoder = Encoder::from_image(&image)?;

  // Encode image into WebPMemory.
  let encoded_webp: WebPMemory = encoder.encode(65f32);

  Ok(encoded_webp)
}
