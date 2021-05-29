//! Library's entry point. Converts the audio stream to text using DeepSpeech bindings

use crate::stream::record_audio;
use deepspeech::Model;
use std::path::Path;

///
/// # Example
///
/// Creates preferences for the configuration of the audio stream and transcription.
/// ```
/// let model_path = args().nth(1).expect("Please specify model dir");
/// let config = ds_transcriber::transcriber::StreamSettings {
///    //value used for pause detection, a pause is detected when the amplitude is less than this
///    silence_level: 200,
///    // the directory of the deep speech model
///    model_dir_str: model_path,
///    // show the amplitude values on stdout (helps you to find your silence level)
///    show_amplitudes: true,
///    // seconds of silence indicating end of speech
///    pause_length: 2.0,
/// };
/// ```

pub struct StreamSettings {
    // value used for pause detection, a pause is detected when the amplitude is less than this
    pub silence_level: i32,
    // the path of the deep speech model
    pub model_dir_str: String,
    // show the amplitude values on stdout (helps you to find your silence level)
    pub show_amplitudes: bool,
    // seconds of silence indicating end of speech
    pub pause_length: f32,
}

///
/// # Usage
/// After getting config ready, all you need to do is pass it to the function.:
/// ```
/// let i_said = ds_transcriber::transcriber::transcriber::transcribe(config);
/// println!("I said: {}", i_said);
/// ```

pub fn transcribe(config: StreamSettings) -> Option<String> {
    match record_audio(
        config.silence_level,
        config.show_amplitudes,
        config.pause_length,
    ) {
        Some(audio_stream) => Some(convert(&audio_stream, &config.model_dir_str)),
        None => None,
    }
}

fn convert(audio_stream: &Vec<i16>, model_dir_str: &String) -> String {
    let dir_path = Path::new(&model_dir_str);
    let mut graph_name: Box<Path> = dir_path.join("output_graph.pb").into_boxed_path();
    let mut scorer_name: Option<Box<Path>> = None;

    for file in dir_path
        .read_dir()
        .expect("Specified model dir is not a dir")
    {
        if let Ok(f) = file {
            let file_path = f.path();
            if file_path.is_file() {
                if let Some(ext) = file_path.extension() {
                    if ext == "pb" || ext == "pbmm" || ext == "tflite" {
                        graph_name = file_path.into_boxed_path();
                    } else if ext == "scorer" {
                        scorer_name = Some(file_path.into_boxed_path());
                    }
                }
            }
        }
    }
    let mut m = Model::load_from_files(&graph_name).unwrap();

    if let Some(scorer) = scorer_name {
        m.enable_external_scorer(&scorer).unwrap();
    }

    let result = m.speech_to_text(audio_stream).unwrap();
    result
}
