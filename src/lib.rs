//! Small, app-independent GPUI building blocks.
//!
//! The host app owns its window, content, actions, translations and theme values.
//! This crate owns the shared token shape and visual primitives only.

pub mod components;
pub mod theme;

pub use components::{Status, section_label, status_badge, surface_card};
pub use theme::{Colors, Durations, Radii, ThemeTokens};
