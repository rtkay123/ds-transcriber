//! Does the actual recording and white noise cleaning

use std::{
    ops::Neg,
    sync::mpsc::{channel, Receiver},
    time::Instant,
};

use cpal::traits::{DeviceTrait, StreamTrait};
use log::error;
#[cfg(feature = "denoise")]
use nnnoiseless::DenoiseState;

use crate::config::StreamConfig;

///
/// # Recording Audio
/// This function handles the audio recording process, parameters are obtained from `ds_transcriber::transcriber::StreamSettings`
///

pub fn record_audio(
    silence_level: i32,
    show_amplitude: bool,
    pause_length: f32,
) -> Result<Vec<i16>, anyhow::Error> {
    let config = StreamConfig::new(silence_level)?;
    let (sound_sender, sound_receiver) = channel();
    let device = config.device();
    let stream_config = config.supported_config().config();
    let stream = device.build_input_stream(
        &stream_config,
        move |data: &[f32], _: &_| {
            if let Err(e) = sound_sender.send(data.to_owned()) {
                error!("{}", e);
            }
        },
        move |err| error!("Stream read error: {}", err),
    )?;

    match stream.play() {
        Ok(()) => {
            let denoised_stream = start(
                &sound_receiver,
                config.silence_level(),
                show_amplitude,
                pause_length,
            )?;
            let mut audio_buf: Vec<_> = Vec::new();
            //convert to i16 stream
            for s in denoised_stream {
                audio_buf.push((s * i16::max_value() as f32) as i16);
            }
            Ok(audio_buf)
        }
        Err(err) => {
            error!("Failed to start the stream: {}", err);
            Err(anyhow::anyhow!(err))
        }
    }
}

#[cfg(feature = "denoise")]
fn start(
    sound_receiver: &Receiver<Vec<f32>>,
    silence_level: i32,
    show_amplitude: bool,
    pause_length: f32,
) -> Result<Vec<f32>, anyhow::Error> {
    let mut silence_start = None;
    let mut sound_from_start_till_pause = vec![];
    loop {
        let small_sound_chunk = sound_receiver.recv()?;
        sound_from_start_till_pause.extend(&small_sound_chunk);
        let sound_as_ints = small_sound_chunk.iter().map(|f| (*f * 1000.0) as i32);
        let max_amplitude = sound_as_ints.clone().max().unwrap_or(0);
        let min_amplitude = sound_as_ints.clone().min().unwrap_or(0);
        if show_amplitude {
            println!("Min is {}, Max is {}", min_amplitude, max_amplitude);
        }
        let silence_detected = max_amplitude < silence_level && min_amplitude > silence_level.neg();
        if silence_detected {
            match silence_start {
                None => silence_start = Some(Instant::now()),
                Some(s) => {
                    if s.elapsed().as_secs_f32() > pause_length {
                        return Ok(denoise(sound_from_start_till_pause));
                    }
                }
            }
        } else {
            silence_start = None;
        }
    }
}

#[cfg(not(feature = "denoise"))]
fn start(
    sound_receiver: &Receiver<Vec<f32>>,
    silence_level: i32,
    show_amplitude: bool,
    pause_length: f32,
) -> Result<Vec<f32>, anyhow::Error> {
    let mut silence_start = None;
    let mut sound_from_start_till_pause = vec![];
    loop {
        let small_sound_chunk = sound_receiver.recv()?;
        sound_from_start_till_pause.extend(&small_sound_chunk);
        let sound_as_ints = small_sound_chunk.iter().map(|f| (*f * 1000.0) as i32);
        let max_amplitude = sound_as_ints.clone().max().unwrap_or(0);
        let min_amplitude = sound_as_ints.clone().min().unwrap_or(0);
        if show_amplitude {
            println!("Min is {}, Max is {}", min_amplitude, max_amplitude);
        }
        let silence_detected = max_amplitude < silence_level && min_amplitude > silence_level.neg();
        if silence_detected {
            match silence_start {
                None => silence_start = Some(Instant::now()),
                Some(s) => {
                    if s.elapsed().as_secs_f32() > pause_length {
                        return Ok(sound_from_start_till_pause);
                    }
                }
            }
        } else {
            silence_start = None;
        }
    }
}

#[cfg(feature = "denoise")]
fn denoise(sound_from_start_till_pause: Vec<f32>) -> Vec<f32> {
    let mut output = Vec::new();
    let mut out_buf = [0.0; DenoiseState::FRAME_SIZE];
    let mut denoise = DenoiseState::new();
    let mut first = true;
    for chunk in sound_from_start_till_pause.chunks_exact(DenoiseState::FRAME_SIZE) {
        denoise.process_frame(&mut out_buf[..], chunk);

        if !first {
            output.extend_from_slice(&out_buf[..]);
        }
        first = false;
    }
    output
}
