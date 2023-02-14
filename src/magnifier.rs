use windows::Win32::Foundation::*;
use windows::Win32::System::Threading::GetCurrentThreadId;
use windows::Win32::UI::Magnification::*;
use windows::Win32::UI::WindowsAndMessaging::*;

use crate::config::*;

const DEAD_ZONE_FACTOR: f32 = 0.1;

pub struct Magnifier {
    factor: f32,
}

impl Magnifier {
    pub fn new() -> Self {
        Self { factor: 1.0 }
    }

    pub fn do_action(&mut self, action: &Action) -> Result<(), String> {
        match action {
            Action::Set(factor) => self.set(*factor),
            Action::Add(factor) => self.add(*factor),
            Action::Toggle(factor) => self.toggle(*factor),
            Action::Exit => unsafe {
                PostThreadMessageA(GetCurrentThreadId(), WM_QUIT, WPARAM(0), LPARAM(0))
                    .as_bool()
                    .then_some(())
                    .ok_or("failed to send exit message")?
            },
        };
        self.update()?;

        Ok(())
    }

    fn set(&mut self, factor: f32) {
        self.factor = factor;
    }

    fn add(&mut self, factor: f32) {
        self.factor += factor;
        if self.factor < 1.0 {
            self.factor = 1.0;
        }
    }

    fn toggle(&mut self, factor: f32) {
        if self.factor == 1.0 {
            self.factor = factor;
        } else {
            self.factor = 1.0;
        }
    }

    pub fn update(&self) -> Result<(), String> {
        let mut point = POINT::default();
        let width;
        let height;

        unsafe {
            // fetch cursor pos info
            if GetCursorPos(&mut point).as_bool() == false {
                Err("failed to get cursor position")?
            }

            // fetch screen info
            width = GetSystemMetrics(SM_CXSCREEN) as f32;
            height = GetSystemMetrics(SM_CYSCREEN) as f32;
        }

        // linear calculations
        let mul_x = width * (1.0 - 1.0 / self.factor) / ((1.0 - 2.0 * DEAD_ZONE_FACTOR) * width);
        let sub_x = mul_x * DEAD_ZONE_FACTOR * width;
        let max_x = width * (1.0 - 1.0 / self.factor);

        let mul_y = height * (1.0 - 1.0 / self.factor) / ((1.0 - 2.0 * DEAD_ZONE_FACTOR) * height);
        let sub_y = mul_y * DEAD_ZONE_FACTOR * height;
        let max_y = height * (1.0 - 1.0 / self.factor);

        let x = (point.x as f32 * mul_x - sub_x).min(max_x).max(0.0) as i32;
        let y = (point.y as f32 * mul_y - sub_y).min(max_y).max(0.0) as i32;

        // for input transform
        let dst = RECT {
            left: 0,
            top: 0,
            right: width as i32,
            bottom: height as i32,
        };
        let src = RECT {
            left: x,
            top: y,
            right: x + (width / self.factor) as i32,
            bottom: y + (height / self.factor) as i32,
        };

        unsafe {
            if MagSetFullscreenTransform(self.factor, x, y).as_bool() == false {
                Err("failed to magnify")?
            }
            // MagSetInputTransform requires uiAcess privileges
            #[cfg(not(debug_assertions))]
            if MagSetInputTransform(TRUE, &src, &dst).as_bool() == false {
                Err("failed to handle input transform")?
            }
        }

        Ok(())
    }
}
