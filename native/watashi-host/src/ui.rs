//! KAMI-powered configuration UI for screen layout.
//!
//! Renders a wgpu window showing the screen arrangement.
//! Uses kami-ui-gpu for Nintendo-style rendering via magatama-kami-host.

use anyhow::Result;
use magatama_kami_host::{Color, KamiScene, PanelNode, Rect, TextNode};

/// Auto-incrementing ID generator for UI nodes.
fn uid(prefix: &str, idx: usize) -> String {
    format!("{prefix}-{idx}")
}

/// Nintendo-style color palette.
mod palette {
    use super::Color;
    pub const TEAL: Color = Color([0.051, 0.420, 0.365, 1.0]);
    pub const TEAL_LIGHT: Color = Color([0.2, 0.6, 0.5, 1.0]);
    pub const GRAY: Color = Color([0.6, 0.6, 0.6, 1.0]);
    pub const WHITE: Color = Color([1.0, 1.0, 1.0, 1.0]);
    pub const WHITE_ALPHA: Color = Color([1.0, 1.0, 1.0, 0.5]);
    pub const DARK: Color = Color([0.2, 0.2, 0.2, 1.0]);
    pub const PINK: Color = Color([0.957, 0.502, 0.639, 1.0]);
}

/// Screen representation in the config UI.
struct UiScreen {
    name: String,
    width: u32,
    height: u32,
    is_local: bool,
    is_connected: bool,
}

/// Build the KAMI scene for the configuration UI.
pub fn build_config_scene(
    window_width: f32,
    window_height: f32,
    screens: &[UiScreen],
) -> KamiScene {
    let mut panels = Vec::new();
    let mut text = Vec::new();
    let mut pid = 0usize;
    let mut tid = 0usize;

    // Title bar
    panels.push(PanelNode {
        id: uid("p", { pid += 1; pid }),
        rect: Rect { x: 0.0, y: 0.0, width: window_width, height: 48.0 },
        fill: palette::TEAL,
        border: None,
        border_width: 0.0,
        radius: 0.0,
    });

    text.push(TextNode {
        id: uid("t", { tid += 1; tid }),
        content: "Watashi — Screen Layout".into(),
        x: 16.0, y: 14.0, size: 20.0,
        color: palette::WHITE,
    });

    // Screen layout area
    let layout_y = 64.0;
    let layout_h = window_height - 128.0;
    let layout_w = window_width - 32.0;

    panels.push(PanelNode {
        id: uid("p", { pid += 1; pid }),
        rect: Rect { x: 16.0, y: layout_y, width: layout_w, height: layout_h },
        fill: Color([0.95, 0.93, 0.88, 1.0]),
        border: Some(palette::WHITE_ALPHA),
        border_width: 2.0,
        radius: 16.0,
    });

    // Render screen rectangles
    let scale = compute_scale(screens, layout_w - 32.0, layout_h - 32.0);
    let base_x = 32.0;
    let base_y = layout_y + 16.0;

    let mut offset_x = 0.0;
    for screen in screens {
        let w = screen.width as f32 * scale;
        let h = screen.height as f32 * scale;

        let fill = if screen.is_local {
            palette::TEAL_LIGHT
        } else if screen.is_connected {
            palette::PINK
        } else {
            palette::GRAY
        };

        panels.push(PanelNode {
            id: uid("p", { pid += 1; pid }),
            rect: Rect {
                x: base_x + offset_x,
                y: base_y + (layout_h - 32.0 - h) / 2.0,
                width: w, height: h,
            },
            fill,
            border: Some(palette::WHITE),
            border_width: 2.0,
            radius: 8.0,
        });

        text.push(TextNode {
            id: uid("t", { tid += 1; tid }),
            content: screen.name.clone(),
            x: base_x + offset_x + 8.0,
            y: base_y + (layout_h - 32.0 - h) / 2.0 + 8.0,
            size: 14.0,
            color: palette::WHITE,
        });

        text.push(TextNode {
            id: uid("t", { tid += 1; tid }),
            content: format!("{}x{}", screen.width, screen.height),
            x: base_x + offset_x + 8.0,
            y: base_y + (layout_h - 32.0 - h) / 2.0 + 28.0,
            size: 11.0,
            color: Color([1.0, 1.0, 1.0, 0.7]),
        });

        offset_x += w + 16.0;
    }

    // Status bar
    let status_y = window_height - 48.0;
    panels.push(PanelNode {
        id: uid("p", { pid += 1; pid }),
        rect: Rect { x: 0.0, y: status_y, width: window_width, height: 48.0 },
        fill: Color([0.95, 0.93, 0.88, 1.0]),
        border: Some(palette::WHITE_ALPHA),
        border_width: 1.0,
        radius: 0.0,
    });

    text.push(TextNode {
        id: uid("t", { tid += 1; tid }),
        content: "Drag screens to arrange. Press ESC to close.".into(),
        x: 16.0, y: status_y + 14.0, size: 14.0,
        color: palette::DARK,
    });

    KamiScene { panels, text, meters: vec![] }
}

/// Compute scale factor to fit all screens into the layout area.
fn compute_scale(screens: &[UiScreen], max_width: f32, max_height: f32) -> f32 {
    if screens.is_empty() {
        return 1.0;
    }
    let total_w: u32 = screens.iter().map(|s| s.width).sum::<u32>()
        + (screens.len().saturating_sub(1) as u32) * 16;
    let max_h: u32 = screens.iter().map(|s| s.height).max().unwrap_or(1080);
    let sx = max_width / total_w as f32;
    let sy = max_height / max_h as f32;
    sx.min(sy).min(1.0)
}

/// Run the configuration UI window.
pub fn run_ui() -> Result<()> {
    // Use the sample window from magatama-kami-host as starting point.
    // Future: custom winit window with screen drag-and-drop.
    magatama_kami_host::runtime::run_sample_window()?;
    Ok(())
}
