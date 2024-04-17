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

  let _r = ctx.set_text(text).ok();

  Some(true)
}

#[napi]
pub fn write_files(files: Vec<String>) -> Option<bool> {
  let ctx = ClipboardContext::new().ok()?;

  let _r = ctx.set_files(files).ok();

  Some(true)
}

#[napi]
pub fn write_image(img: Buffer) -> Option<bool> {
  let ctx = ClipboardContext::new().ok()?;

  let image_data = RustImageData::from_bytes(&img.to_vec()).ok()?;

  let img = ctx.set_image(image_data);

  Some(img.is_ok())
}
