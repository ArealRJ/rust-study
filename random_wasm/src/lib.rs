extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;
use rand::Rng;

#[wasm_bindgen]
pub fn generate_random_number() -> u32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(0..=1000)
}
