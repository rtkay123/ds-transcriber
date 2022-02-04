use std::env::set_var;

fn main() -> Result<(), anyhow::Error> {
    let model_path = initialise_app();
    let mut model = ds_transcriber::model::DeepSpeechModel::new(model_path.as_ref())?;
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
    Ok(())
}

fn initialise_app() -> impl AsRef<str> {
    let m = clap::app_from_crate!()
        .arg(
            clap::Arg::new("model_path")
                .takes_value(true)
                .short('m')
                .long("model")
                .help("Path to your DeepSpeech [.pb/.pbmm] model"),
        )
        .arg(
            clap::Arg::new("native_client")
                .takes_value(true)
                .short('c')
                .long("client")
                .help("Path to the DeepSpeech [native_client]"),
        )
        .get_matches();
    let model_path = match m.value_of("model_path") {
        Some(val) => val,
        None => panic!("no model specified"),
    };
    let native_client = match m.value_of("native_client") {
        Some(val) => val,
        None => panic!("no native client specified"),
    };
    set_var("LD_LIBRARY_PATH", native_client);
    set_var("LIBRARY_PATH", native_client);
    model_path.to_owned()
}
