use std::{env::set_var, path::Path};

fn main() -> Result<(), anyhow::Error> {
    let (model_path, scorer_path) = initialise_app();
    let mut model = ds_transcriber::model::instance_model(
        model_path.as_ref(),
        match scorer_path {
            Some(scorer) => {
                let val = Path::new(&scorer).to_owned();
                Some(val.into_boxed_path())
            }
            None => None,
        },
    )?;
    let i_said = ds_transcriber::transcribe(ds_transcriber::StreamSettings::default(), &mut model)?;
    println!("I said: {}", i_said);
    Ok(())
}

fn initialise_app() -> (impl AsRef<str>, Option<String>) {
    let m = clap::command!()
        .arg(
            clap::Arg::new("model_path")
                .takes_value(true)
                .short('m')
                .long("model")
                .help("Path to your DeepSpeech [.pb/.pbmm] model"),
        )
        .arg(
            clap::Arg::new("scorer_path")
                .takes_value(true)
                .short('s')
                .long("scorer")
                .help("An optional path pointing to your DeepSpeech [.scorer] scorer"),
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
    let scorer_path = m.value_of("scorer_path").map(|val| val.to_owned());
    let native_client = match m.value_of("native_client") {
        Some(val) => val,
        None => panic!("no native client specified"),
    };
    set_var("LD_LIBRARY_PATH", native_client);
    set_var("LIBRARY_PATH", native_client);
    (model_path.to_owned(), scorer_path)
}
