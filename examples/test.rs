extern crate portaudio;
extern crate sample;

const CHANNELS: i32 = 2;
const FRAMES: u32 = 64;
const SAMPLE_HZ: f64 = 44_100.0;
const DURATION: usize = 2;
const N_FRAMES: usize = ((SAMPLE_HZ as usize) * DURATION) as usize;
const SIGNAL_HZ: f64 = 440.0;

fn main() {
  run().unwrap()
}

fn square_wave(time: f64) -> f32 {
  let period = 1.0 / SIGNAL_HZ;
  let ratio = time / period;

  if ratio.fract() <= 0.5 {
    1.0
  } else {
    -1.0
  }
}

fn list_devices(pa: &portaudio::PortAudio) -> Result<(), portaudio::Error> {
  println!("Devices");
  println!("=======");

  for device in pa.devices()? {
    let (device_index, device_info) = device?;
    println!("{}: {}", device_index.0, device_info.name);
  }

  Ok(())
}

fn run() -> Result<(), portaudio::Error> {
  let pa = portaudio::PortAudio::new()?;
  list_devices(&pa)?;

  let settings = pa.default_output_stream_settings::<f32>(CHANNELS, SAMPLE_HZ, FRAMES)?;

  let mut frames_count = 0;
  let mut time = 0.0;

  let callback = move |portaudio::OutputStreamCallbackArgs { buffer, frames, .. }| {
    let buffer: &mut [[f32; CHANNELS as usize]] =
      sample::slice::to_frame_slice_mut(buffer).unwrap();

    for frame in buffer {
      let val = square_wave(time);

      frame[0] = val;
      frame[1] = val;

      time += 1.0 / SAMPLE_HZ;
    }

    frames_count += frames;

    if frames_count >= N_FRAMES {
      portaudio::Complete
    } else {
      portaudio::Continue
    }
  };

  let mut stream = pa.open_non_blocking_stream(settings, callback)?;
  stream.start()?;

  while let Ok(true) = stream.is_active() {
    std::thread::sleep(std::time::Duration::from_millis(16));
  }

  Ok(())
}
