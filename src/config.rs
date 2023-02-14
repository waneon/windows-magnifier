use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::time::Instant;
use windows::Win32::UI::Input::KeyboardAndMouse::*;
use windows::Win32::UI::WindowsAndMessaging::*;

#[derive(Serialize, Deserialize)]
struct RawConfig {
    shortcut: BTreeMap<String, RawShortcut>,
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "action")]
enum RawShortcut {
    #[serde(rename = "set")]
    Set {
        factor: f32,
        #[serde(default)]
        cooltime: u32,
    },
    #[serde(rename = "add")]
    Add {
        factor: f32,
        #[serde(default)]
        cooltime: u32,
    },
    #[serde(rename = "toggle")]
    Toggle {
        factor: f32,
        #[serde(default)]
        cooltime: u32,
    },
    #[serde(rename = "exit")]
    Exit {
        #[serde(default)]
        cooltime: u32,
    },
}

#[derive(Debug, Default)]
pub struct Config {
    pub shortcuts: Vec<Shortcut>,
}

#[derive(Debug)]
pub struct Shortcut {
    pub combination: Combination,
    pub action: Action,
    pub last_run: Instant,
    pub cooltime: u128,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Combination {
    Key {
        modifiers: HOT_KEY_MODIFIERS,
        key: u32,
    },
    Button {
        modifiers: HOT_KEY_MODIFIERS,
        button: u32,
        extra: i32,
    },
}

#[derive(Debug)]
pub enum Action {
    Set(f32),
    Add(f32),
    Toggle(f32),
    Exit,
}

impl Config {
    pub fn new<R>(rdr: R) -> Result<Self, Box<dyn std::error::Error>>
    where
        R: std::io::Read,
    {
        let raw_config: RawConfig = serde_yaml::from_reader(rdr)?;
        let mut config = Self { shortcuts: vec![] };

        for (c, s) in raw_config.shortcut {
            let seq: Vec<&str> = c.split('-').collect();
            // if there's no sequences
            if seq.len() == 0 {
                Err(format!("invalid combination: {}", c))?;
            }

            let mut modifiers = MOD_NOREPEAT;
            // set modifiers
            for &m in &seq[..seq.len() - 1] {
                modifiers |= match m {
                    "C" => MOD_CONTROL,
                    "M" => MOD_ALT,
                    "S" => MOD_SHIFT,
                    "W" => MOD_WIN,
                    _ => Err(format!("invalid modifiers: {}", c))?,
                };
            }

            // set combination
            let last_str = seq[seq.len() - 1];
            let last: Vec<char> = last_str.chars().collect();
            let combination = if last.len() == 1 && last[0] >= '0' && last[0] <= '9' {
                // 0-9
                Combination::Key {
                    modifiers,
                    key: last[0] as u32 - '0' as u32 + 0x30,
                }
            } else if last.len() == 1 && last[0] >= 'A' && last[0] <= 'Z' {
                // A-Z
                Combination::Key {
                    modifiers,
                    key: last[0] as u32 - 'A' as u32 + 0x41,
                }
            } else if last.len() == 2 && last[0] == 'F' && last[1] >= '1' && last[1] <= '9' {
                // F1-F9
                Combination::Key {
                    modifiers,
                    key: last[1] as u32 - '1' as u32 + 0x70,
                }
            } else if last.len() == 3 && last[0] == 'F' && last[1] == '1' {
                // F10-F12
                Combination::Key {
                    modifiers,
                    key: match last[2] {
                        '0' => 0x79,
                        '1' => 0x7A,
                        '2' => 0x7B,
                        _ => Err(format!("invalid key: {}", c))?,
                    },
                }
            } else {
                // mouse button
                Combination::Button {
                    modifiers,
                    button: match last_str {
                        "Left" => WM_LBUTTONDOWN,
                        "Right" => WM_RBUTTONDOWN,
                        "Middle" => WM_MBUTTONDOWN,
                        "WheelUp" | "WheelDown" => WM_MOUSEWHEEL,
                        "WheelRight" | "WheelLeft" => WM_MOUSEHWHEEL,
                        "Side1" | "Side2" => WM_XBUTTONDOWN,
                        _ => Err(format!("invalid button: {}", c))?,
                    },
                    extra: match last_str {
                        "WheelDown" | "WheelLeft" | "Side2" => 1,
                        _ => 0,
                    },
                }
            };

            // set action
            let (action, cooltime) = match s {
                RawShortcut::Set { factor, cooltime } => {
                    if factor < 1.0 {
                        Err(format!("set-factor must not be smaller than 1.0: {}", c))?
                    }
                    (Action::Set(factor), cooltime)
                }
                RawShortcut::Add { factor, cooltime } => (Action::Add(factor), cooltime),
                RawShortcut::Toggle { factor, cooltime } => {
                    if factor < 1.0 {
                        Err(format!("toggle-factor must not be smaller than 1.0: {}", c))?
                    }
                    (Action::Toggle(factor), cooltime)
                }
                RawShortcut::Exit { cooltime } => (Action::Exit, cooltime),
            };

            config.shortcuts.push(Shortcut {
                combination,
                action,
                last_run: std::time::Instant::now(),
                cooltime: cooltime as u128,
            });
        }

        Ok(config)
    }
}
