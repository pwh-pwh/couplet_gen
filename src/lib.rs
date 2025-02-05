pub mod gpt_client;

pub mod image_gen;

pub use gpt_client::gen_couplet_by_gpt;

pub use image_gen::gen_couplet;

pub use image_gen::Couplet;