//! Instantiates a deep speech model to be used for transcription
use std::path::Path;

use deepspeech::Model;
use log::trace;

/// Create a new instance of a model
pub fn instance_model<A: AsRef<Path>>(model: A, scorer: Option<A>) -> anyhow::Result<Model> {
    let graph_name = model.as_ref();
    let mut model = Model::load_from_files(graph_name)?;
    trace!("model created");
    if let Some(scorer) = scorer {
        trace!("using scorer {:?}", scorer.as_ref());
        model.enable_external_scorer(scorer.as_ref())?;
    }
    Ok(model)
}
