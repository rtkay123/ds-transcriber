//! Instantiates a deep speech model to be used for transcription
use std::path::Path;

use deepspeech::Model;

///
/// # Object to hold the model
///
pub struct DeepSpeechModel {
    pub model: Model,
}

impl DeepSpeechModel {
    ///
    /// Accepts a string with the path of the deep speech model. Tested with DeepSpeech model versions `0.9.x`
    ///
    pub fn instantiate_from(model_dir_str: String) -> Self {
        let dir_path = Path::new(&model_dir_str);
        let mut graph_name: Box<Path> = dir_path.join("output_graph.pb").into_boxed_path();
        let mut scorer_name: Option<Box<Path>> = None;

        for file in dir_path.read_dir().into_iter().flatten() {
            let file_path = file.unwrap().path();
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
        let mut model = Model::load_from_files(&graph_name).unwrap();

        if let Some(scorer) = scorer_name {
            model.enable_external_scorer(&scorer).unwrap();
        }

        DeepSpeechModel { model }
    }

    /// Returns a mutable reference of the model - Which we pass into `ds_transcriber::transcriber::StreamSettings`
    pub fn model(&mut self) -> &mut Model {
        &mut self.model
    }
}
