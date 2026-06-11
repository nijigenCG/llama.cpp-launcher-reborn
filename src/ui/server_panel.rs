use crate::config::settings::{is_server_binary_name, AppSettings, SettingsManager};
use crate::i18n;
use crate::ui::theme;
#[cfg(target_os = "windows")]
use std::os::windows::process::CommandExt;

pub fn ui(
    ui: &mut egui::Ui,
    settings: &mut AppSettings,
    settings_manager: &SettingsManager,
    lang: &i18n::Language,
) {
    theme::page_frame().show(ui, |ui| {
        theme::page_title(ui, i18n::t(i18n::Key::PanelServerTitle, lang));

        theme::section_card(ui, i18n::t(i18n::Key::LabelServerPath, lang), |ui| {
            let mut server_path_str = settings.server_path.to_string_lossy().to_string();
            if ui.text_edit_singleline(&mut server_path_str).changed() {
                settings.server_path = std::path::PathBuf::from(&server_path_str);
            }

            ui.add_space(8.0);
            ui.horizontal_wrapped(|ui| {
                if ui
                    .add(theme::subtle_button(i18n::t(i18n::Key::BtnBrowse, lang)))
                    .clicked()
                {
                    if let Some(path) = rfd::FileDialog::new()
                        .set_title(i18n::t(i18n::Key::DialogSelectServer, lang))
                        .add_filter(i18n::t(i18n::Key::FilterExecutable, lang), &["exe"])
                        .pick_file()
                    {
                        settings.server_path = path;
                    }
                }

                if ui
                    .add(theme::accent_button(
                        i18n::t(i18n::Key::BtnAutoDetect, lang),
                        theme::INFO,
                    ))
                    .clicked()
                {
                    settings.server_path = settings_manager
                        .auto_detect_server_path()
                        .unwrap_or_default();
                }

                let server_path_valid = settings
                    .server_path
                    .file_name()
                    .and_then(|f| f.to_str())
                    .is_some_and(is_server_binary_name)
                    && settings.server_path.exists();

                if ui
                    .add_enabled(
                        server_path_valid,
                        theme::subtle_button(i18n::t(i18n::Key::BtnCheckVersion, lang)),
                    )
                    .clicked()
                {
                    let mut cmd = std::process::Command::new(&settings.server_path);
                    cmd.arg("--version")
                        .stdout(std::process::Stdio::piped())
                        .stderr(std::process::Stdio::piped());
                    #[cfg(target_os = "windows")]
                    cmd.creation_flags(0x0800_0000u32);
                    match cmd.output() {
                        Ok(output) => {
                            let stdout = String::from_utf8_lossy(&output.stdout);
                            let stderr = String::from_utf8_lossy(&output.stderr);
                            settings.llama_version = stdout
                                .lines()
                                .chain(stderr.lines())
                                .find(|line| line.contains("version:"))
                                .and_then(|line| {
                                    line.split_once("version:")
                                        .map(|(_, value)| value.trim().to_string())
                                })
                                .unwrap_or_else(|| {
                                    i18n::t(i18n::Key::TextUnknownVersion, lang).to_string()
                                });
                        }
                        Err(error) => {
                            settings.llama_version =
                                format!("{}: {}", i18n::t(i18n::Key::TextFetchFailed, lang), error);
                        }
                    }
                }
            });

            if !settings.llama_version.is_empty() {
                ui.add_space(6.0);
                ui.small(egui::RichText::new(&settings.llama_version).color(theme::TEXT_MUTED));
            }
        });

        ui.add_space(12.0);

        theme::section_card(ui, i18n::t(i18n::Key::LabelHost, lang), |ui| {
            ui.horizontal_wrapped(|ui| {
                ui.label(i18n::t(i18n::Key::LabelHost, lang));
                ui.text_edit_singleline(&mut settings.host);
                ui.label(i18n::t(i18n::Key::LabelPort, lang));
                ui.add(egui::DragValue::new(&mut settings.port).range(1..=65535));
            });

            ui.add_space(8.0);
            ui.horizontal_wrapped(|ui| {
                if ui
                    .add(theme::subtle_button(i18n::t(i18n::Key::BtnHostLocal, lang)))
                    .clicked()
                {
                    settings.host = "127.0.0.1".to_string();
                }
                if ui
                    .add(theme::subtle_button(i18n::t(i18n::Key::BtnHostAny, lang)))
                    .clicked()
                {
                    settings.host = "0.0.0.0".to_string();
                }
            });
        });

        ui.add_space(12.0);

        theme::section_card(ui, i18n::t(i18n::Key::PanelServerTitle, lang), |ui| {
            ui.horizontal_wrapped(|ui| {
                ui.label(i18n::t(i18n::Key::LabelParallelSlots, lang));
                ui.add(egui::DragValue::new(&mut settings.parallel_slots).range(1..=32));
            });

            ui.horizontal_wrapped(|ui| {
                ui.label(i18n::t(i18n::Key::LabelAlias, lang));
                ui.text_edit_singleline(&mut settings.alias);
            });

            ui.separator();

            ui.checkbox(
                &mut settings.verbose,
                i18n::t(i18n::Key::CheckboxVerbose, lang),
            );
            ui.checkbox(
                &mut settings.offline_mode,
                i18n::t(i18n::Key::CheckboxOfflineMode, lang),
            );
            ui.checkbox(
                &mut settings.rpc_mode,
                i18n::t(i18n::Key::CheckboxRpcMode, lang),
            );

            if settings.rpc_mode {
                ui.indent("rpc_endpoints", |ui| {
                    ui.horizontal_wrapped(|ui| {
                        ui.label(i18n::t(i18n::Key::LabelRpcEndpoints, lang));
                        ui.text_edit_singleline(&mut settings.rpc_endpoints);
                    });
                    ui.small(
                        egui::RichText::new(i18n::t(i18n::Key::HintRpcEndpoints, lang))
                            .color(theme::TEXT_MUTED),
                    );
                });
            }

            ui.checkbox(
                &mut settings.web_ui_enabled,
                i18n::t(i18n::Key::CheckboxEnableWebClient, lang),
            );
        });
    });
}
