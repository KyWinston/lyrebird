use bevy::{
    prelude::Image,
    render::{
        render_asset::RenderAssetUsages,
        texture::{CompressedImageFormats, ImageSampler, ImageType},
    },
    text::Font,
};

pub fn load_font(bytes: &[u8], _path: String) -> Font {
    Font::try_from_bytes(bytes.to_vec()).unwrap()
}

pub fn load_image(bytes: &[u8], _path: String) -> Image {
    const IS_SRGB: bool = true;
    Image::from_buffer(
        bytes,
        ImageType::Extension("png"),
        CompressedImageFormats::NONE,
        IS_SRGB,
        ImageSampler::Default,
        RenderAssetUsages::RENDER_WORLD,
    )
    .unwrap()
}
