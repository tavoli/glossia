use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum ThemeMode {
    Light,
    Dark,
}

impl Default for ThemeMode {
    fn default() -> Self {
        ThemeMode::Light
    }
}

#[derive(Clone, PartialEq)]
pub struct Theme {
    pub mode: ThemeMode,
    pub background: &'static str,
    pub surface: &'static str,
    pub text_primary: &'static str,
    pub text_secondary: &'static str,
    pub border: &'static str,
    pub shadow: &'static str,
    pub accent: &'static str,
    pub error: &'static str,
    pub error_bg: &'static str,
    pub hover_bg: &'static str,
    pub gallery_bg: &'static str,
    pub gallery_border: &'static str,
}

impl Theme {
    pub fn light() -> Self {
        Self {
            mode: ThemeMode::Light,
            background: "#ffffff",
            surface: "#ffffff",
            text_primary: "#333333",
            text_secondary: "#495057",
            border: "#f1f3f4",
            shadow: "rgba(0,0,0,0.08)",
            accent: "#4a90e2",
            error: "#c62828",
            error_bg: "#ffebee",
            hover_bg: "#f8f9fa",
            gallery_bg: "transparent",
            gallery_border: "rgba(0,0,0,0.05)",
        }
    }

    pub fn dark() -> Self {
        Self {
            mode: ThemeMode::Dark,
            background: "#1a1a1a",
            surface: "#2d2d2d",
            text_primary: "#e0e0e0",
            text_secondary: "#b0b0b0",
            border: "#404040",
            shadow: "rgba(0,0,0,0.3)",
            accent: "#64b5f6",
            error: "#ef5350",
            error_bg: "#3d1a1a",
            hover_bg: "#3a3a3a",
            gallery_bg: "transparent",
            gallery_border: "rgba(255,255,255,0.05)",
        }
    }

    pub fn from_mode(mode: ThemeMode) -> Self {
        match mode {
            ThemeMode::Light => Self::light(),
            ThemeMode::Dark => Self::dark(),
        }
    }
}

pub fn use_theme() -> Signal<ThemeMode> {
    use_signal(|| ThemeMode::Light)
}

