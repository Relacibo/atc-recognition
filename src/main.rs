use std::sync::{Arc, Mutex};

use cpal::traits::{DeviceTrait, HostTrait};
use ringbuf::{
    HeapRb,
    traits::{Producer, Split},
};
use whisper_rs::{FullParams, SamplingStrategy, WhisperContext, WhisperContextParameters};

mod airlines;
mod aviation_command;
mod errors;
use rubato::{
    Resampler, SincFixedIn, SincInterpolationParameters, SincInterpolationType, WindowFunction,
};

const AIRLINES_JSON_PATH: &str = "resources/known-strings/airlines.json";
const ALPHABET_JSON_PATH: &str = "resources/known-strings/alphabet.json";
const MODEL_PATH: &str = "resources/models/ggml-base.en.bin";

const SAMPLE_RATE_HZ: u32 = 16000;

pub fn create_resampler(sample_rate_in: u32) -> SincFixedIn<f32> {
    let params = SincInterpolationParameters {
        sinc_len: 256,
        f_cutoff: 0.95,
        interpolation: SincInterpolationType::Linear,
        oversampling_factor: 256,
        window: WindowFunction::BlackmanHarris2,
    };
    let resample_ratio = SAMPLE_RATE_HZ as f64 / sample_rate_in as f64;
    SincFixedIn::<f32>::new(resample_ratio, 2.0, params, 1024, 1).unwrap()
}

fn main() -> Result<(), crate::errors::Error> {
    let cpal_host = cpal::default_host();
    let input_device = cpal_host
        .default_input_device()
        .ok_or(crate::errors::Error::FailedToFindDefaultInputDevice)?;

    #[cfg(debug_assertions)]
    if let Ok(name) = input_device.name() {
        println!("Using input device: {}", name);
    }

    // We'll try and use the same configuration between streams to keep it simple.
    let config: cpal::StreamConfig = input_device.default_input_config()?.into();

    let sample_rate_in = config.sample_rate.0;
    let channel_count_in = config.channels;

    // 2 seconds
    let latency_samples = SAMPLE_RATE_HZ * 2;
    // The buffer to share samples. We can buffer 16 seconds maximum.
    let ring = HeapRb::<f32>::new(latency_samples as usize * 8);
    let (mut producer, mut consumer) = ring.split();

    let resample_buffer: Arc<Mutex<[Vec<f32>; 1]>> = Arc::new(Mutex::new([vec![]]));

    // Fill the samples with 0.0 equal to the length of the delay.
    for _ in 0..latency_samples {
        // The ring buffer has twice as much space as necessary to add latency here,
        // so this should never fail
        producer.try_push(0.0).unwrap();
    }

    let input_data_fn = move |data: &[f32], _: &cpal::InputCallbackInfo| {
        let Ok(mut rb) = resample_buffer.lock() else {
            eprintln!("Could not lock mutex");
            return;
        };
        rb[0].clear();

        let data2 = if sample_rate_in != SAMPLE_RATE_HZ {
            let mut resampler = create_resampler(sample_rate_in);
            if let Err(err) = resampler.process_into_buffer(
                &[data
                    .iter()
                    .skip(channel_count_in.into())
                    .copied()
                    .collect::<Vec<_>>()],
                rb.as_mut_slice(),
                None,
            ) {
                eprintln!("Rubato resampling failed");
                dbg!(err);
                return;
            }
            &rb[0][..]
        } else {
            data
        };
        let len = data2.len();
        let pushed_count = producer.push_slice(data2);
        if len - pushed_count != 0 {
            eprintln!("Mic buffer overflow");
        }
    };
    let _input_stream = input_device.build_input_stream(&config, input_data_fn, err_fn, None)?;

    // let ctx = WhisperContext::new_with_params(MODEL_PATH, WhisperContextParameters::default())
    //     .expect("failed to open model");
    // let mut state = ctx.create_state().expect("failed to create key");

    // let mut params = FullParams::new(SamplingStrategy::default());

    // params.set_initial_prompt("experience");
    // params.set_progress_callback_safe(|progress| println!("Progress callback: {}%", progress));
    Ok(())
}

fn err_fn(err: cpal::StreamError) {
    eprintln!("an error occurred on stream: {}", err);
}
