use egui::{Button, Color32, Frame, Margin, RichText, Stroke, Ui, Visuals};

pub const APP_BG: Color32 = Color32::from_rgb(245, 239, 230);
pub const SURFACE_BG: Color32 = Color32::from_rgb(255, 251, 246);
pub const SURFACE_ALT: Color32 = Color32::from_rgb(238, 231, 221);
pub const SURFACE_DARK: Color32 = Color32::from_rgb(39, 43, 50);
pub const BORDER: Color32 = Color32::from_rgb(214, 201, 184);
pub const TEXT_MUTED: Color32 = Color32::from_rgb(109, 97, 83);
pub const ACCENT: Color32 = Color32::from_rgb(184, 96, 52);
pub const ACCENT_SOFT: Color32 = Color32::from_rgb(240, 214, 197);
pub const INFO: Color32 = Color32::from_rgb(71, 126, 153);
pub const SUCCESS: Color32 = Color32::from_rgb(81, 142, 104);
pub const DANGER: Color32 = Color32::from_rgb(179, 78, 73);
pub const WARNING: Color32 = Color32::from_rgb(196, 138, 53);

pub fn apply(ctx: &egui::Context) {
    let mut visuals = Visuals::light();
    visuals.panel_fill = APP_BG;
    visuals.window_fill = SURFACE_BG;
    visuals.extreme_bg_color = SURFACE_ALT;
    visuals.faint_bg_color = SURFACE_BG;
    visuals.widgets.noninteractive.bg_fill = SURFACE_BG;
    visuals.widgets.noninteractive.bg_stroke = Stroke::new(1.0, BORDER);
    visuals.widgets.inactive.bg_fill = SURFACE_BG;
    visuals.widgets.inactive.bg_stroke = Stroke::new(1.0, BORDER);
    visuals.widgets.hovered.bg_fill = ACCENT_SOFT;
    visuals.widgets.hovered.bg_stroke = Stroke::new(1.0, ACCENT);
    visuals.widgets.active.bg_fill = ACCENT;
    visuals.widgets.active.bg_stroke = Stroke::new(1.0, ACCENT);
    visuals.selection.bg_fill = ACCENT;
    visuals.selection.stroke = Stroke::new(1.0, Color32::WHITE);
    visuals.hyperlink_color = INFO;
    visuals.override_text_color = Some(Color32::from_rgb(42, 37, 33));
    ctx.set_visuals(visuals);

    let mut style = (*ctx.global_style()).clone();
    style.spacing.item_spacing = egui::vec2(12.0, 10.0);
    style.spacing.button_padding = egui::vec2(14.0, 9.0);
    style.spacing.menu_margin = Margin::same(10);
    style.spacing.indent = 18.0;
    style.spacing.text_edit_width = 320.0;
    style.spacing.slider_width = 220.0;
    style
        .text_styles
        .insert(egui::TextStyle::Heading, egui::FontId::proportional(24.0));
    style
        .text_styles
        .insert(egui::TextStyle::Body, egui::FontId::proportional(15.0));
    style
        .text_styles
        .insert(egui::TextStyle::Button, egui::FontId::proportional(15.0));
    style
        .text_styles
        .insert(egui::TextStyle::Small, egui::FontId::proportional(13.0));
    ctx.set_global_style(style);
}

pub fn chrome_frame() -> Frame {
    Frame::default()
        .fill(SURFACE_BG)
        .inner_margin(Margin::symmetric(20, 18))
        .outer_margin(Margin::symmetric(10, 10))
        .stroke(Stroke::new(1.0, BORDER))
        .corner_radius(22.0)
}

pub fn page_frame() -> Frame {
    Frame::default()
        .fill(SURFACE_BG)
        .inner_margin(Margin::same(18))
        .stroke(Stroke::new(1.0, BORDER))
        .corner_radius(20.0)
}

pub fn section_frame() -> Frame {
    Frame::default()
        .fill(Color32::from_rgb(252, 247, 240))
        .inner_margin(Margin::same(16))
        .stroke(Stroke::new(1.0, BORDER))
        .corner_radius(18.0)
}

pub fn code_frame() -> Frame {
    Frame::default()
        .fill(SURFACE_DARK)
        .inner_margin(Margin::same(14))
        .stroke(Stroke::new(1.0, Color32::from_rgb(66, 74, 86)))
        .corner_radius(16.0)
}

pub fn page_title(ui: &mut Ui, title: &str) {
    ui.label(RichText::new(title).size(26.0).strong().color(ACCENT));
    ui.add_space(10.0);
}

pub fn section_title(ui: &mut Ui, title: &str) {
    ui.label(RichText::new(title).size(18.0).strong());
    ui.add_space(8.0);
}

pub fn section_card<R>(ui: &mut Ui, title: &str, add_contents: impl FnOnce(&mut Ui) -> R) -> R {
    section_frame()
        .show(ui, |ui| {
            section_title(ui, title);
            add_contents(ui)
        })
        .inner
}

pub fn pill_button(ui: &mut Ui, selected: bool, text: &str) -> egui::Response {
    let fill = if selected { ACCENT } else { SURFACE_BG };
    let stroke = if selected {
        Stroke::new(1.0, ACCENT)
    } else {
        Stroke::new(1.0, BORDER)
    };
    let text_color = if selected {
        Color32::WHITE
    } else {
        Color32::from_rgb(61, 52, 45)
    };

    ui.add(
        Button::new(RichText::new(text).color(text_color).strong())
            .fill(fill)
            .stroke(stroke)
            .corner_radius(999.0)
            .min_size(egui::vec2(0.0, 34.0)),
    )
}

pub fn accent_button(text: &str, fill: Color32) -> Button<'static> {
    Button::new(RichText::new(text).color(Color32::WHITE).strong())
        .fill(fill)
        .stroke(Stroke::new(1.0, fill))
        .corner_radius(14.0)
        .min_size(egui::vec2(0.0, 36.0))
}

pub fn subtle_button(text: &str) -> Button<'static> {
    Button::new(RichText::new(text).strong())
        .fill(SURFACE_BG)
        .stroke(Stroke::new(1.0, BORDER))
        .corner_radius(14.0)
        .min_size(egui::vec2(0.0, 36.0))
}

pub fn status_badge(ui: &mut Ui, label: &str, value: &str, color: Color32) {
    Frame::default()
        .fill(Color32::from_rgb(250, 245, 238))
        .inner_margin(Margin::symmetric(12, 10))
        .stroke(Stroke::new(1.0, BORDER))
        .corner_radius(16.0)
        .show(ui, |ui| {
            ui.vertical(|ui| {
                ui.label(RichText::new(label).small().color(TEXT_MUTED));
                ui.label(RichText::new(value).strong().color(color));
            });
        });
}

pub fn tag_chip(ui: &mut Ui, text: &str, color: Color32) {
    let button = Button::new(RichText::new(text).color(Color32::WHITE).small())
        .fill(color)
        .stroke(Stroke::NONE)
        .corner_radius(999.0)
        .sense(egui::Sense::hover());
    ui.add(button);
}
