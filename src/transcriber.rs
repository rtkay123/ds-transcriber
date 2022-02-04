//! Library's entry point. Converts the audio stream to text using DeepSpeech bindings

use std::fmt::Debug;

use crate::stream::record_audio;
use deepspeech::Model;

///
/// # Example
///
/// Creates preferences for the configuration of the audio stream and transcription.
/// ```
/// use std::env::args;
/// use ds_transcriber::model::DeepSpeechModel;
/// # fn main()->Result<(),Box<dyn std::error::Error>> {
///     if let Some(model_dir_str) = args().nth(1) {
///         let mut ds_model = DeepSpeechModel::new(model_dir_str)?;
///         let model = ds_model.prepared_model();
///         let mut config = ds_transcriber::StreamSettings {
///             //value used for pause detection, a pause is detected when the amplitude is less than this
///             silence_level: 200,
///             // takes a reference of the model we instantiated earlier
///             model,
///             // show the amplitude values on stdout (helps you to find your silence level)
///             show_amplitudes: true,
///             // seconds of silence indicating end of speech (begin transcribe when pause_length is grater than....)
///             pause_length: 2.0,
///         };
///     }
///    # Ok(())
/// # }
/// ```
pub struct StreamSettings<'a> {
    /// value used for pause detection, a pause is detected when the amplitude is less than this
    pub silence_level: i32,
    /// the reference of the model we instantiated earlier
    pub model: &'a mut Model,
    /// show the amplitude values on stdout (helps you to find your silence level)
    pub show_amplitudes: bool,
    /// seconds of silence indicating end of speech
    pub pause_length: f32,
}

impl Debug for StreamSettings<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("StreamSettings")
            .field("silence_level", &self.silence_level)
            .field("amplitudes_in_stdout", &self.show_amplitudes)
            .field("pause_length", &self.pause_length)
            .finish()
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
///    #    let mut ds_model = DeepSpeechModel::new(model_dir_str)?;
///    #    let model = ds_model.prepared_model();
///    #    let mut config = ds_transcriber::StreamSettings {
///    #     //value used for pause detection, a pause is detected when the amplitude is less than this
///    #     silence_level: 200,
///    #     // takes a reference of the model we instantiated earlier
///    #     model,
///    #     // show the amplitude values on stdout (helps you to find your silence level)
///    #     show_amplitudes: true,
///    #     // seconds of silence indicating end of speech (begin transcribe when pause_length is grater than....)
///    #     pause_length: 2.0,
///    #    };
///         let i_said = ds_transcriber::transcribe(&mut config).unwrap();
///         println!("I said: {}", i_said);
///    #  }
///    # Ok(())
/// # }
/// ```

pub fn transcribe(config: &mut StreamSettings) -> Result<String, anyhow::Error> {
    match record_audio(
        config.silence_level,
        config.show_amplitudes,
        config.pause_length,
    ) {
        Ok(audio_stream) => convert(&audio_stream, config.model),
        Err(e) => Err(anyhow::anyhow!(e)),
    }
}

fn convert(audio_stream: &[i16], model: &mut Model) -> Result<String, anyhow::Error> {
    let buf = model.speech_to_text(audio_stream)?;
    Ok(buf)
}
