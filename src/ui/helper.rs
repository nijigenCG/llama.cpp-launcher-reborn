use crate::ui::theme;
use egui::{Color32, RichText, Stroke, Vec2};

pub struct HelpButtonStyle {
    pub fill: Color32,
    pub stroke_color: Color32,
    pub corner_radius: f32,
    pub size: Vec2,
    pub text_size: f32,
}

impl Default for HelpButtonStyle {
    fn default() -> Self {
        Self {
            fill: theme::INFO,
            stroke_color: theme::INFO,
            corner_radius: 12.0,
            size: Vec2::new(20.0, 20.0),
            text_size: 12.0,
        }
    }
}

pub fn help_button(ui: &mut egui::Ui, help_text: &str) -> egui::Response {
    help_button_with_style(ui, help_text, &HelpButtonStyle::default())
}

pub fn help_button_with_style(
    ui: &mut egui::Ui,
    help_text: &str,
    style: &HelpButtonStyle,
) -> egui::Response {
    let visuals = ui.visuals_mut();
    let old_hover_expansion = visuals.widgets.hovered.expansion;
    let old_hover_bg_stroke = visuals.widgets.hovered.bg_stroke;
    let old_active_expansion = visuals.widgets.active.expansion;
    let old_active_bg_stroke = visuals.widgets.active.bg_stroke;

    visuals.widgets.hovered.expansion = 0.0;
    visuals.widgets.hovered.bg_stroke = Stroke::NONE;
    visuals.widgets.active.expansion = 0.0;
    visuals.widgets.active.bg_stroke = Stroke::NONE;

    let response = ui.add(
        egui::Button::new(
            RichText::new("?")
                .color(Color32::WHITE)
                .strong()
                .size(style.text_size),
        )
        .fill(style.fill)
        .corner_radius(style.corner_radius)
        .min_size(style.size)
        .stroke(Stroke::new(1.0, style.stroke_color))
        .sense(egui::Sense::hover()),
    );

    let visuals = ui.visuals_mut();
    visuals.widgets.hovered.expansion = old_hover_expansion;
    visuals.widgets.hovered.bg_stroke = old_hover_bg_stroke;
    visuals.widgets.active.expansion = old_active_expansion;
    visuals.widgets.active.bg_stroke = old_active_bg_stroke;

    response.on_hover_text(help_text)
}

pub fn help_button_small(ui: &mut egui::Ui, help_text: &str) -> egui::Response {
    let style = HelpButtonStyle {
        size: Vec2::new(16.0, 16.0),
        text_size: 10.0,
        corner_radius: 8.0,
        ..Default::default()
    };
    help_button_with_style(ui, help_text, &style)
}

pub fn help_button_inline(ui: &mut egui::Ui, help_text: &str) -> egui::Response {
    let style = HelpButtonStyle {
        size: Vec2::new(16.0, 16.0),
        text_size: 10.0,
        corner_radius: 8.0,
        fill: theme::TEXT_MUTED,
        stroke_color: theme::TEXT_MUTED,
    };
    help_button_with_style(ui, help_text, &style)
}
