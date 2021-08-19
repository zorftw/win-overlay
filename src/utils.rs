macro_rules! native_str {
    ($str: expr) => {
        format!("{}\0", $str).as_ptr() as *const i8
    };
}

use winapi::{shared::windef::HWND, um::winuser::FindWindowA};

pub fn find_window(class: Option<&str>, name: Option<&str>) -> Option<HWND> {
    let hwnd = unsafe {
        FindWindowA(
            match class {
                Some(e) => native_str!(e),
                _ => std::ptr::null_mut(),
            },
            match name {
                Some(e) => native_str!(e),
                _ => std::ptr::null_mut(),
            },
        )
    };

    if hwnd.is_null() {
        return None;
    }

    Some(hwnd)
}
