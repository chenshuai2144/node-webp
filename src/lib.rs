#![deny(clippy::all)]
use regex::Regex;
use std::{
  fs::File,
  io::{Read, Write},
};
mod webp;
use image::io::Reader as ImageReader;
use reqwest;

#[macro_use]
extern crate napi_derive;

// 将本地的图片转化为 webp
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
  let mut webp_file = File::create(dist_path).unwrap();
  webp_file.write_all(&webp_file_content.to_vec()).unwrap();
  true
}
// 将 url 图片下载，并且保存为 webp
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
  let mut webp_file = File::create(dist_path).unwrap();
  webp_file.write_all(&webp_file_content.to_vec()).unwrap();
  true
}

#[napi]
fn find_all_url(path: String) -> Vec<String> {
  let re = Regex::new(
    r"(http[s]?://(?:[a-zA-Z]|[0-9]|[$-_@.&+]|[!*\(\),]|(?:%[0-9a-fA-F][0-9a-fA-F]))+)'",
  )
  .unwrap();
  let mut contents = String::new();
  File::open(path)
    .unwrap()
    .read_to_string(&mut contents)
    .unwrap();

  let mut urls: Vec<String> = Vec::new();

  for line in contents.lines() {
    if line.contains("http") {
      re.captures_iter(line).for_each(|cap| {
        urls.push(cap.get(1).unwrap().as_str().to_string());
      });
    }
  }

  urls
}

#[cfg(test)]
mod tests {

  use crate::find_all_url;
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
  #[test]
  fn find_all_url_ts() {
    let url_list = find_all_url("common.ts.bk".to_string());
    assert_eq!(url_list.len(), 8);
  }
}
