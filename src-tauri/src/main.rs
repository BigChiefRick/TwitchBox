#![cfg_attr(
	all(not(debug_assertions), target_os = "windows"),
	windows_subsystem = "windows"
)]

use tauri::Manager;

#[derive(Clone, serde::Serialize)]
struct Payload {
	args: Vec<String>,
	cwd: String,
}

fn main() {
	tauri::Builder::default()
	.setup(|app| {
		let main_window = tauri::WindowBuilder::new(
			app,
			"main",
			tauri::WindowUrl::External("https://dashboard.twitch.tv/u/ticklefitz/stream-manager".parse().unwrap())
		)
		.title("TwitchBox - Stream Manager")
		.min_inner_size(1400.0, 700.0)
		.fullscreen(false)
		.focused(true)
		.resizable(true)
		.decorations(true)
		.center(true)
		// Initialization scripts run at the native WebView2 level before CSP is applied,
		// which allows loading external scripts like FFZ just like a browser extension would.
		.initialization_script(r#"
			// DISABLE KEYBOARD SHORTCUTS
			window.addEventListener('keydown', function(e) {
				if (e.keyCode == 116) e.preventDefault();                              // F5 (reload)
				if (e.ctrlKey && e.shiftKey && e.keyCode == 82) e.preventDefault();    // CTRL+SHIFT+R (hard refresh)
				if (e.ctrlKey && e.keyCode == 85) e.preventDefault();                  // CTRL+U (view source)
				if (e.ctrlKey && e.keyCode == 80) e.preventDefault();                  // CTRL+P (print)
				if (e.ctrlKey && e.shiftKey && e.keyCode == 80) e.preventDefault();    // CTRL+SHIFT+P (print setup)
				if (e.ctrlKey && e.shiftKey && e.keyCode == 83) e.preventDefault();    // CTRL+SHIFT+S (screenshot)
				if (e.ctrlKey && e.shiftKey && e.keyCode == 88) e.preventDefault();    // CTRL+SHIFT+X (screenshot)
				if (e.ctrlKey && e.shiftKey && e.keyCode == 73) e.preventDefault();    // CTRL+SHIFT+I (devtools)
				if (e.keyCode == 118) e.preventDefault();                              // F7 (caret browsing)
			});
			// FIND IN PAGE (CTRL+F) ENABLED for Stream Manager dashboard search
			// RIGHT CLICK ENABLED for Stream Manager mod tools
			window.addEventListener('auxclick', function(e) {
				if (e.button == 1) e.preventDefault(); // middle-click
			});
		"#)
		.initialization_script(r#"
			// LOAD FRANKERFACEZ (with BTTV & 7TV add-on support)
			(function() {
				var script = document.createElement('script');
				script.src = 'https://cdn.frankerfacez.com/script/frankerfacez.min.js';
				document.head.appendChild(script);
			})();
		"#)
		.build()?;

		Ok(())
	})
	.plugin(tauri_plugin_window_state::Builder::default().build())
	.plugin(tauri_plugin_single_instance::init(|app, argv, cwd| {
		println!("{}, {argv:?}, {cwd}", app.package_info().name);
		app.emit_all("single-instance", Payload { args: argv, cwd }).unwrap();
	})) // Blocking Multiple Instances
	.run(tauri::generate_context!())
	.expect("failed to run app");
}
