use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use toml;

use crossterm::style::Color;
use dirs::home_dir;

#[derive(Serialize, Deserialize)]
struct ColorTable {
    fg: Option<String>,
    bg: Option<String>,
    typed: Option<String>,
    missing: Option<String>,
    error: Option<String>,
    accent: Option<String>,
}

#[derive(Serialize, Deserialize)]
struct ConfigToml {
    colors: Option<ColorTable>,
}

impl Default for ConfigToml {
    fn default() -> Self {
        ConfigToml { colors: None }
    }
}

#[derive(Debug)]
pub struct ThemeColors {
    pub fg: Color,
    pub bg: Option<Color>,
    pub typed: Color,
    pub missing: Color,
    pub error: Color,
    pub accent: Color,
}

impl ThemeColors {
    pub fn new() -> Self {
        let mut config_filepaths: Vec<PathBuf> = vec![PathBuf::from("./config.toml")];

        if let Some(home_path) = home_dir() {
            config_filepaths.push(home_path.join(".config/typy/config.toml"));
        }

        let mut content = "".to_owned();

        for filepath in config_filepaths {
            let result = fs::read_to_string(filepath);

            if result.is_ok() {
                content = result.unwrap();
                break;
            }
        }

        let config_toml: ConfigToml =
            toml::from_str(&content).unwrap_or_else(|_| ConfigToml::default());

        let theme_colors: ThemeColors = match config_toml.colors {
            Some(colors) => {
                let fg = colors
                    .fg
                    .and_then(|c| hex_to_rgb(&c))
                    .unwrap_or(Color::White);
                let bg = colors
                    .bg
                    .and_then(|c| hex_to_rgb(&c));
                let typed = colors
                    .typed
                    .and_then(|c| hex_to_rgb(&c))
                    .unwrap_or(Color::Green);
                let missing = colors
                    .missing
                    .and_then(|c| hex_to_rgb(&c))
                    .unwrap_or(Color::Grey);
                let error = colors
                    .error
                    .and_then(|c| hex_to_rgb(&c))
                    .unwrap_or(Color::Red);
                let accent = colors
                    .accent
                    .and_then(|c| hex_to_rgb(&c))
                    .unwrap_or(Color::Yellow);

                ThemeColors {
                    fg,
                    bg,
                    typed,
                    missing,
                    error,
                    accent,
                }
            }
            None => ThemeColors::default(),
        };
        theme_colors
    }
}

impl Default for ThemeColors {
    fn default() -> Self {
        ThemeColors {
            fg: Color::White,
            bg: None,
            typed: Color::Green,
            missing: Color::Grey,
            error: Color::Red,
            accent: Color::Yellow,
        }
    }
}

fn hex_to_rgb(hex: &str) -> Option<Color> {
    if hex.len() == 7 && hex.starts_with('#') {
        let r = u8::from_str_radix(&hex[1..3], 16).ok()?;
        let g = u8::from_str_radix(&hex[3..5], 16).ok()?;
        let b = u8::from_str_radix(&hex[5..7], 16).ok()?;
        Some(Color::Rgb { r, g, b })
    } else {
        None
    }
}


#[cfg(test)]
mod theme_tests {
    use super::*;

    #[test]
    fn test_hex_to_rgb() {
        assert_eq!(hex_to_rgb("#ffffff"), Some(Color::Rgb { r: 255, g: 255, b: 255 }));
        assert_eq!(hex_to_rgb("#000000"), Some(Color::Rgb { r: 0, g: 0, b: 0 }));
        assert_eq!(hex_to_rgb("#ff0000"), Some(Color::Rgb { r: 255, g: 0, b: 0 }));
        assert_eq!(hex_to_rgb("#00ff00"), Some(Color::Rgb { r: 0, g: 255, b: 0 }));
        assert_eq!(hex_to_rgb("#0000ff"), Some(Color::Rgb { r: 0, g: 0, b: 255 }));
        assert_eq!(hex_to_rgb("#123456"), Some(Color::Rgb { r: 18, g: 52, b: 86 }));
        assert_eq!(hex_to_rgb("#abcdef"), Some(Color::Rgb { r: 171, g: 205, b: 239 }));
        assert_eq!(hex_to_rgb("#12345"), None);
        assert_eq!(hex_to_rgb("#1234567"), None);
        assert_eq!(hex_to_rgb("123456"), None);
        assert_eq!(hex_to_rgb("#12345g"), None);
    }
}
