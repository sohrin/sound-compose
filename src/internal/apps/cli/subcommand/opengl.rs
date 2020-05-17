use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::time::Duration;
use std::thread::sleep;

pub fn opengl_proc() -> Result<(), sdl2::Error>  {
    debug!("opengl_proc() BEGIN.");

    // TODO: 例外処理を.unwrap()で手を抜かずにやる。

    // SDL2の初期化
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("SDL", 640, 480)
        .position_centered()
        .build()
        .unwrap();
    
    let mut canvas = window.into_canvas().build().unwrap();
    canvas.set_draw_color(Color::RGB(255, 255, 255));
    canvas.clear();
    canvas.present();

    let mut event_pump = sdl_context.event_pump().unwrap();
    /*
     * MEMO: ラベル
     *       "'"(半角シングルクォート)を付けて定義する。break等でラベルを指定する場合も同様。
     *       https://doc.rust-jp.rs/rust-by-example-ja/flow_control/loop/nested.html
     */
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }

            canvas.present();

            sleep(Duration::new(0, 1_000_000_000u32 / 60));
        }
    }

    return Ok(());
}