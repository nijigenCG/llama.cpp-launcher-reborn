use crate::config::settings::AppSettings;
use crate::engine::server::{LogLevel, ServerManager};
use crate::i18n;
use crate::ui::theme;

pub fn ui(
    ui: &mut egui::Ui,
    settings: &mut AppSettings,
    server: &mut ServerManager,
    lang: &i18n::Language,
) {
    theme::page_frame().show(ui, |ui| {
        theme::page_title(ui, i18n::t(i18n::Key::PanelLogTitle, lang));

        let auto_scroll_before = settings.auto_scroll_logs;

        theme::section_card(ui, i18n::t(i18n::Key::PanelLogTitle, lang), |ui| {
            ui.horizontal_wrapped(|ui| {
                if ui
                    .add(theme::subtle_button(i18n::t(i18n::Key::BtnClearLogs, lang)))
                    .clicked()
                {
                    server.clear_logs();
                }
                ui.checkbox(
                    &mut settings.auto_scroll_logs,
                    i18n::t(i18n::Key::CheckboxAutoScroll, lang),
                );
                ui.label(i18n::t(i18n::Key::LabelMaxLogLines, lang));
                ui.add(egui::DragValue::new(&mut settings.max_log_lines).range(-1..=10000));
            });
            ui.small(
                egui::RichText::new(i18n::t(i18n::Key::HintLogSession, lang))
                    .color(theme::TEXT_MUTED),
            );

            let progress = server.progress();
            if progress > 0.0 {
                ui.add_space(10.0);
                let pct = (progress * 100.0).round() as u32;
                let label = format!(
                    "{}: {}/100%",
                    i18n::t(i18n::Key::LabelPreFillProgress, lang),
                    pct
                );
                ui.add(egui::ProgressBar::new(progress).text(label));
            }
        });

        ui.add_space(12.0);

        if settings.max_log_lines == 0 {
            return;
        }

        let should_scroll_to_bottom = !auto_scroll_before && settings.auto_scroll_logs;
        let limit = if settings.max_log_lines > 0 {
            Some(settings.max_log_lines as usize)
        } else {
            None
        };
        let logs = server.logs_tail(limit);

        theme::section_card(ui, i18n::t(i18n::Key::PanelLogTitle, lang), |ui| {
            theme::code_frame().show(ui, |ui| {
                egui::ScrollArea::vertical()
                    .id_salt("log_scroll_area")
                    .stick_to_bottom(settings.auto_scroll_logs)
                    .show(ui, |ui| {
                        if should_scroll_to_bottom {
                            ui.scroll_to_cursor(Some(egui::Align::BOTTOM));
                        }

                        if logs.is_empty() {
                            ui.add_space(6.0);
                            ui.colored_label(
                                theme::TEXT_MUTED,
                                i18n::t(i18n::Key::HintNoLogs, lang),
                            );
                            return;
                        }

                        for entry in &logs {
                            let color = match entry.level {
                                LogLevel::Info => Color32::from_rgb(233, 236, 242),
                                LogLevel::Warn => Color32::from_rgb(255, 215, 125),
                                LogLevel::Error => Color32::from_rgb(255, 154, 145),
                            };

                            ui.label(egui::RichText::new(&entry.text).monospace().color(color));
                        }
                    });
            });
        });
    });
}

use egui::Color32;
