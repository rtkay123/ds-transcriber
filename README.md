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

```rust
let mut model = ds_transcriber::model::DeepSpeechModel::new("path_to_ds_model")?;
let ds_model = model.prepared_model();
let mut config = ds_transcriber::StreamSettings {
    //value used for pause detection, a pause is detected when the amplitude is less than this
    silence_level: 200,
    // takes a reference of the model we instantiated earlier
    model: ds_model,
    // show the amplitude values on stdout (helps you to find your silence level)
    show_amplitudes: true,
    // seconds of silence indicating end of speech (begin transcribe when pause_length is grater than....)
    pause_length: 1.0,
};
let i_said = ds_transcriber::transcribe(&mut config)?;
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
cargo run --example transcribe -- -m model_path_dir -c deepspeech_native_client_dir
```

## Transcription Tips
Downloading the DeepSpeech model alone will give you results that are passable, at best, (depending on your accent), if you want to significantly improve them, you might also want to download a [language model/scorer](https://github.com/mozilla/DeepSpeech/releases/tag/v0.9.0). It helps in cases like: `I read a book last night` vs `I red a book last night`. Simply put the scorer in the same directory as your model. The crate will automatically set it when you create your `ds_transcriber::model::DeepSpeechModel`

If you want to train your own model, for the best results, look into [Mimic Recording Studio](https://github.com/MycroftAI/mimic-recording-studio), it gives you prompts to read from and **automatically** prepares your audio files with their respective transcriptions for training which you can then use for [fine tuning](https://deepspeech.readthedocs.io/en/r0.9/TRAINING.html)

## Other works
I'm working on a digital assistant written completely in Rust. It will be taking advantage of `ds-transcriber`. It features offline natural language understanding with Bidirectional Encoder Representations from Transformers (BERT). If it is of interest, you can find it [here](https://github.com/kawaki-san/lyra)

## Contributions
Always welcome! Open an issue or a PR if you have something in mind
