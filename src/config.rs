//! Initialises recording device and audio stream's silence level
use anyhow::{anyhow, Result};
use cpal::{
    traits::{DeviceTrait, HostTrait},
    Device, SampleRate, SupportedStreamConfig,
};
use log::error;

const SAMPLE_RATE: u32 = 16000;

///
/// # Input device configuration
/// Gets data ready to begin recording

pub(crate) struct StreamConfig {
    device: Device,
    config: SupportedStreamConfig,
    silence_level: i32,
}

#[cfg(test)]
mod tests {
    use crate::config::get_config;
    use cpal::traits::HostTrait;

    #[test]
    fn supported_device() {
        let host = cpal::default_host();
        let device = host.default_input_device().expect("no input device found");
        assert_eq!(1, get_config(&device).unwrap().channels());
    }
}

impl StreamConfig {
    /// Creates a new stream configuration:
    pub fn new(silence_level: i32) -> Result<Self> {
        let device = get_default_device()?;
        match get_config(&device) {
            Ok(config) => Ok(StreamConfig {
                config,
                device,
                silence_level,
            }),
            Err(e) => {
                error!("{}", e);
                Err(anyhow!(e))
            }
        }
    }

    /// Returns the device in use:
    pub fn device(&self) -> &Device {
        &self.device
    }

    /// Returns SupportedStreamConfig (.config() returns the configuration we use on our stream)
    pub fn supported_config(&self) -> &SupportedStreamConfig {
        &self.config
    }

    /// Returns the silence level
    pub fn silence_level(&self) -> i32 {
        self.silence_level
    }
}

///Returns the configuration of the mono channel
fn get_config(device: &Device) -> Result<SupportedStreamConfig> {
    let mut config = device.default_input_config()?;
    while config.channels() != 1 {
        let mut supported_configs_range = device.supported_input_configs()?;
        config = match supported_configs_range.next() {
            Some(conf) => conf.with_sample_rate(SampleRate(SAMPLE_RATE)), //16K from deepspeech
            None => break,
        };
    }
    Ok(config)
}
fn get_default_device() -> Result<Device> {
    let host = cpal::default_host();
    match host.default_input_device() {
        Some(device) => Ok(device),
        None => Err(anyhow!("no input device found")),
    }
}
