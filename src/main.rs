mod config;
mod magnifier;

#[macro_use]
extern crate ref_thread_local;
use ref_thread_local::RefThreadLocal;
use std::fs::File;
use std::path::Path;
use std::{error::Error, time::Instant};
use windows::{
    core::*, Win32::Foundation::*, Win32::System::Console::*, Win32::System::LibraryLoader::*,
    Win32::System::Threading::*, Win32::UI::Input::KeyboardAndMouse::*,
    Win32::UI::Magnification::*, Win32::UI::WindowsAndMessaging::*,
};

use config::*;
use magnifier::*;

const WM_UPDATE: u32 = WM_USER;

ref_thread_local! {
    static managed CONFIG: Config = Config::default();
}

fn main() {
    if let Err(error) = try_main() {
        // make null-terminated
        let mut error = error.to_string();
        error.push('\0');
        // encode to utf-16
        let error = error.encode_utf16().collect::<Vec<u16>>();
        unsafe {
            MessageBoxW(
                None,
                PCWSTR::from_raw(error.as_ptr()),
                h!("error"),
                MB_OK | MB_ICONERROR,
            );
        }
    }
}

fn try_main() -> std::result::Result<(), Box<dyn Error>> {
    // set config
    let default_config_file = get_default_config_file()?;
    *CONFIG.borrow_mut() = Config::new(default_config_file)?;

    // magnifier
    let mut magnifier = Magnifier::new();

    unsafe {
        // free console
        if FreeConsole().as_bool() == false {
            Err("failed to disable console")?
        };

        // init magnification
        if MagInitialize().as_bool() == false {
            Err("failed to run magnification")?
        }

        // set dpi-aware setting
        if SetProcessDPIAware().as_bool() == false {
            Err("failed to set dpi-aware setting")?
        }

        // register key shortcuts
        for (idx, shortcut) in CONFIG.borrow().shortcuts.iter().enumerate() {
            if let Combination::Key { modifiers, key } = shortcut.combination {
                let result = RegisterHotKey(HWND(0), idx as i32, modifiers, key);
                if result.as_bool() == false {
                    Err(format!("failed to register shortcut with idx {}", idx))?
                }
            }
        }

        // set mouse hook for checking mouse movements and mouse shortcuts
        SetWindowsHookExA(WH_MOUSE_LL, Some(mouse_hook), GetModuleHandleA(None)?, 0)?;

        // loop
        let mut msg = MSG::default();
        while GetMessageA(&mut msg, None, 0, 0).as_bool() {
            match msg.message {
                WM_HOTKEY => {
                    let shortcut = &mut CONFIG.borrow_mut().shortcuts[msg.wParam.0 as usize];
                    let elapsed = shortcut.last_run.elapsed().as_millis();
                    if elapsed >= shortcut.cooltime {
                        shortcut.last_run = Instant::now();
                        magnifier.do_action(&shortcut.action)?;
                    }
                }
                WM_UPDATE => magnifier.update()?,
                WM_QUIT => {
                    return Ok(());
                }
                _ => (),
            }
        }
    }

    Ok(())
}

unsafe extern "system" fn mouse_hook(code: i32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    if code < HC_ACTION as i32 {
        return CallNextHookEx(None, code, wparam, lparam);
    }

    let mut down = true;
    let mut extra = 0;
    let mut event = WM_NULL;

    // set event / extra / down?
    let hook_struct = unsafe { &*(lparam.0 as *const MSLLHOOKSTRUCT) };
    match wparam.0 as u32 {
        WM_MOUSEMOVE => {
            PostThreadMessageA(GetCurrentThreadId(), WM_UPDATE, WPARAM(0), LPARAM(0));
        }
        WM_LBUTTONDOWN => {
            event = WM_LBUTTONDOWN;
        }
        WM_LBUTTONUP => {
            down = false;
            event = WM_LBUTTONDOWN;
        }
        WM_MBUTTONDOWN => {
            event = WM_MBUTTONDOWN;
        }
        WM_MBUTTONUP => {
            down = false;
            event = WM_MBUTTONDOWN;
        }
        WM_RBUTTONDOWN => {
            event = WM_RBUTTONDOWN;
        }
        WM_RBUTTONUP => {
            down = false;
            event = WM_RBUTTONDOWN;
        }
        WM_MOUSEWHEEL => {
            event = WM_MOUSEWHEEL;
            if hook_struct.mouseData & 0x80000000 != 0 {
                extra = 1;
            }
        }
        WM_MOUSEHWHEEL => {
            event = WM_MOUSEHWHEEL;
            if hook_struct.mouseData & 0x80000000 != 0 {
                extra = 1;
            }
        }
        WM_XBUTTONDOWN => {
            event = WM_XBUTTONDOWN;
            if hook_struct.mouseData & 0x10000 != 0 {
                extra = 1;
            }
        }
        WM_XBUTTONUP => {
            down = false;
            event = WM_XBUTTONDOWN;
            if hook_struct.mouseData & 0x10000 != 0 {
                extra = 1;
            }
        }
        _ => (),
    }

    // set modifiers
    let mut modifiers = MOD_NOREPEAT;
    unsafe {
        if GetKeyState(VK_CONTROL.0 as i32) as u32 & 0x8000 != 0 {
            modifiers |= MOD_CONTROL;
        }
        if GetKeyState(VK_MENU.0 as i32) as u32 & 0x8000 != 0 {
            modifiers |= MOD_ALT;
        }
        if GetKeyState(VK_SHIFT.0 as i32) as u32 & 0x8000 != 0 {
            modifiers |= MOD_SHIFT;
        }
        if GetKeyState(VK_LWIN.0 as i32) as u32 & 0x8000 != 0 {
            modifiers |= MOD_WIN;
        }
        if GetKeyState(VK_RWIN.0 as i32) as u32 & 0x8000 != 0 {
            modifiers |= MOD_WIN;
        }
    }

    let button = Combination::Button {
        modifiers,
        button: event,
        extra,
    };

    // button shortcut
    for (idx, shortcut) in CONFIG.borrow().shortcuts.iter().enumerate() {
        if shortcut.combination == button {
            if down == true {
                PostThreadMessageA(GetCurrentThreadId(), WM_HOTKEY, WPARAM(idx), LPARAM(0));
            }
            return LRESULT(-1);
        }
    }

    CallNextHookEx(None, code, wparam, lparam)
}

#[cfg(debug_assertions)]
fn get_default_config_file() -> std::io::Result<File> {
    let file = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("doc")
        .join("default-config.yaml");
    File::open(file)
}

#[cfg(not(debug_assertions))]
fn get_default_config_file() -> std::io::Result<File> {
    let file = Path::new(env!("LOCALAPPDATA"))
        .join("windows-magnifier")
        .join("config.yaml");
    File::open(file)
}
