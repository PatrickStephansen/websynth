extern crate rand;

use rand::{thread_rng, Rng};

#[no_mangle]
pub extern fn add_one(a: u32) -> u32 {
    a + 1
}

#[no_mangle]
pub extern fn random_value() -> f32 {
    thread_rng().gen_range(-1.0f32, 1.0f32)
}
