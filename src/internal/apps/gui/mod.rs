use cpp_core::{Ptr, Ref, StaticUpcast};
use qt_core::{qs, slot, ContextMenuPolicy, QBox, QObject, QPoint, SlotNoArgs};
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

struct Form {
    widget: QBox<QWidget>,
    line_edit: QBox<QLineEdit>,
    button: QBox<QPushButton>,
    table: QBox<QTableWidget>,
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

            let button = QPushButton::from_q_string(&qs("Start"));
            button.set_enabled(false);
            layout.add_widget(&button);

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
                button,
                line_edit,
                table,
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
        // 別スレッドを起動し音声を再生
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