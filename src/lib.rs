#![feature(vec_into_raw_parts)]

use serde::Deserialize;
use spreet::{Sprite, Spritesheet};
use std::{collections::BTreeMap, path::PathBuf};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PluginOptions {
    pub input: PathBuf,
    pub output: PathBuf,
    pub files: Vec<String>,
    pub ratio: Option<u8>,
    pub retina: Option<bool>,
    pub unique: Option<bool>,
    pub minify_index_file: Option<bool>,
    pub sdf: Option<bool>,
}

/// Plugin.
///
/// # Note
/// This is mostly just a 1:1 translation from the cli for spreet.
/// See [spreet/main.rs](https://github.com/flother/spreet/blob/master/src/bin/spreet/main.rs).
// #[wasm_bindgen]
#[no_mangle]
pub fn plugin(buf: *mut u8, len: usize) {
    // SAFETY: we trust that only our js code calls this
    let options = unsafe { load_string(buf, len) };
    let options: PluginOptions = serde_json::from_str(&options).unwrap();

    let output = options
        .output
        .file_name()
        .map(|s| s.to_string_lossy())
        .unwrap_or_default();
    let output = PathBuf::from(format!("/output/{output}"));

    // The ratio between the pixels in an SVG image and the pixels in the resulting PNG sprite.
    // A value of 2 means the PNGs will be double the size of the SVG images.
    let pixel_ratio = match (options.retina, options.ratio) {
        (Some(true), _) => 2,
        (_, Some(ratio)) => ratio,
        _ => 1,
    };

    // let mut out = std::fs::read_to_string("/input/ab.svg").unwrap();
    // for dir in std::fs::read_dir("/input").unwrap() {
    //     let dir = dir.unwrap();
    // };

    // std::fs::write(output, out).unwrap();
    // return;

    let sprites = options
        .files
        .iter()
        .map(|file| {
            let svg_path = format!("/input/{file}");
            if let Ok(tree) = spreet::load_svg(&svg_path) {
                let sprite = if options.sdf.unwrap_or(false) {
                    Sprite::new_sdf(tree, pixel_ratio).expect("failed to load an SDF sprite")
                } else {
                    Sprite::new(tree, pixel_ratio).expect("failed to load a sprite")
                };
                let name = file.strip_suffix(".svg").unwrap().to_string();
                (name, sprite)
            } else {
                panic!("{svg_path:?}: not a valid SVG image");
            }
        })
        .collect::<BTreeMap<String, Sprite>>();

    if sprites.is_empty() {
        panic!("no valid SVGs found in {:?}", options.input);
    }

    let mut spritesheet_builder = Spritesheet::build();
    spritesheet_builder.sprites(sprites);
    if options.unique.unwrap_or(false) {
        spritesheet_builder.make_unique();
    };
    if options.sdf.unwrap_or(false) {
        spritesheet_builder.make_sdf();
    };

    // Generate sprite sheet
    let Some(spritesheet) = spritesheet_builder.generate() else {
        panic!("could not pack the sprites within an area fifty times their size.");
    };

    // Save the bitmapped spritesheet to a local PNG.
    let file_prefix = output.to_string_lossy();
    let file_prefix: &str = &file_prefix;
    let spritesheet_path = format!("{file_prefix}.png");
    if let Err(e) = spritesheet.save_spritesheet(&spritesheet_path) {
        panic!("could not save spritesheet to {spritesheet_path} ({e})");
    };

    // Save the index file to a local JSON file with the same name as the spritesheet.
    if let Err(e) = spritesheet.save_index(&file_prefix, options.minify_index_file.unwrap_or(false))
    {
        panic!("could not save sprite index to {file_prefix} ({e})");
    };
}

#[no_mangle]
pub fn _start() {}

#[no_mangle]
pub fn alloc_string(len: usize) -> *mut u8 {
    let string = String::with_capacity(len);
    let (ptr, _, _) = string.into_raw_parts();
    ptr
}

unsafe fn load_string(buf: *mut u8, len: usize) -> String {
    let buf = Vec::from_raw_parts(buf, len, len);
    String::from_utf8(buf).expect("valid utf-8 string")
}
