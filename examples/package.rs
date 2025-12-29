use rust_cef_runtime::Runtime;
use cef::CefString;
use std::path::PathBuf;

fn frontend_root() -> PathBuf {
    let exe = std::env::current_exe()
        .expect("Failed to determine executable path");

    exe.parent()
        .expect("Executable has no parent directory")
        .join("content")
}

fn main() {
    let frontend_root = frontend_root();

    if !frontend_root.exists() {
        panic!(
            "Frontend root directory not found: {}\n\
             Expected a 'content/' directory next to the executable.",
            frontend_root.display()
        );
    }

    std::env::set_current_dir(&frontend_root)
        .expect("Failed to set frontend root directory");

    Runtime::run(CefString::from("app://app/index.html"));
}
