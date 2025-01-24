use ab_glyph::{Font, FontRef, PxScale};
use image::{ImageReader, Pixel, Rgba};
use imageproc::definitions::Clamp;
use imageproc::drawing::{draw_text_mut, text_size, Canvas};

pub struct Couplet {
    pub title: String,
    pub top: String,
    pub bottom: String,
}

impl Couplet {
    pub fn new(title: String, top: String, bottom: String) -> Self {
        Self { title, top, bottom }
    }
}

pub fn gen_couplet(couplet: &Couplet) {
    let mut image = ImageReader::open("r1.png").unwrap().decode().unwrap().to_rgba8();

    // 使用 imageproc 填充整个图像为白色
    // let white = Rgb([255, 255, 255]);
    // draw_filled_rect_mut(&mut image, Rect::at(0, 0).of_size(707, 1024), white);
    let font = FontRef::try_from_slice(include_bytes!("../STXINGKA.TTF")).unwrap();

    let size = 80.0;
    let scale = PxScale {
        x: size,
        y: size,
    };

    let text = "Rust无敌";
    draw_text_mut(&mut image, Rgba([0u8, 0u8, 0u8,200u8]), 200, 85, scale, &font, &couplet.title);
    draw_text_mut_vertical(&mut image, Rgba([0u8, 0u8, 0u8,200u8]), 90, 300, scale, &font, &couplet.bottom);
    draw_text_mut_vertical(&mut image, Rgba([0u8, 0u8, 0u8,200u8]), 535, 300, scale, &font, &couplet.top);
    let (w, h) = text_size(scale, &font, text);
    println!("Text size: {}x{}", w, h);
    image.save("result.png").unwrap();
}
pub fn draw_text_mut_vertical<C>(
    canvas: &mut C,
    color: C::Pixel,
    x: i32,
    mut y: i32,
    scale: impl Into<PxScale> + Copy,
    font: &impl Font,
    text: &str,
) where
    C: Canvas,
    <C::Pixel as Pixel>::Subpixel: Into<f32> + Clamp<f32>,
{
    for c in text.chars() {
        draw_text_mut(canvas, color, x, y, scale, &font, &c.to_string());
        let (_, h) = text_size(scale, &font, &c.to_string());
        y += h as i32 + 15; // 根据字符的高度调整y坐标
    }
}