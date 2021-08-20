
#[macro_export]
/// Convert regular expression to a native string, to be passable as an argument in WinAPI
macro_rules! native_str {
    ($str: expr) => {
        format!("{}\0", $str).as_ptr() as *const i8
    };
}

use winapi::{shared::windef::HWND, um::winuser::FindWindowA};

/// Get handle to window with (optional) class name and (optional) title
pub fn find_window(class: Option<*const i8>, name: Option<*const i8>) -> Option<HWND> {
    let hwnd = unsafe {
        FindWindowA(
            match class {
                Some(e) => e,
                _ => std::ptr::null_mut(),
            },
            match name {
                Some(e) => e,
                _ => std::ptr::null_mut(),
            },
        )
    };

    if hwnd.is_null() {
        return None;
    }

    Some(hwnd)
}
