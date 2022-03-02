//! Instantiates a deep speech model to be used for transcription
use std::path::Path;

use deepspeech::Model;
use log::trace;

///
/// # Struct to hold the model
///
#[allow(missing_debug_implementations)]
pub struct DeepSpeechModel;

impl DeepSpeechModel {
    ///
    /// Create model and scorer from paths
    ///
    #[allow(clippy::new_ret_no_self)]
    pub fn new(model: impl AsRef<str>, scorer: Option<Box<Path>>) -> Result<Model, anyhow::Error> {
        let graph_name = Path::new(model.as_ref());
        let mut model = Model::load_from_files(graph_name)?;
        trace!("model created");
        if let Some(scorer) = scorer {
            trace!("using scorer {:?}", scorer.file_name());
            model.enable_external_scorer(&scorer)?;
        }
        Ok(model)
    }
}
