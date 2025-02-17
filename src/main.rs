use whisper_rs::{FullParams, SamplingStrategy, WhisperContext, WhisperContextParameters};

mod airlines;
mod aviation_command;
mod errors;

const AIRLINES_JSON_PATH: &str = "resources/known-strings/airlines.json";
const ALPHABET_JSON_PATH: &str = "resources/known-strings/alphabet.json";
const MODEL_PATH: &str = "resources/models/ggml-base.en.bin";

fn main() {
    let ctx = WhisperContext::new_with_params(MODEL_PATH, WhisperContextParameters::default())
        .expect("failed to open model");
    let mut state = ctx.create_state().expect("failed to create key");

    let mut params = FullParams::new(SamplingStrategy::default());

    params.set_initial_prompt("experience");
    params.set_progress_callback_safe(|progress| println!("Progress callback: {}%", progress));
    
}
