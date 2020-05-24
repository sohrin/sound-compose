use cpal::{StreamData, UnknownTypeOutputBuffer};
use cpal::traits::{HostTrait, DeviceTrait, EventLoopTrait};
use std::fs::File;
use std::io::BufReader;
use rodio::Source;
use std::thread;
use std::time::Duration;

pub fn debug_proc() -> Result<(), anyhow::Error> {
    debug!("debug_proc() BEGIN.");

    // TODO: https://docs.rs/rodio/0.11.0/rodio/
    let device = rodio::default_output_device().unwrap();
    let file = File::open("assets/wav/test/2608_bd.wav").unwrap();
    let source = rodio::Decoder::new(BufReader::new(file)).unwrap();

    thread::sleep(Duration::from_secs(1));
    rodio::play_raw(&device, source.convert_samples());

    // TODO: https://docs.rs/cpal/0.11.0/cpal/

    let host = cpal::default_host();
    let device = host
        .default_output_device()
        .expect("no default output device");
    let format = device
        .default_output_format()
        .expect("failed to get device name");
    println!("{:?}", format);

    let event_loop = host.event_loop();
    let stream_id = event_loop
        .build_output_stream(&device, &format)
        .unwrap();
    println!("{:?}", stream_id);

    // match config.sample_format() {
    //     cpal::SampleFormat::F32 => run::<f32>(&device, &config.into())?,
    //     cpal::SampleFormat::I16 => run::<i16>(&device, &config.into())?,
    //     cpal::SampleFormat::U16 => run::<u16>(&device, &config.into())?,
    // }

// TODO: 後で消す
println!("1");
    event_loop.run(move |stream_id, stream_result| {
// TODO: 後で消す
println!("2");
        let stream_data = match stream_result {
            Ok(data) => data,
            Err(err) => {
                eprintln!("an error occurred on stream {:?}: {}", stream_id, err);
                return;
            }
            _ => return,
        };
    
        match stream_data {
            StreamData::Output { buffer: UnknownTypeOutputBuffer::U16(mut buffer) } => {
                for elem in buffer.iter_mut() {
                    *elem = u16::max_value() / 2;
                }
            },
            StreamData::Output { buffer: UnknownTypeOutputBuffer::I16(mut buffer) } => {
                for elem in buffer.iter_mut() {
                    *elem = 0;
                }
            },
            StreamData::Output { buffer: UnknownTypeOutputBuffer::F32(mut buffer) } => {
                for elem in buffer.iter_mut() {
                    *elem = 0.0;
                }
            },
            _ => (),
        }
    });

    Ok(())
}

// fn run<T>(device: &cpal::Device, config: &cpal::StreamConfig) -> Result<(), anyhow::Error>
// where
//     T: cpal::Sample,
// {
//     let sample_rate = config.sample_rate.0 as f32;
//     let channels = config.channels as usize;

//     // Produce a sinusoid of maximum amplitude.
//     let mut sample_clock = 0f32;
//     let mut next_value = move || {
//         sample_clock = (sample_clock + 1.0) % sample_rate;
//         (sample_clock * 440.0 * 2.0 * 3.141592 / sample_rate).sin()
//     };

//     let err_fn = |err| eprintln!("an error occurred on stream: {}", err);

//     let stream = device.build_output_stream(
//         config,
//         move |data: &mut [T], _: &cpal::OutputCallbackInfo| {
//             write_data(data, channels, &mut next_value)
//         },
//         err_fn,
//     )?;
//     stream.play()?;

//     std::thread::sleep(std::time::Duration::from_millis(1000));

//     Ok(())
// }

// fn write_data<T>(output: &mut [T], channels: usize, next_sample: &mut dyn FnMut() -> f32)
// where
//     T: cpal::Sample,
// {
//     for frame in output.chunks_mut(channels) {
//         let value: T = cpal::Sample::from::<f32>(&next_sample());
//         for sample in frame.iter_mut() {
//             *sample = value;
//         }
//     }
// }