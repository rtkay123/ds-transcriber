//! Instantiates a deep speech model to be used for transcription
use std::path::Path;

use deepspeech::Model;
use log::trace;

/// Create a new instance of a model
pub fn instance_model(model: &str, scorer: Option<Box<Path>>) -> anyhow::Result<Model> {
    let graph_name = Path::new(model);
    let mut model = Model::load_from_files(graph_name)?;
    trace!("model created");
    if let Some(scorer) = scorer {
        trace!("using scorer {:?}", scorer.file_name());
        model.enable_external_scorer(&scorer)?;
    }
    Ok(model)
}
