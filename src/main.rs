use std::io::{stdout, Write};
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Arc;
use std::thread::sleep;
use std::time::Duration;

use clap::{Parser, Subcommand};
use color_eyre::eyre::{self, ContextCompat};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::StreamConfig;
use pitch_detection::detector::{mcleod::McLeodDetector, PitchDetector};

use ui::tuner::TunerUi;
use ui::Ui;

use crate::ui::pitch_test::PitchTestUi;

mod notes;
mod pitch;
mod ui;

fn list_inputs() -> eyre::Result<()> {
    let host = cpal::default_host();
    let devices = host.devices()?;

    println!("availiable audio inputs");
    for device in devices {
        println!("- {}", device.name()?);
    }

    Ok(())
}

const SAMPLE_RATE: usize = 44100;
const SIZE: usize = 4096;
const PADDING: usize = SIZE / 2;
const POWER_THRESHOLD: f32 = 1.0;
const CLARITY_THRESHOLD: f32 = 0.7;

fn run(device: Option<String>, ui: &mut dyn Ui) -> eyre::Result<()> {
    let host = cpal::default_host();
    let input = match device {
        Some(name) => host
            .devices()?
            .try_fold::<_, _, eyre::Result<_>>(None, |o, d| {
                if o.is_some() {
                    return Ok(o);
                }
                if d.name()? == name {
                    Ok(Some(d))
                } else {
                    Ok(None)
                }
            })?
            .with_context(|| format!("no such input: {name}"))?,
        None => host
            .default_input_device()
            .context("couldn't find the default input")?,
    };
    let config = StreamConfig {
        channels: 1,
        sample_rate: cpal::SampleRate(44_100),
        buffer_size: cpal::BufferSize::Default,
    };
    let mut buffer = Vec::new();
    let freq = Arc::new(AtomicU32::new(0));
    let freq_clone = freq.clone();
    let stream = input.build_input_stream::<f32, _, _>(
        &config,
        move |d, _| {
            if buffer.len() < SIZE {
                buffer.extend_from_slice(d);
            } else {
                let mut detector = McLeodDetector::new(SIZE, PADDING);
                if let Some(pitch) = detector.get_pitch(
                    &buffer[0..SIZE],
                    SAMPLE_RATE,
                    POWER_THRESHOLD,
                    CLARITY_THRESHOLD,
                ) {
                    freq_clone.store(pitch.frequency.to_bits(), Ordering::Relaxed);
                }
                buffer.clear();
            }
        },
        |_e| (),
        None,
    )?;
    stream.play()?;

    loop {
        let freq = f32::from_bits(freq.load(Ordering::Relaxed));
        ui.update(freq);
        print!("{ui}");
        stdout().flush()?;
        sleep(Duration::from_millis(1000 / 60));
    }
}

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    mode: Option<Mode>,
    /// specify a specific input device to use. Default to the host's default
    #[clap(long, short)]
    input: Option<String>,
}

#[derive(Subcommand)]
enum Mode {
    /// list all input devices
    ListInputs,
    /// A game where you must play the requested note
    PitchTest,
}

fn main() -> eyre::Result<()> {
    let args = Cli::parse();
    color_eyre::install()?;

    if let Some(mode) = args.mode {
        match mode {
            Mode::ListInputs => list_inputs()?,
            Mode::PitchTest => {
                let mut ui = PitchTestUi::new();
                println!("Play the requested note:");
                run(args.input, &mut ui)?;
            }
        }
    } else {
        let mut ui = TunerUi(0.0);
        run(args.input, &mut ui)?;
    }

    Ok(())
}
