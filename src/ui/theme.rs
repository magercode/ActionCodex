use ratatui::style::Color;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThemeMode {
    Dark,
    Light,
}

#[derive(Debug, Clone, Copy)]
pub struct ThemePalette {
    pub accent: Color,
    pub accent_text: Color,
    pub warning: Color,
    pub warning_text: Color,
    pub tree_inactive_bg: Color,
    pub status_fg: Color,
    pub status_bg: Color,
    pub selection_bg: Color,
    pub search_bg: Color,
    pub line_current: Color,
    pub line_other: Color,
}

impl ThemeMode {
    pub fn toggle(self) -> Self {
        match self {
            Self::Dark => Self::Light,
            Self::Light => Self::Dark,
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Self::Dark => "Gelap",
            Self::Light => "Terang",
        }
    }

    pub fn palette(self) -> ThemePalette {
        match self {
            Self::Dark => ThemePalette {
                accent: Color::Cyan,
                accent_text: Color::Black,
                warning: Color::Yellow,
                warning_text: Color::Black,
                tree_inactive_bg: Color::Rgb(45, 45, 45),
                status_fg: Color::Black,
                status_bg: Color::White,
                selection_bg: Color::Rgb(70, 90, 130),
                search_bg: Color::Rgb(120, 95, 40),
                line_current: Color::Yellow,
                line_other: Color::DarkGray,
            },
            Self::Light => ThemePalette {
                accent: Color::Blue,
                accent_text: Color::White,
                warning: Color::Yellow,
                warning_text: Color::Black,
                tree_inactive_bg: Color::Rgb(232, 232, 232),
                status_fg: Color::White,
                status_bg: Color::Rgb(45, 45, 45),
                selection_bg: Color::Rgb(173, 201, 255),
                search_bg: Color::Rgb(255, 228, 153),
                line_current: Color::Blue,
                line_other: Color::Gray,
            },
        }
    }
}
