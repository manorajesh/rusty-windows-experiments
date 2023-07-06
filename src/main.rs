use windows::{
    core::*, Win32::Foundation::*, Win32::Graphics::Gdi::ValidateRect,
    Win32::{System::LibraryLoader::GetModuleHandleA, Graphics::Gdi::{BeginPaint, PAINTSTRUCT, FillRect, COLOR_WINDOW, GetSysColorBrush, LineTo, InvalidateRect, MoveToEx}}, Win32::UI::WindowsAndMessaging::*,
};

fn main() -> Result<()> {
    unsafe {
        let instance = GetModuleHandleA(None)?;
        debug_assert!(instance.0 != 0);

        let window_class = s!("window");

        let wc = WNDCLASSA {
            hCursor: LoadCursorW(None, IDC_ARROW)?,
            hInstance: instance,
            lpszClassName: window_class,

            style: CS_HREDRAW | CS_VREDRAW,
            lpfnWndProc: Some(wndproc),
            ..Default::default()
        };

        let atom = RegisterClassA(&wc);
        debug_assert!(atom != 0);

        CreateWindowExA(
            WINDOW_EX_STYLE::default(),
            window_class,
            s!("This is a sample window"),
            WS_OVERLAPPEDWINDOW | WS_VISIBLE,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            None,
            None,
            instance,
            None,
        );

        let mut message = MSG::default();

        while GetMessageA(&mut message, None, 0, 0).into() {
            DispatchMessageA(&message);
        }

        Ok(())
    }
}

extern "system" fn wndproc(window: HWND, message: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    static mut COUNTER: i32 = 0;
    static mut x: isize = 0;
    static mut y: isize = 0;
    unsafe {
        match message {
            WM_PAINT => {
                // println!("WM_PAINT {:?}, {:?}, {:?}", COUNTER, x, y);
                COUNTER += 1;
                let mut ps = PAINTSTRUCT::default();
                let hdc =BeginPaint(window, &mut ps);
                FillRect(hdc, &ps.rcPaint, GetSysColorBrush(COLOR_WINDOW));
                for i in 100..2000 {
                    LineTo(hdc, x as i32+i, y as i32+i);
                    MoveToEx(hdc, 0, 0, None);
                }
                LRESULT(0)
            }
            WM_DESTROY => {
                // println!("WM_DESTROY");
                PostQuitMessage(0);
                LRESULT(0)
            }
            WM_LBUTTONDOWN => {
                x = lparam.0 & 0x0000ffff;
                y = (lparam.0 & 0xffff0000) >> 16;
                // println!("{:x}, {:x}, {:x}", lparam.0, x, y);
                InvalidateRect(window, None, true);
                LRESULT(0)
            }
            WM_MOUSEMOVE => {
                x = lparam.0 & 0x0000ffff;
                y = (lparam.0 & 0xffff0000) >> 16;
                // println!("{:x}, {:x}, {:x}", lparam.0, x, y);
                InvalidateRect(window, None, true);
                LRESULT(0)
            }
            _ => DefWindowProcA(window, message, wparam, lparam),
        }
    }
}