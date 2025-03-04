use serde::{Deserialize, Serialize};
use spreet::{resvg::usvg::{self, TreeParsing}, Sprite, Spritesheet};
use thiserror::Error;
use std::collections::BTreeMap;
use wasm_bindgen::prelude::*;
use tsify_next::Tsify;
use serde_bytes::ByteBuf;

#[derive(Debug, Deserialize, Tsify, Default)]
#[serde(rename_all = "camelCase")]
#[tsify(from_wasm_abi)]
pub struct Options {
    #[tsify(optional)]
    pub ratio: Option<u8>,
    #[tsify(optional)]
    pub retina: Option<bool>,
    #[tsify(optional)]
    pub unique: Option<bool>,
    #[tsify(optional)]
    pub minify_index_file: Option<bool>,
    #[tsify(optional)]
    pub sdf: Option<bool>,
    #[tsify(optional)]
    pub pretty_json: Option<bool>,
}

#[derive(Debug, Deserialize, Tsify)]
#[tsify(from_wasm_abi)]
pub struct SpriteSvg {
    // final name of sprite, truncating the ".svg" needs to be handled on the js side
    pub name: String,
    pub content: ByteBuf,
}

#[derive(Serialize, Tsify)]
#[tsify(into_wasm_abi)]
pub struct Output {
    #[serde(with = "serde_bytes")]
    pub png: ByteBuf,
    pub json: String,
}

#[wasm_bindgen]
pub fn spreet_impl(files: Vec<SpriteSvg>, options: Option<Options>) -> Result<Output, Error> {
    let options = options.unwrap_or_default();

    // The ratio between the pixels in an SVG image and the pixels in the resulting PNG sprite.
    // A value of 2 means the PNGs will be double the size of the SVG images.
    let pixel_ratio = match (options.retina, options.ratio) {
        (Some(true), _) => 2,
        (_, Some(ratio)) => ratio,
        _ => 1,
    };

    let sprites = files.iter().map(|SpriteSvg { name, content }| {
        let tree = usvg::Tree::from_data(content, &usvg::Options::default())?;
        let sprite = match options.sdf {
            Some(true) => Sprite::new_sdf(tree, pixel_ratio).ok_or_else(|| Error::SdfSprite(name.clone()))?,
            _ => Sprite::new(tree, pixel_ratio).ok_or_else(|| Error::Sprite(name.clone()))?,
        };
        Ok((name.clone(), sprite))
    }).collect::<Result<BTreeMap<String, Sprite>, Error>>()?;

    let mut spritesheet_builder = Spritesheet::build();
    spritesheet_builder.sprites(sprites);
    if options.unique.unwrap_or(false) {
        spritesheet_builder.make_unique();
    };
    if options.sdf.unwrap_or(false) {
        spritesheet_builder.make_sdf();
    };

    let spritesheet = spritesheet_builder.generate().ok_or(Error::Generate)?;
    let png = spritesheet.encode_png()?.into();
    let index = spritesheet.get_index().clone();
    let json = match options.pretty_json {
        Some(true) => serde_json::to_string_pretty(&index).unwrap(),
        _ => serde_json::to_string(&index).unwrap(),
    };
    Ok(Output {png, json})
}

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Usvg(#[from] usvg::Error),
    #[error(transparent)]
    Spreet(#[from] spreet::SpreetError),
    #[error("failed to load SDF sprite: {0}")]
    SdfSprite(String),
    #[error("failed to load sprite: {0}")]
    Sprite(String),
    #[error("could not pack the sprites within an area fifty times their size")]
    Generate,
}

impl From<Error> for JsValue {
    fn from(value: Error) -> Self {
        let msg = value.to_string();

        use ImplErrorKind as Kind;
        let (kind, sprite) = match value {
            Error::Usvg(_) => (Kind::Usvg, None),
            Error::Spreet(_) => (Kind::Spreet, None),
            Error::SdfSprite(sprite) => (Kind::SdfSprite, Some(sprite)),
            Error::Sprite(sprite) => (Kind::Sprite, Some(sprite)),
            Error::Generate => (Kind::Generate, None),
        };

        serde_wasm_bindgen::to_value(&ImplError {
            kind,
            msg,
            sprite,
        }).unwrap()
    }
}

#[derive(Debug, Serialize, Tsify)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ImplErrorKind {
    Usvg,
    Spreet,
    SdfSprite,
    Sprite,
    Generate,
}

#[derive(Debug, Serialize, Tsify)]
pub struct ImplError {
    pub kind: ImplErrorKind,
    pub msg: String,
    pub sprite: Option<String>,
}
