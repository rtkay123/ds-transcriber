//! Library's entry point. Converts the audio stream to text using DeepSpeech bindings

use crate::stream::record_audio;
use deepspeech::Model;
use std::fmt::Debug;

///
/// # Example
///
/// Creates preferences for the configuration of the audio stream and transcription.
/// ```
/// use std::env::args;
/// use ds_transcriber::model::DeepSpeechModel;
/// # fn main()->Result<(),Box<dyn std::error::Error>> {
///     if let Some(model_dir_str) = args().nth(1) {
///         let mut model = DeepSpeechModel::new(model_dir_str, None)?;
///         let mut config = ds_transcriber::StreamSettings::default();
///     }
///    # Ok(())
/// # }
/// ```
#[derive(Debug, Clone, Copy)]
pub struct StreamSettings {
    /// value used for pause detection, a pause is detected when the amplitude is less than this
    pub silence_level: i32,
    /// show the amplitude values on stdout (helps you to find your silence level)
    pub show_amplitudes: bool,
    /// seconds of silence indicating end of speech
    pub pause_length_millis: u32,
}

impl StreamSettings {
    /// Create a new configuration for a mic stream
    pub fn new(silence_level: i32, show_amplitudes: bool, pause_length_millis: u32) -> Self {
        Self {
            silence_level,
            show_amplitudes,
            pause_length_millis,
        }
    }
}

impl Default for StreamSettings {
    fn default() -> Self {
        Self {
            silence_level: 200,
            show_amplitudes: true,
            pause_length_millis: 1000,
        }
    }
}

///
/// # Usage
/// After getting config ready, all you need to do is pass a ref of it to the function:
/// ```
/// use std::env::args;
/// # use ds_transcriber::model::DeepSpeechModel;
/// # fn main()-> Result<(),Box<dyn std::error::Error>> {
///    # if let Some(model_dir_str) = args().nth(1) {
///    #    let mut model = DeepSpeechModel::new(model_dir_str, None)?;
///    #    let config = ds_transcriber::StreamSettings::default();
///         let i_said = ds_transcriber::transcribe(config,&mut model)?;
///         println!("I said: {}", i_said);
///    #  }
///    # Ok(())
/// # }
/// ```

pub fn transcribe(config: StreamSettings, model: &mut Model) -> Result<String, anyhow::Error> {
    let pause_length_secs: f32 = (config.pause_length_millis / 1000) as f32;
    match record_audio(
        config.silence_level,
        config.show_amplitudes,
        pause_length_secs,
    ) {
        Ok(audio_stream) => convert(&audio_stream, model),
        Err(e) => Err(anyhow::anyhow!(e)),
    }
}

fn convert(audio_stream: &[i16], model: &mut Model) -> Result<String, anyhow::Error> {
    let buf = model.speech_to_text(audio_stream)?;
    Ok(buf)
}
