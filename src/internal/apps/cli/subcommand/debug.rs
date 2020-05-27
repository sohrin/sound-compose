// use cpal::{EventLoop, StreamData, UnknownTypeOutputBuffer};
// use cpal::traits::{HostTrait, DeviceTrait, EventLoopTrait};

pub fn debug_proc() {
    debug!("debug_proc() BEGIN.");

    // // MEMO: https://docs.rs/cpal/0.11.0/cpal/
    // let host = cpal::default_host();
    // let device = host.default_output_device().expect("no output device available");
    // let format = device.default_output_format().unwrap();
    // println!("{:?}", format);

    // let event_loop = host.event_loop();
    // let stream_id = event_loop.build_output_stream(&device, &format).unwrap();
    // println!("{:?}", stream_id);
    // event_loop.play_stream(stream_id);
  
    // let mut samples_written: u64 = 0;

    // event_loop.run(move |stream_id, stream_result| {
    //     let stream_data = match stream_result {
    //         Ok(data) => data,
    //         Err(err) => {
    //             eprintln!("an error occurred on stream {:?}: {}", stream_id, err);
    //             return;
    //         }
    //         _ => return,
    //     };

    //     let mut samples_written: u64 = 0;
    
    //     match stream_data {
    //         /* TODO: どういう時に必要になる？
    //          *       デフォルトのformatは「Format { channels: 2, sample_rate: SampleRate(48000), data_type: F32 }」
    //          *      だったのでF32のルートしか動かなかった。
    //          */
    //         // StreamData::Output { buffer: UnknownTypeOutputBuffer::U16(mut buffer) } => {
    //         //     println!("1");
    //         //     for elem in buffer.iter_mut() {
    //         //         *elem = u16::max_value() / 2;
    //         //     }
    //         // },
    //         // StreamData::Output { buffer: UnknownTypeOutputBuffer::I16(mut buffer) } => {
    //         //     println!("2");
    //         //     for elem in buffer.iter_mut() {
    //         //         *elem = 0;
    //         //     }
    //         // },
    //         StreamData::Output { buffer: UnknownTypeOutputBuffer::F32(mut buffer) } => {
    //             for elem in buffer.iter_mut() {
    //                 // MEMO: https://gist.github.com/ablwr/c02ccd4f6c48bd90614647ec9dbd3380
    //                 let time = (samples_written / format.channels as u64) as f32 
    //                          / format.sample_rate.0 as f32;
    //                 let t = time / (1.0/220.0) % 1.0;

    //                 if t < 0.5 {
    //                     *elem = 0.4;
    //                 } else {
    //                     *elem = -0.4; 
    //                 }      

    //                 samples_written+=1;
    //             }
    //         },
    //         _ => (),
    //     }
    // });
    
}