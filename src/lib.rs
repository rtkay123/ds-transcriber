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
//! ds-transcriber = "1.0.0-beta"
//! ```
//!
//! Download the DeepSpeech [native client](https://github.com/mozilla/DeepSpeech/releases/tag/v0.9.0) and then add its directory to your `LD_LIBRARY_PATH` and
//! `LIBRARY_PATH` variables.
//!
//! Have a look at [StreamSettings](StreamSettings) to fine tune the transcription stream to parameters that better suite
//! your environment
//!
//! ```no_run
//! # use std::{path::PathBuf, str::FromStr};
//! # fn main()->Result<(),Box<dyn std::error::Error>>{
//! let mut model = ds_transcriber::model::instance_model(
//!     "model_file.pbmm",
//!     Some(PathBuf::from_str("scorer_file.scorer")?.into_boxed_path()),
//! )?;
//! let config = ds_transcriber::StreamSettings::default();
//! let i_said = ds_transcriber::transcribe(config, &mut model)?;
//! println!("I said: {}", i_said);
//! # Ok(())
//! # }
//! ```
//!
//! Rinse and repeat the last two lines
//!
//! # Features
//
//! This crate provides an optional feature of denoising of the audio stream (may result in better
//! transcription). It is disabled by default, to enable it: use either the `denoise` or `full` key
//! in the crate's features list
//!
//! ```toml
//! ds-transcriber = { version = "1.0.0-beta", features = ["denoise"] } # or features = ["full"]
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
//! cargo run --example transcribe -- -m model_file -s scorer_file -c deepspeech_native_client_dir
//! ```
//! The `s` or `--scorer` argument is optional, but it is recommended to set one.
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
