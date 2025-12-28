//! Root CEF application object.

use cef::*;
use cef::rc::*;
use std::sync::{Arc, Mutex};

use crate::browser::DemoBrowserProcessHandler;

wrap_app! {
    pub struct DemoApp {
        window: Arc<Mutex<Option<Window>>>,
        start_url: CefString,
    }

    impl App {
        fn browser_process_handler(&self) -> Option<BrowserProcessHandler> {
            Some(
                DemoBrowserProcessHandler::new(
                    self.window.clone(),
                    self.start_url.clone(),
                )
            )
        }
    }
}
