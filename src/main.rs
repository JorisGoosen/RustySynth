//! Play a sine wave for several seconds.
//!
//! A rusty adaptation of the official PortAudio C "paex_sine.c" example by Phil Burk and Ross
//! Bencina.

extern crate portaudio;

use portaudio as pa;
use std::f64::consts::PI;

const CHANNELS: i32 = 2;
const NUM_SECONDS: i32 = 5;
const SAMPLE_RATE: f64 = 44_100.0;
const FRAMES_PER_BUFFER: u32 = 64;
const TABLE_SIZE: usize = 200;

fn main() {
    match run() {
        Ok(_) => {}
        e => {
            eprintln!("Example failed with the following: {:?}", e);
        }
    }
}

fn geluid  (index: i64, freq: f64) -> f32
{
    ((index as f64 / TABLE_SIZE as f64 * PI * 2.0 * freq).sin() * 0.2) as f32
}

fn run() -> Result<(), pa::Error> {
    println!(
        "PortAudio Test: output sine wave. SR = {}, BufSize = {}",
        SAMPLE_RATE, FRAMES_PER_BUFFER
    );


    let pa = pa::PortAudio::new()?;

    let mut settings =
        pa.default_output_stream_settings(CHANNELS, SAMPLE_RATE, FRAMES_PER_BUFFER)?;
    // we won't output out of range samples so don't bother clipping them.
    //settings.flags = pa::stream_flags::CLIP_OFF;

    let mut idxCum: i64 = 0;
    let mut freq:   f64 = 0.1;

    // This routine will be called by the PortAudio engine when audio is needed. It may called at
    // interrupt level on some machines so don't do anything that could mess up the system like
    // dynamic resource allocation or IO.
    let callback = move |pa::OutputStreamCallbackArgs { buffer, frames, .. }| {
        let mut idx = 0;
        for _ in 0..frames {
            let geluidsmonster: f32 = geluid(idx as i64 + idxCum, freq);
            buffer[idx    ] = geluidsmonster;
            buffer[idx + 1] = geluidsmonster;
            
            idx += 2;
        }

        idxCum += idx as i64;
        freq *= 1.002;
        
       /* right_shift += 1;
        if right_shift > TABLE_SIZE / 20 
        {
            right_shift = 1;
        }*/

        pa::Continue
    };

    let mut stream = pa.open_non_blocking_stream(settings, callback)?;

    stream.start()?;

    println!("Play for {} seconds.", NUM_SECONDS);
    pa.sleep(NUM_SECONDS * 1_000);

    stream.stop()?;
    stream.close()?;

    println!("Test finished.");

    Ok(())
}
