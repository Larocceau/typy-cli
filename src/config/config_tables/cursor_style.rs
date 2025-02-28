use crossterm::cursor::SetCursorStyle;

use crate::config::toml_parser::get_config;
use crate::config::toml_parser::CursorTable;

pub struct CursorKind {
    pub style: SetCursorStyle,
}

impl CursorKind {
    pub fn new() -> Self {
        let cursor_table: CursorTable =
            get_config()
                .lock()
                .unwrap()
                .get_cursor()
                .unwrap_or(CursorTable {
                    style: Some("DefaultUserShape".to_owned()),
                });

        let cursor_kind = match cursor_table.style.as_deref() {
            Some("DefaultUserShape") => SetCursorStyle::DefaultUserShape,
            Some("BlinkingBlock") => SetCursorStyle::BlinkingBlock,
            Some("SteadyBlock") => SetCursorStyle::SteadyBlock,
            Some("BlinkingUnderScore") => SetCursorStyle::BlinkingUnderScore,
            Some("SteadyUnderScore") => SetCursorStyle::SteadyUnderScore,
            Some("BlinkingBar") => SetCursorStyle::BlinkingBar,
            Some("SteadyBar") => SetCursorStyle::SteadyBar,
            _ => SetCursorStyle::DefaultUserShape,
        };

        CursorKind { style: cursor_kind }
    }
}

impl Default for CursorKind {
    fn default() -> Self {
        CursorKind {
            style: SetCursorStyle::DefaultUserShape,
        }
    }
}
