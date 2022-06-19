use image::{DynamicImage, GenericImageView};

pub fn read(path: String) -> Result<DynamicImage, String> {
  // Use the open function to load an image from a Path.
  // `open` returns a `DynamicImage` on success.
  let img = image::open(path).unwrap();

  // The dimensions method returns the images width and height.
  println!("dimensions {:?}", img.dimensions());

  Ok(img)
}
