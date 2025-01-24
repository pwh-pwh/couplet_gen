mod image_gen;

use crate::image_gen::{gen_couplet, Couplet};
use ab_glyph::Font;
use image::Pixel;
use imageproc::definitions::Clamp;
use imageproc::drawing::Canvas;

fn main() {
    gen_couplet(&Couplet::new(
        "Rust无敌".to_string(),
        "并发性能铸匠魂".to_string(),
        "安全高效为本心".to_string(),
    ));
}
