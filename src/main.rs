use couplet_gen::gpt_client::gen_couplet_by_gpt;
use couplet_gen::image_gen::{gen_couplet, Couplet};

fn main() {
    let couplet = gen_couplet_by_gpt("发财");
    gen_couplet(&Couplet::new(
        couplet.title,
        couplet.top,
        couplet.bottom,
    ));
}
