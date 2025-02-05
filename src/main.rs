mod image_gen;
mod gpt_client;

use crate::image_gen::{gen_couplet, Couplet};
use ab_glyph::Font;
use image::Pixel;
use imageproc::definitions::Clamp;
use imageproc::drawing::Canvas;
use crate::gpt_client::gen_couplet_by_gpt;

fn main() {
    let couplet = gen_couplet_by_gpt("发财");
    gen_couplet(&Couplet::new(
        couplet.title,
        couplet.top,
        couplet.bottom,
    ));
}
