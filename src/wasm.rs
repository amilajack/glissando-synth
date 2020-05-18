extern crate wasm_bindgen;
//extern crate wasm_bindgen_futures;

use wasm_bindgen::prelude::*;
//use futures::executor::block_on;
use web_sys::*;

static MAX_GAIN: f32 = 1.0;
static OSC_FREQ: f32 = 440.0;
static ANGULAR_FREQUENCY: f32 = 2.0 * std::f32::consts::PI * OSC_FREQ;

#[wasm_bindgen]
pub fn generate_sample(sample_number: f32, sample_rate: f32) -> f32 {
    let sample_time = sample_number / sample_rate;
    let sample_angle = sample_time * ANGULAR_FREQUENCY;
    return f32::sin(sample_angle);
}

fn fill_samples(audio_buffer: &AudioBuffer, sample_rate: f32) {
    // Fill the buffer with a sine wave
    let mut left_channel = audio_buffer.get_channel_data(0).unwrap();
    let mut right_channel = audio_buffer.get_channel_data(1).unwrap();
    for i in 0..audio_buffer.length() {
        let sample_time = i as f32 / sample_rate;
        let sample_angle = sample_time * ANGULAR_FREQUENCY;
        let sample = f32::sin(sample_angle);
        left_channel[i as usize] = sample;
        right_channel[i as usize] = sample;
    }
    audio_buffer.copy_to_channel(&mut left_channel, 0).expect("Could not copy channel");
    audio_buffer.copy_to_channel(&mut right_channel, 1).expect("Could not copy channel");
}

/*async fn audio_worklet_add_module(ctx: &AudioContext) -> Result<JsValue, JsValue> {
    let _audio_worklet = ctx.audio_worklet()?;
    let promise = _audio_worklet.add_module("/packages/glissando-app/white-noise-processor.js")?;
    let result = wasm_bindgen_futures::JsFuture::from(promise).await?;
    Ok(result)
}*/

#[wasm_bindgen]
pub struct Osc {
    ctx: AudioContext,
    osc_amp: GainNode,
    audio_buffer_amp: GainNode,
    //white_noise_amp: GainNode,
}

impl Drop for Osc {
    fn drop(&mut self) {
        let _ = self.ctx.close();
    }
}

#[wasm_bindgen]
impl Osc {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Result<Osc, JsValue> {
        let ctx = AudioContext::new()?;

        let osc = ctx.create_oscillator()?;
        let osc_amp = ctx.create_gain()?;
        osc.frequency().set_value_at_time(OSC_FREQ, ctx.current_time())?;
        osc_amp.gain().set_value_at_time(MAX_GAIN / 10.0, ctx.current_time())?;
        osc.connect_with_audio_node(&osc_amp)?;
        osc_amp.connect_with_audio_node(&ctx.destination())?;

        let audio_buffer_source = ctx.create_buffer_source()?;
        let audio_buffer = ctx.create_buffer(2, ctx.sample_rate() as u32 * 15, ctx.sample_rate())?;
        let audio_buffer_amp = ctx.create_gain()?;
        fill_samples(&audio_buffer, ctx.sample_rate());
        audio_buffer_source.set_buffer(Some(&audio_buffer));
        audio_buffer_amp.gain().set_value_at_time(MAX_GAIN / 10.0, ctx.current_time())?;
        audio_buffer_source.connect_with_audio_node(&audio_buffer_amp)?;
        audio_buffer_amp.connect_with_audio_node(&ctx.destination())?;

        //let white_noise_amp = ctx.create_gain()?;
        //block_on(audio_worklet_add_module(&ctx))?;
        //let white_noise_node = AudioWorkletNode::new(&ctx, "white-noise-processor")?;
        //white_noise_amp.gain().set_value_at_time(MAX_GAIN / 10.0, ctx.current_time())?;
        //white_noise_node.connect_with_audio_node(&white_noise_amp)?;
        //white_noise_amp.connect_with_audio_node(&ctx.destination())?;

        osc.start()?;
        audio_buffer_source.start()?;

        Ok(Osc {
            ctx,
            osc_amp,
            audio_buffer_amp,
            //white_noise_amp,
        })
    }

    #[wasm_bindgen]
    pub fn set_osc_amp(&self, mut gain: f32) {
        if gain > 1.0 {
            gain = 1.0;
        }
        if gain < 0.0 {
            gain = 0.0;
        }
        self.osc_amp.gain().set_value(gain);
    }

    #[wasm_bindgen]
    pub fn set_audio_buffer_amp(&self, mut gain: f32) {
        if gain > 1.0 {
            gain = 1.0;
        }
        if gain < 0.0 {
            gain = 0.0;
        }
        self.audio_buffer_amp.gain().set_value(gain);
    }

    /*#[wasm_bindgen]
    pub fn set_white_noise_amp(&self, mut gain: f32) {
        if gain > 1.0 {
            gain = 1.0;
        }
        if gain < 0.0 {
            gain = 0.0;
        }
        self.white_noise_amp.gain().set_value(gain);
    }*/

    #[wasm_bindgen]
    pub fn suspend(&self) {
        self.ctx.suspend();
    }

    #[wasm_bindgen]
    pub fn resume(&self) {
        self.ctx.resume();
    }
}
