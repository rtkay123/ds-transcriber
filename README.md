# ds-transcriber
![GitHub](https://img.shields.io/github/license/kawaki-san/ds-transcriber) ![Crates.io](https://img.shields.io/crates/v/ds-transcriber) ![docs.rs](https://img.shields.io/docsrs/ds-transcriber)
## A DeepSpeech powered utility for getting microphone stream transcriptions

## Prelude

You can think of this crate as a wrapper for RustAudio's [deepspeech-rs](https://github.com/rustaudio/deepspeech-rs). It aims to provide transcription for microphone streams with optional denoising see `cargo-features` below.

##### Getting Started

This example shows the quickest way to get started with ds-transcriber. First, add `ds-transcriber` to your `Cargo.toml`

```toml
ds-transcriber = "1"
```

Download the DeepSpeech [native client](https://github.com/mozilla/DeepSpeech/releases/tag/v0.9.0) and then add its directory to your `LD_LIBRARY_PATH` and `LIBRARY_PATH` variables.

Have a look at [StreamSettings](StreamSettings) to fine tune the transcription stream to parameters that better suit
your environment

```rust
let mut model = ds_transcriber::model::instance_model(
    "model_file.pbmm",
    Some("scorer_file.scorer"),
)?;
let config = ds_transcriber::StreamSettings::default();
let i_said = ds_transcriber::transcribe(config, &mut model)?;
println!("I said: {}", i_said);
```
Rinse and repeat the last two lines

## Cargo Features
This crate provides an optional feature of denoising of the audio stream (may result in better transcription). It is **disabled by default**, to enable it: use either the `denoise` or `full` key in the crate's features list.

```toml
ds-transcriber = { version = "1", features = ["denoise"] } # or features = ["full"]
```

# Extras
This crate contains an [example](examples/transcribe.rs) to get you started. 
Clone the repository and run it:

For help with arguments, run:
```sh
cargo run --example transcribe -- -h
```

To start the example, run
```sh
cargo run --example transcribe -- -m model_path -c deepspeech_native_client_dir
```
An optional (but **recommended**) argument for a language model (scorer) path can be provided with `-s` or `--scorer`

# Re-exports

This crate also re-exports the `deepspeech` and `nnnoiseless` crates (if the `denoise` feature is enabled). You can use these re-exports instead of also depending on them separately.

## Transcription Tips
Downloading the DeepSpeech model alone will give you results that are passable, at best, (depending on your accent), if you want to significantly improve them, you might also want to download a [language model/scorer](https://github.com/mozilla/DeepSpeech/releases/tag/v0.9.0). It helps in cases like: `I read a book last night` vs `I red a book last night`. Simply put the scorer in the same directory as your model. The crate will automatically set it when you create your `ds_transcriber::model::DeepSpeechModel`

If you want to train your own model, for the best results, look into [Mimic Recording Studio](https://github.com/MycroftAI/mimic-recording-studio), it gives you prompts to read from and **automatically** prepares your audio files with their respective transcriptions for training which you can then use for [fine tuning](https://deepspeech.readthedocs.io/en/r0.9/TRAINING.html)

## Contributions
Always welcome! Open an issue or a PR if you have something in mind
