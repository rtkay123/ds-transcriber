//! Library's entry point. Converts the audio stream to text using DeepSpeech bindings

use crate::stream::record_audio;
use deepspeech::Model;

///
/// # Example
///
/// Creates preferences for the configuration of the audio stream and transcription.
/// ```
/// let model_dir_str = args().nth(1).expect("Please specify model dir");
/// let mut ds_model = DeepSpeechModel::instantiate_from(model_dir_str);
/// let model = ds_model.model();
///
/// let mut config = ds_transcriber::transcriber::StreamSettings {
///     //value used for pause detection, a pause is detected when the amplitude is less than this
///     silence_level: 200,
///     // takes a reference of the model we instantiated earlier
///     model,
///     // show the amplitude values on stdout (helps you to find your silence level)
///     show_amplitudes: true,
///     // seconds of silence indicating end of speech (begin transcribe when pause_length is grater than....)
///     pause_length: 2.0,
/// };
/// ```

pub struct StreamSettings<'a> {
    // value used for pause detection, a pause is detected when the amplitude is less than this
    pub silence_level: i32,
    // the reference of the model we instantiated earlier
    pub model: &'a mut Model,
    // show the amplitude values on stdout (helps you to find your silence level)
    pub show_amplitudes: bool,
    // seconds of silence indicating end of speech
    pub pause_length: f32,
}

///
/// # Usage
/// After getting config ready, all you need to do is pass a ref of it to the function:
/// ```
/// let i_said = ds_transcriber::transcriber::transcriber::transcribe(&mut config).unwrap();
/// println!("I said: {}", i_said);
/// ```

pub fn transcribe(config: &mut StreamSettings) -> Option<String> {
    match record_audio(
        config.silence_level,
        config.show_amplitudes,
        config.pause_length,
    ) {
        Some(audio_stream) => Some(convert(&audio_stream, config.model)),
        None => None,
    }
}

fn convert(audio_stream: &[i16], model: &mut Model) -> String {
    model.speech_to_text(audio_stream).unwrap()
}
