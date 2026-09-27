//! Theme contract shared by GPUI applications.
//!
//! Host apps keep their own JSON palettes. The common shape allows the same
//! component to render with each app's colours and motion preferences.

use gpui::{Hsla, Rgba};
use serde::{Deserialize, Deserializer};

/// An application-supplied theme. Compatible with the current Ginka, e1 and
/// Kirikumo `assets/themes/*.json` files.
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ThemeTokens {
    /// Display name for settings and diagnostics.
    pub name: String,
    /// `dark` or `light`; the host remains responsible for OS appearance changes.
    pub appearance: String,
    /// Palette used by components.
    pub colors: Colors,
    /// Corner sizes in logical pixels.
    pub radius: Radii,
    /// Animation durations in milliseconds.
    pub duration_ms: Durations,
}

impl ThemeTokens {
    /// Parse one host-owned theme file, rejecting missing or misspelled tokens.
    pub fn parse(source: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(source)
    }
}

/// Shared semantic colours, rather than product-specific palette values.
#[derive(Debug, Clone, Copy, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Colors {
    /// Window background.
    #[serde(rename = "bg.window", deserialize_with = "hex")]
    pub bg_window: Hsla,
    /// Navigation background.
    #[serde(rename = "bg.sidebar", deserialize_with = "hex")]
    pub bg_sidebar: Hsla,
    /// Standard content surface.
    #[serde(rename = "bg.surface", deserialize_with = "hex")]
    pub bg_surface: Hsla,
    /// Elevated card surface.
    #[serde(rename = "bg.raised", deserialize_with = "hex")]
    pub bg_raised: Hsla,
    /// Terminal background.
    #[serde(rename = "bg.terminal", deserialize_with = "hex")]
    pub bg_terminal: Hsla,
    /// Subtle divider.
    #[serde(rename = "border.subtle", deserialize_with = "hex")]
    pub border_subtle: Hsla,
    /// Emphasized divider.
    #[serde(rename = "border.strong", deserialize_with = "hex")]
    pub border_strong: Hsla,
    /// Main text.
    #[serde(rename = "text.primary", deserialize_with = "hex")]
    pub text_primary: Hsla,
    /// Supporting text.
    #[serde(rename = "text.secondary", deserialize_with = "hex")]
    pub text_secondary: Hsla,
    /// Deemphasized text.
    #[serde(rename = "text.muted", deserialize_with = "hex")]
    pub text_muted: Hsla,
    /// Interactive emphasis.
    #[serde(deserialize_with = "hex")]
    pub accent: Hsla,
    /// In-progress state.
    #[serde(rename = "status.working", deserialize_with = "hex")]
    pub status_working: Hsla,
    /// State requiring attention.
    #[serde(rename = "status.attention", deserialize_with = "hex")]
    pub status_attention: Hsla,
    /// Successful state.
    #[serde(rename = "status.done", deserialize_with = "hex")]
    pub status_done: Hsla,
    /// Error state.
    #[serde(rename = "status.error", deserialize_with = "hex")]
    pub status_error: Hsla,
    /// Code surface.
    #[serde(rename = "code.bg", deserialize_with = "hex")]
    pub code_bg: Hsla,
}

/// Corner sizes in logical pixels.
#[derive(Debug, Clone, Copy, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Radii {
    /// Window corner.
    pub window: f32,
    /// Card corner.
    pub card: f32,
    /// Panel corner.
    pub panel: f32,
    /// Row corner.
    pub row: f32,
}

/// Motion durations in milliseconds.
#[derive(Debug, Clone, Copy, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Durations {
    /// Small feedback animation.
    pub quick: u64,
    /// Standard layout animation.
    pub standard: u64,
    /// Optional fade animation; older app themes omit this token.
    #[serde(default)]
    pub fade: Option<u64>,
}

fn hex<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Hsla, D::Error> {
    use serde::de::Error as _;
    let raw = String::deserialize(deserializer)?;
    parse_hex(&raw).map_err(D::Error::custom)
}

fn parse_hex(raw: &str) -> Result<Hsla, String> {
    let digits = raw.strip_prefix('#').unwrap_or(raw);
    let (rgb, alpha) = match digits.len() {
        6 => (digits, 0xff),
        8 => (
            &digits[..6],
            u8::from_str_radix(&digits[6..], 16).map_err(|error| error.to_string())?,
        ),
        _ => return Err(format!("expected #RRGGBB or #RRGGBBAA, got {raw:?}")),
    };
    let value = u32::from_str_radix(rgb, 16).map_err(|error| error.to_string())?;
    Ok(Rgba {
        r: ((value >> 16) & 0xff) as f32 / 255.0,
        g: ((value >> 8) & 0xff) as f32 / 255.0,
        b: (value & 0xff) as f32 / 255.0,
        a: alpha as f32 / 255.0,
    }
    .into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_invalid_colour() {
        assert!(parse_hex("#12").is_err());
        assert!(parse_hex("#gggggg").is_err());
    }

    #[test]
    fn keeps_colour_alpha() {
        let colour = parse_hex("#33669980").unwrap();
        assert!((colour.a - 128.0 / 255.0).abs() < 0.001);
    }
}
