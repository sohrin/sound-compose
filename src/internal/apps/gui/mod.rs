use cpp_core::{Ptr, Ref, StaticUpcast};
use qt_core::{qs, slot, ContextMenuPolicy, QBox, QObject, QPoint, SlotNoArgs, SlotOfBool};
use qt_widgets::{
    QAction, QApplication, QLineEdit, QMenu, QMessageBox, QPushButton, QTableWidget,
    QTableWidgetItem, QVBoxLayout, QWidget, SlotOfQPoint, SlotOfQTableWidgetItemQTableWidgetItem,
};
use std::rc::Rc;
use std::fmt::Debug;
use std::fs::File;
use std::io::BufReader;
use rodio::Source;
use std::thread;
use std::time::Duration;
use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;

use time_calc::bars::Bars;
use time_calc::calc::SampleHz;
use time_calc::calc::Bpm;
use time_calc::time_sig::TimeSig;

use cpal::traits::{HostTrait, DeviceTrait, StreamTrait};

use hound;

struct Form {
    widget: QBox<QWidget>,
    line_edit: QBox<QLineEdit>,
    table: QBox<QTableWidget>,
    rodio_wav_button: QBox<QPushButton>,
    wave_button: QBox<QPushButton>,
    sequence_button: QBox<QPushButton>,
}

impl StaticUpcast<QObject> for Form {
    unsafe fn static_upcast(ptr: Ptr<Self>) -> Ptr<QObject> {
        ptr.widget.as_ptr().static_upcast()
    }
}

impl Form {
    fn new() -> Rc<Form> {
        unsafe {
            let widget = QWidget::new_0a();
            let layout = QVBoxLayout::new_1a(&widget);

            let line_edit = QLineEdit::new();
            layout.add_widget(&line_edit);

            let rodio_wav_button = QPushButton::from_q_string(&qs("(rodio) WAV file Play"));
            rodio_wav_button.set_enabled(false);
            layout.add_widget(&rodio_wav_button);

            // MEMO: https://doc.qt.io/qt-5/qpushbutton.html
            let wave_button = QPushButton::from_q_string(&qs("(cpal) Wave Data Start"));
            // MEMO: https://doc.qt.io/qt-5/qabstractbutton.html#checkable-prop
            //       http://yu00.hatenablog.com/entry/2015/09/15/185014
            wave_button.set_checkable(true);
            layout.add_widget(&wave_button);

            let sequence_button = QPushButton::from_q_string(&qs("(cpal) WAV file sequence Play"));
            sequence_button.set_enabled(true);
            layout.add_widget(&sequence_button);

            let table = QTableWidget::new_0a();
            table.set_context_menu_policy(ContextMenuPolicy::CustomContextMenu);
            table.set_row_count(2);
            table.set_column_count(1);

            let item1 = QTableWidgetItem::new().into_ptr();
            item1.set_text(&qs("Item 1"));
            table.set_item(0, 0, item1);

            let item2 = QTableWidgetItem::new().into_ptr();
            item2.set_text(&qs("Item 2"));
            table.set_item(1, 0, item2);

            layout.insert_widget_2a(0, &table);

            widget.show();

            let this = Rc::new(Self {
                widget,
                line_edit,
                table,
                wave_button,
                rodio_wav_button,
                sequence_button,
            });
            this.init();
            this
        }
    }

    unsafe fn init(self: &Rc<Self>) {
        self.line_edit
            .text_edited()
            .connect(&self.slot_on_line_edit_text_edited());
        self.table
            .current_item_changed()
            .connect(&self.slot_on_table_current_item_changed());
        self.table
            .custom_context_menu_requested()
            .connect(&self.slot_on_table_context_menu_requested());
        self.rodio_wav_button
            .clicked()
            .connect(&self.slot_on_rodio_wav_button_clicked());
        self.wave_button
            .toggled()
            .connect(&self.slot_on_wave_button_clicked());
        self.sequence_button
            .clicked()
            .connect(&self.slot_on_sequence_button_clicked());
    }

    #[slot(SlotNoArgs)]
    unsafe fn on_line_edit_text_edited(self: &Rc<Self>) {
        self.rodio_wav_button.set_enabled(!self.line_edit.text().is_empty());
    }

    #[slot(SlotNoArgs)]
    unsafe fn on_rodio_wav_button_clicked(self: &Rc<Self>) {
        debug!("on_rodio_wav_button_clicked() BEGIN.");

        /* TODO: スレッド
                 thread::spawnでクロージャーを定義してスレッド開始
                 引数をjoin()することで待機可能。
                 https://doc.rust-jp.rs/book/second-edition/ch16-01-threads.html
                 https://totem3.hatenablog.jp/entry/2017/05/10/210000
        */
        // 別スレッドを起動しwavファイルを再生
        // TODO: 別スレッドでないと「スレッド モードを設定してから変更することはできません。」エラーが発生する件についてまとめる。
        let handle = thread::spawn(|| {
            let wav_file_path = "assets/wav/test/2608_bd.wav";
            let device = rodio::default_output_device().unwrap();
            let file = File::open(wav_file_path).unwrap();
            let source = rodio::Decoder::new(BufReader::new(file)).unwrap();
            rodio::play_raw(&device, source.convert_samples());
            println!("{} played!", wav_file_path);
        });

        // 別スレッドの完了を待機
        handle.join().unwrap();

        debug!("on_rodio_wav_button_clicked() END.");
    }

    #[slot(SlotOfBool)]
    unsafe fn on_wave_button_clicked(self: &Rc<Self>, checked_before_click: bool) {
        debug!("on_wave_button_clicked() BEGIN.");

        /* MEMO: チャンネル
         *       別スレッドとやり取りするためのもの。
         *       https://doc.rust-jp.rs/book/second-edition/ch16-02-message-passing.html
         *       https://doc.rust-lang.org/stable/rust-by-example/std_misc/channels.html
         */
        let (tx, rx): (Sender<&str>, Receiver<&str>) = mpsc::channel();

        if checked_before_click {
            // ボタンON
            self.wave_button.set_text(&qs("Wave Data Stop"));

            fn run<T:Debug>(device: &cpal::Device, config: &cpal::StreamConfig) -> Result<(), anyhow::Error>        // TODO: 理解のために<T:Debug>にしているので、後で<T>に戻したほうがよさそう。
            where
                T: cpal::Sample,
            {
                let sample_rate = config.sample_rate.0 as f32;
                let channels = config.channels as usize;
        
                // Produce a sinusoid of maximum amplitude.
                let mut sample_clock = 0f32;
                let mut next_value = move || {
                    sample_clock = (sample_clock + 1.0) % sample_rate;
                    (sample_clock * 440.0 * 2.0 * 3.141592 / sample_rate).sin()
                };
        
                let err_fn = |err| eprintln!("an error occurred on stream: {}", err);
        
                let stream = device.build_output_stream(
                    config,
                    move |data: &mut [T], _: &cpal::OutputCallbackInfo| {
                        write_data(data, channels, &mut next_value)
                    },
                    err_fn,
                )?;
                stream.play()?;
        
                debug!("Y:before sleep...");
                std::thread::sleep(std::time::Duration::from_millis(2000));
                debug!("Y:before sleep!!!");

                stream.pause()?;

                debug!("Z:before sleep...");
                std::thread::sleep(std::time::Duration::from_millis(2000));
                debug!("Z:before sleep!!!");

                stream.play()?;

                debug!("W:before sleep...");
                std::thread::sleep(std::time::Duration::from_millis(2000));
                debug!("W:before sleep!!!");
        
                Ok(())
            }

            /* MEMO: Rustは関数内に関数を定義可能（JavaScriptに近い）。
             *       前後を探してくれる（run内でwrite_data関数を呼び出しているが、関数定義が後にあってもエラーとならない）
             *       https://stackoverflow.com/questions/26685666/a-local-function-in-rust
             */
            /* TODO: 型パラメータに特性を与えることも可能な件について。
             *       https://www.finddevguides.com/s/rust/rust_generic_types
             */
            fn write_data<T:Debug>(output: &mut [T], channels: usize, next_sample: &mut dyn FnMut() -> f32)
            where
                T: cpal::Sample,
            {
                // TODO: 標準出力を後で消す
                // MEMO: channelsは2だった。LとR？
                println!("channels:{:?}", channels);
                println!("output_elem:{:?}", output);
                // MEMO: https://webbibouroku.com/Blog/Article/rust-iter-index
                // for (i, val) in (*output).iter().enumerate() {
                //     println!("output_elem:{:?}", val);
                // }

                /* TODO: chunks_mutメソッドとは？最終的に以下を返すようだ。&'a mut [T]　と　&mut dyn FnMut() -> f32　の関係は？'aは「ライフライムa」らしいが・・・ライフタイムとは？
                 *       pub struct ChunksMut<'a, T: 'a> {
                 *           v: &'a mut [T],
                 *           chunk_size: usize,
                 *       }
                 *       https://doc.rust-jp.rs/the-rust-programming-language-ja/1.6/book/lifetimes.html
                 *       https://qiita.com/mosh/items/709effc9e451b9b8a5f4
                 */
                for frame in output.chunks_mut(channels) {
                    let value: T = cpal::Sample::from::<f32>(&next_sample());
                    for sample in frame.iter_mut() {
                        *sample = value;
                    }
                }
            }

            // 別スレッドを起動し音声を再生
            /* MEMO: moveキーワード
             *       メインスレッドの値を利用できるようにする。
             *       今回はチャンネル共有のために利用。
             */
            let wave_button_thread_handle = thread::spawn(move || {
                // TODO: 古いライブラリでevent_loop使用時は音が鳴り続けていたが、以下サンプルで一定秒数しか音が鳴らない理由を理解する。
                //       https://github.com/RustAudio/cpal/blob/master/examples/beep.rs
                //       ※ドキュメント（https://docs.rs/cpal/0.11.0/cpal/ など）は動かないコードがあったため、examplesを参考にした。
                let host = cpal::default_host();
                let device = host.default_output_device().expect("no output device available");
                let config = device.default_output_config().expect("no output config available");

                match config.sample_format() {
                    cpal::SampleFormat::F32 => run::<f32>(&device, &config.into()).unwrap(),
                    cpal::SampleFormat::I16 => run::<i16>(&device, &config.into()).unwrap(),
                    cpal::SampleFormat::U16 => run::<u16>(&device, &config.into()).unwrap(),
                }

                // let event_loop = host.event_loop();
                // let stream_id = event_loop.build_output_stream(&device, &format).unwrap();
                // println!("{:?}", stream_id);
            
                // let mut samples_written: u64 = 0;

                // debug!("before recv...");
                // println!("{}", rx.recv().unwrap());
                // debug!("after  recv!!!");

                // // TODO: これがないと音がならない。
                // //       playした後、どうやってpause_streamやdestroy_streamを実行する？
                // //       https://999eagle.moe/posts/rust-video-player-part-4/
                // event_loop.play_stream(stream_id);
                // thread::spawn(move || {
                //     event_loop.run(move |stream_id, stream_result| {
                //         // match rx.try_recv() {
                //         //     Ok(send_data) => {
                //         //         debug!("{:?}", send_data);
                //         //         event_loop.destroy_stream(stream_id);
                //         //     },
                //         //     Err(err) => {
                //         //         debug!("{:?}", err);
                //         //     }
                //         // }
                        
                //         let stream_data = match stream_result {
                //             Ok(data) => data,
                //             Err(err) => {
                //                 eprintln!("an error occurred on stream {:?}: {}", stream_id, err);
                //                 return;
                //             }
                //             _ => return,
                //         };

                //         let mut samples_written: u64 = 0;
                    
                //         match stream_data {
                //             /* TODO: どういう時に必要になる？
                //             *       デフォルトのformatは「Format { channels: 2, sample_rate: SampleRate(48000), data_type: F32 }」
                //             *      だったのでF32のルートしか動かなかった。
                //             */
                //             // StreamData::Output { buffer: UnknownTypeOutputBuffer::U16(mut buffer) } => {
                //             //     println!("1");
                //             //     for elem in buffer.iter_mut() {
                //             //         *elem = u16::max_value() / 2;
                //             //     }
                //             // },
                //             // StreamData::Output { buffer: UnknownTypeOutputBuffer::I16(mut buffer) } => {
                //             //     println!("2");
                //             //     for elem in buffer.iter_mut() {
                //             //         *elem = 0;
                //             //     }
                //             // },
                //             StreamData::Output { buffer: UnknownTypeOutputBuffer::F32(mut buffer) } => {
                //                 for elem in buffer.iter_mut() {
                //                     // MEMO: https://gist.github.com/ablwr/c02ccd4f6c48bd90614647ec9dbd3380
                //                     let time = (samples_written / format.channels as u64) as f32 
                //                             / format.sample_rate.0 as f32;
                //                     let t = time / (1.0/220.0) % 1.0;

                //                     if t < 0.5 {
                //                         *elem = 0.4;
                //                     } else {
                //                         *elem = -0.4; 
                //                     }      

                //                     samples_written+=1;
                //                 }
                //             },
                //             _ => (),
                //         }
                //     });
                // });

                // // 5秒待機
                // debug!("X:before sleep...");
                // thread::sleep(Duration::from_secs(5));
                // debug!("X:after  sleep!!!");

//                event_loop.destroy_stream(stream_id);
            });
        } else {
            // ボタンOFF
            self.wave_button.set_text(&qs("Wave Data Start"));
        }

        // 3秒待機
        debug!("1:before sleep...");
        thread::sleep(Duration::from_secs(3));
        debug!("1:after  sleep!!!");

        tx.send("Hello world!").unwrap();

        // debug!("2:before sleep...");
        // thread::sleep(Duration::from_secs(3));
        // debug!("2:after  sleep!!!");

        // tx.send("Stop sound!").unwrap();

        // // 別スレッドの完了を待機
        // handle.join().unwrap();

        debug!("on_wave_button_clicked() END.");
    }

    #[slot(SlotNoArgs)]
    unsafe fn on_sequence_button_clicked(self: &Rc<Self>) {
        debug!("on_sequence_button_clicked() BEGIN.");

        fn run<T>(device: &cpal::Device, config: &cpal::StreamConfig) -> Result<(), anyhow::Error>
        where
            T: cpal::Sample,
        {
            let sample_rate = config.sample_rate.0 as f32;
            let channels = config.channels as usize;

            // wavファイル読み込み
            // https://docs.rs/hound/3.4.0/hound/struct.WavReader.html
            let mut reader = hound::WavReader::open("assets/wav/test/2608_sd.wav").unwrap();
            let spec = reader.spec();
            println!("channels is {}", spec.channels);
            println!("sample_rate is {}", spec.sample_rate);
            println!("bits_per_sample is {}", spec.bits_per_sample);
            match spec.sample_format {
                hound::SampleFormat::Float => println!("SampleFormat is WAVE_FORMAT_IEEE_FLOAT"),
                hound::SampleFormat::Int => println!("SampleFormat is WAVE_FORMAT_PCM"),
            }
            let samples = reader.samples::<i16>();

            // 1秒44100サンプル、BPM155、4つ打ちの場合のwavサンプル配置タイミングをtime_calcで計算
            const SAMPLE_HZ: SampleHz = 44_100.0;
            let bpm: Bpm = 155.0;
            let time_sig = TimeSig{top: 4, bottom: 4};
            println!(
                "Convert 1 bars to samples where the tempo is 155bpm, the time signature is 4/4
                    and the sample rate is 44,100 samples per second: {}",
                Bars(1).samples(bpm, time_sig, SAMPLE_HZ)
            );
            println!(
                "Convert 2 bars to samples where the tempo is 155bpm, the time signature is 4/4
                    and the sample rate is 44,100 samples per second: {}",
                Bars(2).samples(bpm, time_sig, SAMPLE_HZ)
            );
            println!(
                "Convert 3 bars to samples where the tempo is 155bpm, the time signature is 4/4
                    and the sample rate is 44,100 samples per second: {}",
                Bars(3).samples(bpm, time_sig, SAMPLE_HZ)
            );
            println!(
                "Convert 4 bars to samples where the tempo is 155bpm, the time signature is 4/4
                    and the sample rate is 44,100 samples per second: {}",
                Bars(4).samples(bpm, time_sig, SAMPLE_HZ)
            );

            // TODO: バスドラ4つ打ちのサンプルベクタをコンパイルエラーなく完成させる。

            // let length = Bars(4).samples(bpm, time_sig, SAMPLE_HZ) as usize;
            // let mut fourBassDrumsVec: Vec<i16> = vec![0; length];

            // let mut idx: usize = 0;
            // for (i, val) in samples.enumerate() {
            //     fourBassDrumsVec[idx] = val.unwrap();
            //     idx += 1;
            // }

            // let mut idx = Bars(1).samples(bpm, time_sig, SAMPLE_HZ) as usize;
            // for (i, val) in samples.enumerate() {
            //     fourBassDrumsVec[idx] = val.unwrap();
            //     idx += 1;
            // }

            // let mut idx = Bars(2).samples(bpm, time_sig, SAMPLE_HZ) as usize;
            // for (i, val) in samples.enumerate() {
            //     fourBassDrumsVec[idx] = val.unwrap();
            //     idx += 1;
            // }
            
            // let mut idx = Bars(3).samples(bpm, time_sig, SAMPLE_HZ) as usize;
            // for (i, val) in samples.enumerate() {
            //     fourBassDrumsVec[idx] = val.unwrap();
            //     idx += 1;
            // }

            // println!("fourBassDrumsVec:[{:?}]", fourBassDrumsVec);

            // https://www.fabrica-com.co.jp/techblog/technology/1118/
            // のノイズジェネレーターの代わりに、1小節分のサンプルVecを作って
            // メソッドで呼ぶたびにサンプルを取り出せれば良いのでは。
            let mut sample_clock = 0f32;
            let mut next_value = move || {
                sample_clock = (sample_clock + 1.0) % sample_rate;
                (sample_clock * 440.0 * 2.0 * 3.141592 / sample_rate).sin()
            };

            let err_fn = |err| eprintln!("an error occurred on stream: {}", err);

            let stream = device.build_output_stream(
                config,
                move |data: &mut [T], _: &cpal::OutputCallbackInfo| {
                    write_data(data, channels, &mut next_value)
                },
                err_fn,
            )?;
            stream.play()?;

            debug!("zz:before sleep...");
            std::thread::sleep(std::time::Duration::from_millis(4000));
            debug!("zz:before sleep!!!");

            Ok(())
        }

        fn write_data<T>(output: &mut [T], channels: usize, next_sample: &mut dyn FnMut() -> f32)
        where
            T: cpal::Sample,
        {
            for frame in output.chunks_mut(channels) {
                let value: T = cpal::Sample::from::<f32>(&next_sample());
                for sample in frame.iter_mut() {
                    *sample = value;
                }
            }
        }

        let sequence_button_thread_handle = thread::spawn(move || {
            let host = cpal::default_host();
            let device = host.default_output_device().expect("no output device available");
            let config = device.default_output_config().expect("no output config available");

            match config.sample_format() {
                cpal::SampleFormat::F32 => run::<f32>(&device, &config.into()).unwrap(),
                cpal::SampleFormat::I16 => run::<i16>(&device, &config.into()).unwrap(),
                cpal::SampleFormat::U16 => run::<u16>(&device, &config.into()).unwrap(),
            }
        });






//         let mut count = 0;
//         let sqr_sum = reader.samples::<i16>()
//                             .fold(0.0, move |sqr_sum, s| {
                                
//             let sample = s.unwrap() as f64;
// //            println!("sample[{}] is {}", count, sample);
//             count += 1;
//             sqr_sum + sample * sample
//         });
//         println!("RMS is {}", (sqr_sum / reader.len() as f64).sqrt());

//         // TODO: https://github.com/ruuda/hound/blob/master/examples/cpal.rs

//         // TODO: 今まではOUTPUTデバイスのみで波形のリアルタイム計算の音が鳴らせてたが、
//         //       INPUTデバイスを使えば読み込んだwavを放り込んだりできるのか？
//         // https://github.com/RustAudio/cpal/blob/master/examples/feedback.rs
//         // https://users.rust-lang.org/t/data-become-corrupted-through-tcp/35462
//         let host = cpal::default_host();
//         let input_device = host.default_input_device().expect("no output device available");
//         let output_device = host.default_output_device().expect("no output device available");
// //        let format = output_device.default_output_format().unwrap();
//         println!("Using default input device: \"{}\"", input_device.name().unwrap());
//         println!("Using default output device: \"{}\"", output_device.name().unwrap());
//         println!("{:?}", format);

//         // MEMO: rodioのSinkは続けて再生には使えそうだが特定のタイミングで鳴らす機能ではなさそう：https://docs.rs/rodio/0.11.0/rodio/



//         // TODO: Cargo.tomlでは「cpal = "0.11.0"」とを指定しているのに、何故かソースが古くてコンパイルが通らない件。
//         //       以下を指定するとStreamConfigが見つかるようになる・・・が、他の記述の最新化をしてあげる必要あり。
//         //       cpal = { git = "https://github.com/RustAudio/cpal" }
//         let config: cpal::StreamConfig = input_device.default_input_config().unwrap().into();

//         // TODO: ①44100khz、BPM155、1小節（4拍）分のサンプルデータ配列を全要素の値0で初期設定
//         // TODO: ②4つ打ちタイミングを算出し、算出タイミング4箇所を開始地点としてwavのサンプルデータ配列の値を上書き
//         //       time_calcクレートが使えそう？自前計算でもいったんよさそう。music-timerクレートはリアルタイムっぽいのでいったんやめておく。
//         //       https://docs.rs/crate/time_calc/0.13.0
//         //       https://github.com/unsignedbytebite/music-timer
//         //       https://kichizyo.hatenablog.jp/entry/2019/08/27/214125
//         //       https://harmonic-sound.com/%E9%9F%B3%E7%AC%A6%E3%81%A8%E3%82%B5%E3%83%B3%E3%83%97%E3%83%AA%E3%83%B3%E3%82%B0%E5%9B%9E%E6%95%B0/
//         //       https://sleepfreaks-dtm.com/produce-recipe/unit/
//         // TODO: ③予めwavのサンプルデータ配列を用意しておき、非リアルタイムでサンプルデータ配列を再生する方法（cpalでどうやる？他の方法のほうがいい？）


//         // TODO: （優先度低）libsoundioを使ってみる。：https://qiita.com/MachiaWorx/items/b213eab29c0c2d81026c


// /*
//         let event_loop = host.event_loop();
//         let format = output_device.default_output_format().unwrap();
//         let stream_id = event_loop.build_output_stream(&output_device, &format).unwrap();
//         println!("{:?}", stream_id);
    
//         event_loop.play_stream(stream_id);

//         let sequence_button_thread_handle = thread::spawn(move || {
//             thread::spawn(move || {
//                 event_loop.run(move |stream_id, stream_result| {
//                     let stream_data = match stream_result {
//                         Ok(data) => data,
//                         Err(err) => {
//                             eprintln!("an error occurred on stream {:?}: {}", stream_id, err);
//                             return;
//                         }
//                         _ => return,
//                     };

//                     let mut samples_written: u64 = 0;
                
//                     match stream_data {
//                         /* TODO: どういう時に必要になる？
//                         *       デフォルトのformatは「Format { channels: 2, sample_rate: SampleRate(48000), data_type: F32 }」
//                         *      だったのでF32のルートしか動かなかった。
//                         */
//                         // StreamData::Output { buffer: UnknownTypeOutputBuffer::U16(mut buffer) } => {
//                         //     println!("1");
//                         //     for elem in buffer.iter_mut() {
//                         //         *elem = u16::max_value() / 2;
//                         //     }
//                         // },
//                         // StreamData::Output { buffer: UnknownTypeOutputBuffer::I16(mut buffer) } => {
//                         //     println!("2");
//                         //     for elem in buffer.iter_mut() {
//                         //         *elem = 0;
//                         //     }
//                         // },
//                         StreamData::Output { buffer: UnknownTypeOutputBuffer::F32(mut buffer) } => {
//                             for elem in buffer.iter_mut() {
//                                 // MEMO: https://gist.github.com/ablwr/c02ccd4f6c48bd90614647ec9dbd3380
//                                 let time = (samples_written / format.channels as u64) as f32 
//                                         / format.sample_rate.0 as f32;
//                                 let t = time / (1.0/220.0) % 1.0;

//                                 if t < 0.5 {
//                                     *elem = 0.4;
//                                 } else {
//                                     *elem = -0.4; 
//                                 }      

//                                 samples_written+=1;
//                             }
//                         },
//                         _ => (),
//                     }
//                 });
//             });
//         });
// */
//         debug!("A:before sleep...");
//         thread::sleep(Duration::from_secs(5));
//         debug!("A:after  sleep!!!");

        debug!("on_sequence_button_clicked() END.");
    }

    #[slot(SlotOfQTableWidgetItemQTableWidgetItem)]
    unsafe fn on_table_current_item_changed(
        self: &Rc<Self>,
        current: Ptr<QTableWidgetItem>,
        previous: Ptr<QTableWidgetItem>,
    ) {
        if !previous.is_null() {
            let font = previous.font();
            font.set_bold(false);
            previous.set_font(&font);
        }
        if !current.is_null() {
            let font = current.font();
            font.set_bold(true);
            current.set_font(&font);
        }
    }

    #[slot(SlotOfQPoint)]
    unsafe fn on_table_context_menu_requested(self: &Rc<Self>, pos: Ref<QPoint>) {
        let global_pos = self.table.viewport().map_to_global(pos);
        let menu = QMenu::new();
        menu.add_action_q_string(&qs("Fake action 1"));
        menu.add_action_q_string(&qs("Fake action 2"));

        let action3 = QAction::from_q_string(&qs("Real action"));
        menu.add_action(&action3);

        let action = menu.exec_1a_mut(&global_pos);
        let message = if action.is_null() {
            "No action selected!".to_string()
        } else if action.as_raw_ptr() == action3.as_raw_ptr() {
            "Real action selected!".to_string()
        } else {
            format!("Fake action selected ({})", action.text().to_std_string())
        };
        QMessageBox::information_q_widget2_q_string(&self.widget, &qs("Example"), &qs(message));
    }
}

pub fn gui_proc() {
    debug!("gui_proc() BEGIN.");

    QApplication::init(|_| unsafe {
        let _form = Form::new();
        QApplication::exec()
    })
}