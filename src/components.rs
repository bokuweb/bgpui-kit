//! Reusable presentation primitives without application state or actions.

use crate::ThemeTokens;
use gpui::{Div, Hsla, ParentElement as _, Styled as _, div, px};
use gpui_component::h_flex;

/// Semantic state for a compact status badge.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    /// Work is in progress.
    Working,
    /// A person needs to respond.
    Attention,
    /// Work finished successfully.
    Done,
    /// Work failed.
    Error,
}

impl Status {
    fn color(self, tokens: &ThemeTokens) -> Hsla {
        match self {
            Self::Working => tokens.colors.status_working,
            Self::Attention => tokens.colors.status_attention,
            Self::Done => tokens.colors.status_done,
            Self::Error => tokens.colors.status_error,
        }
    }
}

/// A compact status indicator with both colour and caller-provided text.
/// The host supplies translated text; colour alone never carries meaning.
pub fn status_badge(label: impl Into<String>, status: Status, tokens: &ThemeTokens) -> Div {
    let color = status.color(tokens);
    h_flex()
        .items_center()
        .gap_1p5()
        .child(div().size_1p5().rounded_full().bg(color))
        .child(div().text_xs().text_color(color).child(label.into()))
}

/// A small heading above a related group of rows.
pub fn section_label(label: impl Into<String>, tokens: &ThemeTokens) -> Div {
    div()
        .w_full()
        .px_3()
        .pt_3()
        .pb_1()
        .text_xs()
        .text_color(tokens.colors.text_muted)
        .child(label.into())
}

/// A themed panel container; the caller owns its content and behaviour.
pub fn surface_card(tokens: &ThemeTokens) -> Div {
    div()
        .bg(tokens.colors.bg_raised)
        .border_1()
        .border_color(tokens.colors.border_subtle)
        .rounded(px(tokens.radius.card))
        .p_3()
}
