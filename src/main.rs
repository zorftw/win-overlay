use std::{alloc::System, time::SystemTime};

use winapi::shared::d3d9types::D3DCOLOR_RGBA;

#[cfg(not(windows))]
compile_error!("win-overlay is exclusive to windows.");

#[macro_use]
pub mod utils;
pub mod win_overlay;

pub fn main() {
    let runtime = std::panic::catch_unwind(|| {
        let target = utils::find_window(None, Some(native_str!("Untitled - Notepad"))).expect("Couldn't find window...");

        let overlay = win_overlay::Overlay::create_overlay(target);
        overlay.draw(&|| {
            overlay.draw_filled_box(20, 20, 100, 100, D3DCOLOR_RGBA(255, 0, 0, 255));
        });
    });

    match runtime {
        Err(e) => println!("{:?}", e),
        _ => println!("Succesfully executed"),
    }

    std::thread::sleep(std::time::Duration::from_secs(10));
}
