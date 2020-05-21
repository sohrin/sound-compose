// TODO: 無くても窓は出てこなかった。
//#![windows_subsystem="windows"]

pub fn gui_proc() {
    debug!("gui_proc() BEGIN.");

    let mut frame = sciter::Window::new();
    frame.load_file("assets/html/minimal.html");
    frame.run_app();

    // TODO: https://github.com/sciter-sdk/rust-sciter/blob/master/examples/minimal.rs

    // TODO: inspector tool

    // TODO: menu bar

    // TODO: プルダウンの選択値を変えると表示がおかしいことがある。

    // TODO: どうやったら「include_bytes!」が使えるようになる？


    // // Step 1: Include the 'minimal.html' file as a byte array.
	// // Hint: Take a look into 'minimal.html' which contains some tiscript code.
    // let html = include_bytes!("minimal.htm");
    
	// // Step 2: Enable the features we need in our tiscript code.
	// sciter::set_options(sciter::RuntimeOptions::ScriptFeatures(
	// 	sciter::SCRIPT_RUNTIME_FEATURES::ALLOW_SYSINFO as u8		// Enables `Sciter.machineName()`
	// 	| sciter::SCRIPT_RUNTIME_FEATURES::ALLOW_FILE_IO as u8	// Enables opening file dialog (`view.selectFile()`)
    //     )).unwrap();
        

}