use cef::{args::Args, *};
use std::sync::{Arc, Mutex};

use crate::app::DemoApp;

/// Public entry point for launching a CEF application.
///
/// Responsible for:
/// - Initializing platform-specific CEF requirements
/// - Spawning CEF subprocesses
/// - Starting the browser process
/// - Running the CEF message loop
pub struct Runtime;

impl Runtime {
    /// Launches the CEF runtime and blocks until shutdown.
    ///
    /// start_url determines what the browser loads on startup.
    pub fn run(start_url: CefString) {
        #[cfg(target_os = "macos")]
        crate::platform::macos::init_ns_app();

        let _ = api_hash(sys::CEF_API_VERSION_LAST, 0);

        let args = Args::new();
        let cmd = args.as_cmd_line().unwrap();

        // Determines whether this process is the main browser process
        let switch = CefString::from("type");
        let is_browser_process = cmd.has_switch(Some(&switch)) != 1;

        let window = Arc::new(Mutex::new(None));
        let mut app = DemoApp::new(window.clone(), start_url);

        let ret = execute_process(
            Some(args.as_main_args()),
            Some(&mut app),
            std::ptr::null_mut(),
        );

        // Subprocesses exit immediately
        if !is_browser_process {
            return;
        }

        assert!(ret == -1);

        let settings = Settings {
            no_sandbox: 1,
            ..Default::default()
        };

        assert_eq!(
            initialize(
                Some(args.as_main_args()),
                Some(&settings),
                Some(&mut app),
                std::ptr::null_mut(),
            ),
            1
        );

        run_message_loop();
        shutdown();
    }
}
