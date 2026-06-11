use crate::config::settings::{AppSettings, SettingsManager};
use crate::i18n;
use crate::ui::theme;

pub fn ui(
    ui: &mut egui::Ui,
    settings: &mut AppSettings,
    settings_manager: &SettingsManager,
    lang: &i18n::Language,
) {
    theme::page_frame().show(ui, |ui| {
        theme::page_title(ui, i18n::t(i18n::Key::PanelRpcTitle, lang));

        theme::section_card(ui, i18n::t(i18n::Key::LabelRpcPath, lang), |ui| {
            let mut rpc_path_str = settings.rpc_server_path.to_string_lossy().to_string();
            if ui.text_edit_singleline(&mut rpc_path_str).changed() {
                settings.rpc_server_path = std::path::PathBuf::from(&rpc_path_str);
            }

            ui.add_space(8.0);
            ui.horizontal_wrapped(|ui| {
                if ui
                    .add(theme::subtle_button(i18n::t(i18n::Key::BtnBrowse, lang)))
                    .clicked()
                {
                    if let Some(path) = rfd::FileDialog::new()
                        .set_title(i18n::t(i18n::Key::DialogSelectRpc, lang))
                        .add_filter(i18n::t(i18n::Key::FilterExecutable, lang), &["exe"])
                        .pick_file()
                    {
                        settings.rpc_server_path = path;
                    }
                }

                if ui
                    .add(theme::accent_button(
                        i18n::t(i18n::Key::BtnAutoDetect, lang),
                        theme::INFO,
                    ))
                    .clicked()
                {
                    settings.rpc_server_path =
                        settings_manager.auto_detect_rpc_path().unwrap_or_default();
                }
            });
        });

        ui.add_space(12.0);

        theme::section_card(ui, i18n::t(i18n::Key::LabelHost, lang), |ui| {
            ui.horizontal_wrapped(|ui| {
                ui.label(i18n::t(i18n::Key::LabelHost, lang));
                ui.text_edit_singleline(&mut settings.rpc_host);
                ui.label(i18n::t(i18n::Key::LabelPort, lang));
                ui.add(egui::DragValue::new(&mut settings.rpc_port).range(1..=65535));
            });

            ui.add_space(8.0);
            ui.horizontal_wrapped(|ui| {
                if ui
                    .add(theme::subtle_button(i18n::t(i18n::Key::BtnHostLocal, lang)))
                    .clicked()
                {
                    settings.rpc_host = "127.0.0.1".to_string();
                }
                if ui
                    .add(theme::subtle_button(i18n::t(i18n::Key::BtnHostAny, lang)))
                    .clicked()
                {
                    settings.rpc_host = "0.0.0.0".to_string();
                }
            });
        });

        ui.add_space(12.0);

        theme::section_card(ui, i18n::t(i18n::Key::PanelRpcTitle, lang), |ui| {
            ui.horizontal_wrapped(|ui| {
                ui.label(i18n::t(i18n::Key::LabelRpcThreads, lang));
                ui.add(egui::DragValue::new(&mut settings.rpc_threads).range(1..=128));
                ui.small(
                    egui::RichText::new(i18n::t(i18n::Key::HintRpcThreads, lang))
                        .color(theme::TEXT_MUTED),
                );
            });

            ui.horizontal_wrapped(|ui| {
                ui.label(i18n::t(i18n::Key::LabelRpcDevice, lang));
                ui.text_edit_singleline(&mut settings.rpc_device);
                ui.small(
                    egui::RichText::new(i18n::t(i18n::Key::HintRpcDevice, lang))
                        .color(theme::TEXT_MUTED),
                );
            });

            ui.checkbox(
                &mut settings.rpc_cache,
                i18n::t(i18n::Key::CheckboxRpcCache, lang),
            );
        });
    });
}
