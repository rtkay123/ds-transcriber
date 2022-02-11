#![warn(missing_debug_implementations)]
#![warn(rustdoc::missing_doc_code_examples)]
#![warn(rustdoc::broken_intra_doc_links)]
#![warn(missing_docs)]

//! ds-transcriber
//! # A deepspeech powered utility for transcribing a microphone stream
//!
//! # Prelude
//!
//! You can think of this crate as a wrapper for RustAudio's
//! [deepspeech-rs](https://github.com/rustaudio/deepspeech-rs). It aims to provide transcription
//! for microphone audio with optional denoising.
//!
//!
//! #### Getting Started
//!
//! This example shows the quickest way to get started with ds-transcriber.
//! First, add ds-transcriber to your `Cargo.toml`
//!
//! ```toml
//! ds-transcriber = "1"
//! ```
//!
//! Download the DeepSpeech [native client](https://github.com/mozilla/DeepSpeech/releases/tag/v0.9.0) and then add its directory to your `LD_LIBRARY_PATH` and
//! `LIBRARY_PATH` variables.
//!
//! ```no_run
//! # fn main()->Result<(),Box<dyn std::error::Error>>{
//! let mut model = ds_transcriber::model::DeepSpeechModel::new("path_to_ds_model")?;
//! let ds_model = model.prepared_model();
//! let mut config = ds_transcriber::StreamSettings {
//!     //value used for pause detection, a pause is detected when the amplitude is less than this
//!     silence_level: 200,
//!     // takes a reference of the model we instantiated earlier
//!     model: ds_model,
//!     // show the amplitude values on stdout (helps you to find your silence level)
//!     show_amplitudes: true,
//!     // seconds of silence indicating end of speech (begin transcribe when pause_length is grater than....)
//!     pause_length: 1.0,
//! };
//! let i_said = ds_transcriber::transcribe(&mut config)?;
//! println!("I said: {}", i_said);
//! # Ok(())
//! # }
//! ```
//! Rinse and repeat the last two lines
//!
//! # Features
//
//! This crate provides an optional feature of denoising of the audio stream (may result in better
//! transcription). It is disabled by default, to enable it: use either the `denoise` or `full` key
//! in the crate's features list
//!
//! ```toml
//! ds-transcriber = { version = "1", features = ["denoise"] } # or features = ["full"]
//! ```
//!
//! # Crate example
//! Clone the repository and run the example
//!
//! For help with arguments, run:
//! ```sh
//! cargo run --example transcribe -- -h
//! ```
//!
//! To start the example, run
//! ```sh
//! cargo run --example transcribe -- -m model_path_dir -c deepspeech_native_client_dir
//! ```
//!
//! # Re-exports
//!
//! This crate also re-exports the `deepspeech` and `nnnoiseless` crates (if the `denoise` feature is enabled). You can use these re-exports instead of also depending on them separately.
//!
mod config;
pub mod model;
mod stream;
/// Transcription workers
pub use transcriber::{transcribe, StreamSettings};
mod transcriber;

/// # Re-exports
pub use deepspeech;
#[cfg(feature = "denoise")]
pub use nnnoiseless;
