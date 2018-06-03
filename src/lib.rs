#![feature(proc_macro, wasm_custom_section, wasm_import_module)]

extern crate rand;
extern crate wasm_bindgen;

use rand::{thread_rng, Rng};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn add_one(a: u32) -> u32 {
  a + 1
}

#[wasm_bindgen]
pub fn random_value() -> f32 {
  thread_rng().gen_range(-1.0f32, 1.0f32)
}

#[wasm_bindgen]
pub fn generate_whitenoise(samples: u32) -> Vec<f32> {
  let mut vec = Vec::new();
  for _ in 0..samples{
    vec.push(random_value())
  }
  vec
}

