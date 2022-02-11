//! Instantiates a deep speech model to be used for transcription
use std::path::Path;

use deepspeech::Model;
use log::trace;

///
/// # Struct to hold the model
///
#[allow(missing_debug_implementations)]
pub struct DeepSpeechModel {
    /// Stores the model
    pub model: Model,
}

impl DeepSpeechModel {
    ///
    /// Accepts a string with the path of the deep speech model. Tested with DeepSpeech model versions `0.9.x`
    ///
    pub fn new(model_dir_str: impl AsRef<str>) -> Result<Self, anyhow::Error> {
        let dir_path = Path::new(model_dir_str.as_ref());
        let mut graph_name: Box<Path> = dir_path.join("output_graph.pb").into_boxed_path();
        let mut scorer_name: Option<Box<Path>> = None;

        for file in dir_path.read_dir().into_iter().flatten() {
            let file = file?;
            let file_path = file.path();
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
        let mut model = Model::load_from_files(&graph_name)?;
        trace!("model created");

        if let Some(scorer) = scorer_name {
            trace!("using scorer {:?}", scorer.file_name());
            model.enable_external_scorer(&scorer)?;
        }

        Ok(DeepSpeechModel { model })
    }

    /// Returns a mutable reference of the model - Which we pass into `ds_transcriber::transcriber::StreamSettings`
    pub fn prepared_model(&mut self) -> &mut Model {
        &mut self.model
    }
}
