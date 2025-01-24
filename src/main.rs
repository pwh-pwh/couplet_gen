mod image_gen;

use ab_glyph::{Font, FontRef, PxScale};
use image::{ImageReader, Pixel, Rgba};
use imageproc::definitions::Clamp;
use imageproc::drawing::{draw_text_mut, text_size, Canvas};
use crate::image_gen::{gen_couplet, Couplet};

fn main() {
    gen_couplet(&Couplet::new("Rust无敌".to_string(), "并发性能铸匠魂".to_string(), "安全高效为本心".to_string()));
}