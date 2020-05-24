use cpp_core::{Ptr, Ref, StaticUpcast};
use qt_core::{qs, slot, ContextMenuPolicy, QBox, QObject, QPoint, SlotNoArgs, SlotOfBool};
use qt_widgets::{
    QAction, QApplication, QLineEdit, QMenu, QMessageBox, QPushButton, QTableWidget,
    QTableWidgetItem, QVBoxLayout, QWidget, SlotOfQPoint, SlotOfQTableWidgetItemQTableWidgetItem,
};
use std::rc::Rc;

use std::fs::File;
use std::io::BufReader;
use rodio::Source;
use std::thread;
use std::time::Duration;
use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;

use cpal::{EventLoop, StreamData, UnknownTypeOutputBuffer};
use cpal::traits::{HostTrait, DeviceTrait, EventLoopTrait};

struct Form {
    widget: QBox<QWidget>,
    line_edit: QBox<QLineEdit>,
    button: QBox<QPushButton>,
    table: QBox<QTableWidget>,
    wave_button: QBox<QPushButton>,

//    wave_button_playing_flg: bool,
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

            let button = QPushButton::from_q_string(&qs("wav file Play"));
            button.set_enabled(false);
            layout.add_widget(&button);

            // MEMO: https://doc.qt.io/qt-5/qpushbutton.html
            let wave_button = QPushButton::from_q_string(&qs("Wave Data Start"));
            // MEMO: https://doc.qt.io/qt-5/qabstractbutton.html#checkable-prop
            //       http://yu00.hatenablog.com/entry/2015/09/15/185014
            wave_button.set_checkable(true);
            layout.add_widget(&wave_button);

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

//            let wave_button_playing_flg = false;

            let this = Rc::new(Self {
                widget,
                button,
                line_edit,
                table,
                wave_button,
//                wave_button_playing_flg,
            });
            this.init();
            this
        }
    }

    unsafe fn init(self: &Rc<Self>) {
        self.button
            .clicked()
            .connect(&self.slot_on_button_clicked());
        self.line_edit
            .text_edited()
            .connect(&self.slot_on_line_edit_text_edited());
        self.table
            .current_item_changed()
            .connect(&self.slot_on_table_current_item_changed());
        self.table
            .custom_context_menu_requested()
            .connect(&self.slot_on_table_context_menu_requested());
        self.wave_button
            .toggled()
            .connect(&self.slot_on_wave_button_clicked());
    }

    #[slot(SlotNoArgs)]
    unsafe fn on_line_edit_text_edited(self: &Rc<Self>) {
        self.button.set_enabled(!self.line_edit.text().is_empty());
    }

    #[slot(SlotNoArgs)]
    unsafe fn on_button_clicked(self: &Rc<Self>) {
        debug!("on_button_clicked() BEGIN.");

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

        debug!("on_button_clicked() END.");
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
            // 別スレッドを起動し音声を再生
            /* MEMO: moveキーワード
             *       メインスレッドの値を利用できるようにする。
             *       今回はチャンネル共有のために利用。
             */
            let wave_button_thread_handle = thread::spawn(move || {
                // MEMO: https://docs.rs/cpal/0.11.0/cpal/
                let host = cpal::default_host();
                let device = host.default_output_device().expect("no output device available");
                let format = device.default_output_format().unwrap();
                println!("{:?}", format);

                let event_loop = host.event_loop();
                let stream_id = event_loop.build_output_stream(&device, &format).unwrap();
                println!("{:?}", stream_id);
            
                let mut samples_written: u64 = 0;

                debug!("before recv...");
                println!("{}", rx.recv().unwrap());
                debug!("after  recv!!!");

                // TODO: これがないと音がならない。
                //       playした後、どうやってpause_streamやdestroy_streamを実行する？
                //       https://999eagle.moe/posts/rust-video-player-part-4/
                event_loop.play_stream(stream_id);
                thread::spawn(move || {
                    event_loop.run(move |stream_id, stream_result| {
                        // match rx.try_recv() {
                        //     Ok(send_data) => {
                        //         debug!("{:?}", send_data);
                        //         event_loop.destroy_stream(stream_id);
                        //     },
                        //     Err(err) => {
                        //         debug!("{:?}", err);
                        //     }
                        // }
                        
                        let stream_data = match stream_result {
                            Ok(data) => data,
                            Err(err) => {
                                eprintln!("an error occurred on stream {:?}: {}", stream_id, err);
                                return;
                            }
                            _ => return,
                        };

                        let mut samples_written: u64 = 0;
                    
                        match stream_data {
                            /* TODO: どういう時に必要になる？
                            *       デフォルトのformatは「Format { channels: 2, sample_rate: SampleRate(48000), data_type: F32 }」
                            *      だったのでF32のルートしか動かなかった。
                            */
                            // StreamData::Output { buffer: UnknownTypeOutputBuffer::U16(mut buffer) } => {
                            //     println!("1");
                            //     for elem in buffer.iter_mut() {
                            //         *elem = u16::max_value() / 2;
                            //     }
                            // },
                            // StreamData::Output { buffer: UnknownTypeOutputBuffer::I16(mut buffer) } => {
                            //     println!("2");
                            //     for elem in buffer.iter_mut() {
                            //         *elem = 0;
                            //     }
                            // },
                            StreamData::Output { buffer: UnknownTypeOutputBuffer::F32(mut buffer) } => {
                                for elem in buffer.iter_mut() {
                                    // MEMO: https://gist.github.com/ablwr/c02ccd4f6c48bd90614647ec9dbd3380
                                    let time = (samples_written / format.channels as u64) as f32 
                                            / format.sample_rate.0 as f32;
                                    let t = time / (1.0/220.0) % 1.0;

                                    if t < 0.5 {
                                        *elem = 0.4;
                                    } else {
                                        *elem = -0.4; 
                                    }      

                                    samples_written+=1;
                                }
                            },
                            _ => (),
                        }
                    });
                });

                // 5秒待機
                debug!("X:before sleep...");
                thread::sleep(Duration::from_secs(5));
                debug!("X:after  sleep!!!");

                event_loop.destroy_stream(stream_id);
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