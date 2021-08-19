use std::{sync::Mutex, time::SystemTime};

use winapi::{
    shared::{
        d3d9::{
            Direct3DCreate9, IDirect3D9, IDirect3DDevice9, D3DADAPTER_DEFAULT,
            D3DCREATE_HARDWARE_VERTEXPROCESSING, D3D_SDK_VERSION,
        },
        d3d9caps::D3DPRESENT_INTERVAL_IMMEDIATE,
        d3d9types::{
            D3DCLEAR_TARGET, D3DCOLOR_ARGB, D3DDEVTYPE_HAL, D3DFMT_A8R8G8B8, D3DPRESENT_PARAMETERS,
            D3DRECT, D3DSWAPEFFECT_DISCARD,
        },
        minwindef::{LPARAM, LRESULT, WPARAM},
        windef::{HWND, RECT},
    },
    um::{
        dwmapi::DwmExtendFrameIntoClientArea,
        uxtheme::MARGINS,
        wingdi::{CreateSolidBrush, RGB},
        winuser::{
            CreateWindowExA, DefWindowProcA, DestroyWindow, GetWindowRect, LoadCursorW,
            RegisterClassExA, SetLayeredWindowAttributes, SetWindowPos, ShowWindow, CS_HREDRAW,
            CS_VREDRAW, HWND_TOPMOST, IDC_ARROW, LWA_ALPHA, SWP_NOMOVE, SWP_NOSIZE, SW_SHOW,
            WM_DESTROY, WNDCLASSEXA, WS_EX_LAYERED, WS_EX_TRANSPARENT, WS_POPUP, WS_VISIBLE,
        },
    },
};

lazy_static! {
    static ref OVERLAY: Mutex<Overlay> = Mutex::new(Overlay::default());
    static ref LAST_TIME: Mutex<SystemTime> = Mutex::new(SystemTime::now());
}

#[derive(Default)]
pub struct Overlay {
    _target: usize,
    _overlay: usize,
    _d3d: usize,
    _device: usize,
    _fps: i32,
}

#[no_mangle]
unsafe extern "system" fn wnd_proc(wnd: HWND, msg: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    // set position of our window incase the target window moves
    {
        let overlay = OVERLAY.lock().unwrap();

        let mut rectangle = RECT {
            bottom: 0,
            left: 0,
            right: 0,
            top: 0,
        };

        // Get rectangle
        GetWindowRect(overlay.get_target(), &mut rectangle);

        // set position
        SetWindowPos(
            overlay.get_target(),
            overlay.get_overlay(),
            rectangle.left,
            rectangle.top,
            rectangle.right - rectangle.left,
            rectangle.bottom - rectangle.top,
            SWP_NOMOVE | SWP_NOSIZE,
        );
    }

    match msg {
        WM_DESTROY => std::process::exit(0),
        _ => {}
    }

    DefWindowProcA(wnd, msg, wparam, lparam)
}

impl Overlay {
    /// Ensure our window is positioned over the target window
    pub fn ensure_position(&self) {
        let rect = self.get_rect();

        unsafe {
            SetWindowPos(
                self.get_overlay(),
                HWND_TOPMOST,
                rect.left,
                rect.top,
                rect.right - rect.left,
                rect.bottom - rect.top,
                SWP_NOSIZE,
            );
        }
    }

    pub fn draw(&self, func: &dyn Fn()) {
        loop {
            self.begin_drawing();

            func();

            self.end_drawing();
        }
    }

    pub fn begin_drawing(&self) {
        self.ensure_position();

        let device = self.get_device();

        unsafe {
            (*device).Clear(
                0,
                std::ptr::null_mut(),
                D3DCLEAR_TARGET,
                D3DCOLOR_ARGB(0, 0, 0, 0),
                1f32,
                0,
            );
            (*device).BeginScene();
        }
    }

    pub fn end_drawing(&self) {
        //static mut FPS: i32 = 0;
        //let last_time = LAST_TIME.lock().unwrap();

        unsafe {
            // FPS += 1;

            // if SystemTime::now()
            //     .duration_since(*last_time)
            //     .expect("Somehow unable to get difference in time...")
            //     >= std::time::Duration::from_secs(1)
            // {
            //     *LAST_TIME.lock().unwrap() = SystemTime::now();

            //     self._fps = FPS;
            //     FPS = 0;
            // }

            let device = self.get_device();
            (*device).EndScene();
            (*device).Present(
                std::ptr::null_mut(),
                std::ptr::null_mut(),
                std::ptr::null_mut(),
                std::ptr::null_mut(),
            );
        }
    }

    pub fn draw_filled_box(&self, x: i32, y: i32, w: i32, h: i32, color: u32) {
        unsafe {
            let device = self.get_device();

            let rect: D3DRECT = D3DRECT {
                x1: x,
                x2: x + w,
                y1: y,
                y2: y + h,
            };

            //device.Clear(1, &const rect as *const _, D3DCLEAR_TARGET, color, 0, 0);
            (*device).Clear(1, &rect as *const _, D3DCLEAR_TARGET, color, 0f32, 0);
        }
    }

    pub fn draw_box(&self, x: i32, y: i32, w: i32, h: i32, t: i32, color: u32) {
        self.draw_filled_box(x, y, t, h, color); // draw left vertical
        self.draw_filled_box(x + w, y, t, h, color); // draw right vertical
        self.draw_filled_box(x, y, w, t, color); // draw top horizontal
        self.draw_filled_box(x, y + h, w, t, color); // draw bottom horizontal
    }

    pub fn get_fps(&self) -> i32 {
        self._fps
    }

    pub fn get_d3d(&self) -> *mut IDirect3D9 {
        self._d3d as *mut _
    }

    pub fn get_device(&self) -> *mut IDirect3DDevice9 {
        self._device as *mut _
    }

    pub fn get_rect(&self) -> RECT {
        let rect = [0i8; std::mem::size_of::<RECT>()].as_mut_ptr() as *mut RECT;

        // get dimensions of target window
        unsafe {
            GetWindowRect(self.get_target(), rect);
            rect.read()
        }
    }

    pub fn get_overlay(&self) -> HWND {
        self._overlay as HWND
    }

    pub fn get_target(&self) -> HWND {
        self._target as HWND
    }

    pub fn init_dx9(&mut self) {
        // create object
        let d3d = unsafe { Direct3DCreate9(D3D_SDK_VERSION) };

        if d3d.is_null() {
            panic!("Unable to create d3d9x object...");
        }

        // set it
        self._d3d = d3d as usize;

        // Create present
        let present = [0i8; std::mem::size_of::<D3DPRESENT_PARAMETERS>()].as_mut_ptr()
            as *mut D3DPRESENT_PARAMETERS;

        // Get dimensions
        let rect = self.get_rect();

        unsafe {
            (*present).Windowed = 1;
            (*present).SwapEffect = D3DSWAPEFFECT_DISCARD;
            (*present).BackBufferFormat = D3DFMT_A8R8G8B8;
            (*present).BackBufferWidth = (rect.right - rect.left) as u32;
            (*present).BackBufferHeight = (rect.bottom - rect.top) as u32;
            (*present).hDeviceWindow = self.get_overlay();
            (*present).PresentationInterval = D3DPRESENT_INTERVAL_IMMEDIATE;
        }

        let mut device = std::ptr::null_mut();

        if unsafe {
            (*d3d).CreateDevice(
                D3DADAPTER_DEFAULT,
                D3DDEVTYPE_HAL,
                self.get_overlay(),
                D3DCREATE_HARDWARE_VERTEXPROCESSING,
                present,
                &mut device,
            )
        } < 0
        {
            unsafe {
                (*d3d).Release();
            }
            panic!("Failed to create device");
        }

        self._device = device as usize;
    }

    pub fn create_overlay(target: HWND) -> Self {
        let mut wc = WNDCLASSEXA {
            cbSize: std::mem::size_of::<WNDCLASSEXA>() as u32,
            style: CS_HREDRAW | CS_VREDRAW,
            lpfnWndProc: Some(wnd_proc),
            cbClsExtra: 0,
            cbWndExtra: 0,
            hInstance: 0usize as _,
            hIcon: 0usize as _,
            hCursor: unsafe { LoadCursorW(std::ptr::null_mut(), IDC_ARROW) },
            hbrBackground: unsafe { CreateSolidBrush(RGB(0, 0, 0)) },
            lpszMenuName: native_str!(""),
            lpszClassName: native_str!("win-overlay::overlay"),
            hIconSm: std::ptr::null_mut(),
        };

        // register it
        if unsafe { RegisterClassExA(&mut wc as *mut _) } == 0 {
            std::panic!("Unable to register window class!");
        }

        let rect = [0i8; std::mem::size_of::<RECT>()].as_mut_ptr() as *mut RECT;

        // get dimensions of target window
        unsafe { GetWindowRect(target, rect) };

        // our own style
        let styles = WS_EX_LAYERED | WS_EX_TRANSPARENT;

        // create our own window
        let window = unsafe {
            CreateWindowExA(
                styles,
                native_str!("win-overlay::overlay"),
                native_str!(""),
                WS_POPUP | WS_VISIBLE,
                rect.read().left,
                rect.read().top,
                rect.read().right - rect.read().left,
                rect.read().bottom - rect.read().top,
                std::ptr::null_mut(),
                std::ptr::null_mut(),
                std::ptr::null_mut(),
                std::ptr::null_mut(),
            )
        };

        // test if we actually created the window
        if window.is_null() {
            panic!("Unable to create window");
        }

        // let's not do any stuff ourself
        let margins: *mut MARGINS =
            [0i8; std::mem::size_of::<MARGINS>()].as_mut_ptr() as *mut MARGINS;
        unsafe {
            (*margins).cxLeftWidth = rect.read().left;
            (*margins).cxRightWidth = rect.read().top;
            (*margins).cyTopHeight = rect.read().right - rect.read().left;
            (*margins).cyBottomHeight = rect.read().bottom - rect.read().top;
            DwmExtendFrameIntoClientArea(window, margins);

            // let is use alpha
            SetLayeredWindowAttributes(window, RGB(0, 0, 0), 255, LWA_ALPHA);

            // show our window
            ShowWindow(window, SW_SHOW);
        }

        let mut res = Self {
            _target: target as usize,
            _overlay: window as usize,
            _d3d: 0usize,
            _device: 0usize,
            _fps: 0i32,
        };

        res.init_dx9();

        res
    }
}

impl Drop for Overlay {
    /// Drop for overlay, handles the window we created, and the DirectX context.
    fn drop(&mut self) {
        println!("Overlay dropped...");

        if self._overlay != 0usize {
            unsafe {
                DestroyWindow(self.get_overlay());
            }
        }

        if self._d3d != 0usize {
            unsafe {
                (*self.get_d3d()).Release();
            }
        }

        if self._device != 0usize {
            unsafe {
                (*self.get_device()).Release();
            }
        }
    }
}
