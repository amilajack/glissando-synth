extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

static OSC_FREQ: f32 = 440.0;
static ANGULAR_FREQUENCY: f32 = 2.0 * OSC_FREQ * 2.0 * std::f32::consts::PI;

#[wasm_bindgen]
pub fn generate_sample(sample_number: f32, sample_rate: f32) -> f32 {
    let sample_time = sample_number / sample_rate;
    let sample_angle = sample_time * ANGULAR_FREQUENCY;
    return f32::sin(sample_angle);
}

#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}