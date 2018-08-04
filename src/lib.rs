#![feature(use_extern_macros)]

extern crate wasm_bindgen;
extern crate rand;

use wasm_bindgen::prelude::*;
use rand::{thread_rng, Rng};

#[wasm_bindgen]
pub extern fn add_one(a: u32) -> u32 {
  a + 1
}

#[wasm_bindgen]
pub extern fn random_value() -> f32 {
  thread_rng().gen_range(-1.0f32, 1.0f32)
}

#[wasm_bindgen]
pub extern fn generate_whitenoise(samples: u32) -> Vec<f32> {
  let mut vec = Vec::new();
  for _ in 0..samples{
    vec.push(random_value())
  }
  vec
}

