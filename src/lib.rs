#![deny(clippy::all)]

use clipboard_rs::{common::RustImage, Clipboard, ClipboardContext, RustImageData};
use napi::bindgen_prelude::Buffer;

#[macro_use]
extern crate napi_derive;

// ---------- read

#[napi]
pub fn read_text() -> Option<String> {
  let ctx = ClipboardContext::new().ok()?;

  let text = ctx.get_text().ok();

  text
}

#[napi]
pub fn read_files() -> Option<Vec<String>> {
  let ctx = ClipboardContext::new().ok()?;

  let files = ctx.get_files().ok();

  files
}

#[napi]
pub fn read_image() -> Option<Buffer> {
  let ctx = ClipboardContext::new().ok()?;

  let img = ctx.get_image().ok()?;

  let png = img.to_png().ok()?;

  let buf: Buffer = png.get_bytes().into();

  Some(buf)
}

// --------- write

#[napi]
pub fn write_text(text: String) -> Option<bool> {
  let ctx = ClipboardContext::new().ok()?;

  let _ = ctx.set_text(text).ok();

  Some(true)
}

#[napi]
pub fn write_files(files: Vec<String>) -> Option<bool> {
  let ctx = ClipboardContext::new().ok()?;

  let _ = ctx.set_files(files).ok();

  Some(true)
}

#[napi]
pub fn write_image(img: Buffer) -> Option<bool> {
  let ctx = ClipboardContext::new().ok()?;

  let image_data = RustImageData::from_bytes(&img.to_vec()).ok()?;

  let _ = ctx.set_image(image_data).ok();

  Some(true)
}

#[cfg(test)]
mod test {
  use crate::{read_text, write_text};

  #[test]
  fn read_and_write_str() {
    let result = write_text(("123").to_string());
    assert_eq!(result, Some(true));

    let t = read_text();
    assert_eq!(t, Some("123".to_string()));
  }
}
