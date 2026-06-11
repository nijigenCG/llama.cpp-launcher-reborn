use crate::config::settings::{is_server_binary_name, AppSettings, GpuLayersMode};
use crate::i18n;
use crate::kv_cache;
use crate::ui::{helper, theme};

pub fn ui(ui: &mut egui::Ui, settings: &mut AppSettings, lang: &i18n::Language) {
    let server_path_valid = settings
        .server_path
        .file_name()
        .and_then(|f| f.to_str())
        .is_some_and(is_server_binary_name);
    let can_start = server_path_valid && !settings.model_path.as_os_str().is_empty();

    theme::page_frame().show(ui, |ui| {
        theme::page_title(ui, i18n::t(i18n::Key::PanelParamsTitle, lang));

        theme::section_card(ui, i18n::t(i18n::Key::PanelParamsTitle, lang), |ui| {
            ui.horizontal_wrapped(|ui| {
                ui.label(i18n::t(i18n::Key::LabelNCtx, lang));
                ui.add(
                    egui::DragValue::new(&mut settings.context)
                        .range(1..=1024)
                        .speed(1),
                );
                ui.label("k");
                ui.small(i18n::t(i18n::Key::HintKUnit, lang));
                helper::help_button_inline(ui, i18n::t(i18n::Key::HelpNCtx, lang));
            });

            if ui
                .add_enabled(
                    can_start,
                    theme::accent_button(
                        i18n::t(i18n::Key::BtnSetMaxContextVram, lang),
                        theme::INFO,
                    ),
                )
                .clicked()
            {
                match kv_cache::calc_max_context_facade(settings) {
                    Ok(value) => settings.context = value,
                    Err(error) => log::warn!("[params_panel] calc_max_context failed: {}", error),
                }
            }

            ui.horizontal_wrapped(|ui| {
                ui.label(i18n::t(i18n::Key::LabelBatchSize, lang));
                ui.add(
                    egui::DragValue::new(&mut settings.batch_size)
                        .range(1..=16)
                        .speed(1),
                );
                ui.label("k");
                ui.small(i18n::t(i18n::Key::HintKUnit, lang));
                helper::help_button_inline(ui, i18n::t(i18n::Key::HelpBatchSize, lang));
            });

            ui.horizontal_wrapped(|ui| {
                ui.label(i18n::t(i18n::Key::LabelUBatchSize, lang));
                ui.add(
                    egui::DragValue::new(&mut settings.ubatch_size)
                        .range(0.5..=16.0)
                        .speed(0.5),
                );
                ui.label("k");
                ui.small(i18n::t(i18n::Key::HintKUnit, lang));
                helper::help_button_inline(ui, i18n::t(i18n::Key::HelpUBatchSize, lang));
            });

            ui.horizontal_wrapped(|ui| {
                ui.label(i18n::t(i18n::Key::LabelSessionTimeout, lang));
                ui.add(
                    egui::DragValue::new(&mut settings.session_timeout)
                        .range(60..=3600)
                        .speed(10),
                );
                ui.label(i18n::t(i18n::Key::HintSUnit, lang));
                helper::help_button_inline(ui, i18n::t(i18n::Key::HelpSessionTimeout, lang));
            });

            ui.horizontal_wrapped(|ui| {
                ui.label(i18n::t(i18n::Key::LabelKvCacheRatio, lang));
                ui.add(
                    egui::DragValue::new(&mut settings.kv_cache_ratio)
                        .range(0.0..=1.0)
                        .speed(0.01),
                );
                ui.label(format!("{:.2}", settings.kv_cache_ratio));
                helper::help_button_inline(ui, i18n::t(i18n::Key::HelpKvCacheRatio, lang));
            });

            ui.horizontal_wrapped(|ui| {
                if ui
                    .add_enabled(
                        can_start,
                        theme::subtle_button(i18n::t(i18n::Key::BtnCalcKvCache, lang)),
                    )
                    .clicked()
                {
                    settings.kv_cache_result = match kv_cache::calc_and_format(settings) {
                        Ok(result) => Some(format!(
                            "{} {}",
                            i18n::t(i18n::Key::LabelKvCacheResult, lang),
                            result
                        )),
                        Err(error) => Some(error),
                    };
                }

                if let Some(result) = &settings.kv_cache_result {
                    ui.small(egui::RichText::new(result).color(theme::TEXT_MUTED));
                }
            });
        });

        ui.add_space(12.0);

        theme::section_card(ui, i18n::t(i18n::Key::SectionSampling, lang), |ui| {
            slider_row(
                ui,
                i18n::t(i18n::Key::LabelTemperature, lang),
                &mut settings.temperature,
                0.0..=2.0,
                i18n::t(i18n::Key::HelpTemperature, lang),
                &mut settings.ignore_temperature,
                i18n::t(i18n::Key::CheckboxIgnoreTemperature, lang),
            );

            slider_row(
                ui,
                i18n::t(i18n::Key::LabelTopP, lang),
                &mut settings.top_p,
                0.0..=1.0,
                i18n::t(i18n::Key::HelpTopP, lang),
                &mut settings.ignore_top_p,
                i18n::t(i18n::Key::CheckboxIgnoreTopP, lang),
            );

            ui.horizontal_wrapped(|ui| {
                ui.label(i18n::t(i18n::Key::LabelTopK, lang));
                ui.add(egui::DragValue::new(&mut settings.top_k).range(0..=1000));
                helper::help_button_inline(ui, i18n::t(i18n::Key::HelpTopK, lang));
                ui.checkbox(
                    &mut settings.ignore_top_k,
                    i18n::t(i18n::Key::CheckboxIgnoreTopK, lang),
                );
            });

            slider_row(
                ui,
                i18n::t(i18n::Key::LabelRepeatPenalty, lang),
                &mut settings.repeat_penalty,
                0.0..=2.0,
                i18n::t(i18n::Key::HelpRepeatPenalty, lang),
                &mut settings.ignore_repeat_penalty,
                i18n::t(i18n::Key::CheckboxIgnoreRepeatPenalty, lang),
            );

            slider_row(
                ui,
                i18n::t(i18n::Key::LabelPresencePenalty, lang),
                &mut settings.presence_penalty,
                -2.0..=2.0,
                i18n::t(i18n::Key::HelpPresencePenalty, lang),
                &mut settings.ignore_presence_penalty,
                i18n::t(i18n::Key::CheckboxIgnorePresencePenalty, lang),
            );

            ui.horizontal_wrapped(|ui| {
                ui.label(i18n::t(i18n::Key::LabelFlashAttn, lang));
                for (value, key) in [
                    ("on", i18n::Key::FaModeOn),
                    ("off", i18n::Key::FaModeOff),
                    ("auto", i18n::Key::FaModeAuto),
                ] {
                    if theme::pill_button(ui, settings.flash_attn == value, i18n::t(key, lang))
                        .clicked()
                    {
                        settings.flash_attn = value.to_string();
                    }
                }
                helper::help_button_inline(ui, i18n::t(i18n::Key::HelpFlashAttn, lang));
            });
        });

        ui.add_space(12.0);

        theme::section_card(ui, i18n::t(i18n::Key::SectionKvCache, lang), |ui| {
            ui.horizontal_wrapped(|ui| {
                ui.checkbox(
                    &mut settings.kv_offload,
                    i18n::t(i18n::Key::CheckboxKvOffload, lang),
                );
                ui.small(
                    egui::RichText::new(i18n::t(i18n::Key::HintKvOffload, lang))
                        .color(theme::TEXT_MUTED),
                );
                helper::help_button_inline(ui, i18n::t(i18n::Key::HelpKvOffload, lang));
            });

            selectable_values_row(
                ui,
                i18n::t(i18n::Key::LabelCacheTypeK, lang),
                &mut settings.cache_type_k,
                &[
                    "f32", "f16", "bf16", "q8_0", "q4_0", "q4_1", "iq4_nl", "q5_0", "q5_1",
                ],
                i18n::t(i18n::Key::HelpCacheTypeK, lang),
            );

            selectable_values_row(
                ui,
                i18n::t(i18n::Key::LabelCacheTypeV, lang),
                &mut settings.cache_type_v,
                &[
                    "f32", "f16", "bf16", "q8_0", "q4_0", "q4_1", "iq4_nl", "q5_0", "q5_1",
                ],
                i18n::t(i18n::Key::HelpCacheTypeV, lang),
            );

            checkbox_row(
                ui,
                &mut settings.kv_mlock,
                i18n::t(i18n::Key::CheckboxKvMlock, lang),
                i18n::t(i18n::Key::HelpKvMlock, lang),
            );
            checkbox_row(
                ui,
                &mut settings.kv_mmap,
                i18n::t(i18n::Key::CheckboxKvMmap, lang),
                i18n::t(i18n::Key::HelpKvMmap, lang),
            );
            checkbox_row(
                ui,
                &mut settings.kv_unified,
                i18n::t(i18n::Key::CheckboxKvUnified, lang),
                i18n::t(i18n::Key::HelpKvUnified, lang),
            );
            checkbox_row(
                ui,
                &mut settings.swa_full,
                i18n::t(i18n::Key::CheckboxSwaFull, lang),
                i18n::t(i18n::Key::HelpSwaFull, lang),
            );
        });

        ui.add_space(12.0);

        theme::section_card(ui, i18n::t(i18n::Key::SectionGpuDevice, lang), |ui| {
            let mut manual_gpu_layers =
                matches!(settings.gpu_layers_mode, GpuLayersMode::Manual(_));
            let mut gpu_layers = match settings.gpu_layers_mode {
                GpuLayersMode::Auto => 0,
                GpuLayersMode::All => 256,
                GpuLayersMode::Manual(value) => value,
            };

            ui.horizontal_wrapped(|ui| {
                ui.label(i18n::t(i18n::Key::LabelGpuDevice, lang));
                for (label_key, mode) in [
                    (i18n::Key::GpuModeAuto, GpuLayersMode::Auto),
                    (i18n::Key::GpuModeAll, GpuLayersMode::All),
                ] {
                    let selected = matches!(
                        (settings.gpu_layers_mode, mode),
                        (GpuLayersMode::Auto, GpuLayersMode::Auto)
                            | (GpuLayersMode::All, GpuLayersMode::All)
                    );
                    if theme::pill_button(ui, selected, i18n::t(label_key, lang)).clicked() {
                        settings.gpu_layers_mode = mode;
                        manual_gpu_layers = false;
                    }
                }
                helper::help_button_inline(ui, i18n::t(i18n::Key::HelpGpuDevice, lang));
            });

            ui.horizontal_wrapped(|ui| {
                ui.checkbox(
                    &mut manual_gpu_layers,
                    i18n::t(i18n::Key::CheckboxManualGpuLayers, lang),
                );
                helper::help_button_inline(ui, i18n::t(i18n::Key::HelpGpuDevice, lang));
            });

            if manual_gpu_layers {
                ui.horizontal_wrapped(|ui| {
                    ui.label(i18n::t(i18n::Key::LabelGpuDevice, lang));
                    ui.add(egui::DragValue::new(&mut gpu_layers).range(0..=256));
                    ui.small(i18n::t(i18n::Key::HintGpuDevice, lang));
                });
                settings.gpu_layers_mode = GpuLayersMode::Manual(gpu_layers);
            }

            ui.horizontal_wrapped(|ui| {
                ui.label(i18n::t(i18n::Key::LabelSplitMode, lang));
                for (value, label_key) in [
                    ("none", i18n::Key::SplitModeNone),
                    ("layer", i18n::Key::SplitModeLayer),
                    ("tensor", i18n::Key::SplitModeTensor),
                ] {
                    if theme::pill_button(
                        ui,
                        settings.split_mode == value,
                        i18n::t(label_key, lang),
                    )
                    .clicked()
                    {
                        settings.split_mode = value.to_string();
                    }
                }
                ui.small(
                    egui::RichText::new(i18n::t(i18n::Key::HintSplitMode, lang))
                        .color(theme::TEXT_MUTED),
                );
                helper::help_button_inline(ui, i18n::t(i18n::Key::HelpSplitMode, lang));
            });

            ui.horizontal_wrapped(|ui| {
                ui.label(i18n::t(i18n::Key::LabelTensorSplit, lang));
                ui.text_edit_singleline(&mut settings.tensor_split);
                ui.small(
                    egui::RichText::new(i18n::t(i18n::Key::HintTensorSplit, lang))
                        .color(theme::TEXT_MUTED),
                );
                helper::help_button_inline(ui, i18n::t(i18n::Key::HelpTensorSplit, lang));
            });

            ui.horizontal_wrapped(|ui| {
                ui.checkbox(
                    &mut settings.cpu_moe,
                    i18n::t(i18n::Key::CheckboxCpuMoe, lang),
                );
                ui.small(
                    egui::RichText::new(i18n::t(i18n::Key::HintCpuMoe, lang))
                        .color(theme::TEXT_MUTED),
                );
                helper::help_button_inline(ui, i18n::t(i18n::Key::HelpCpuMoe, lang));
            });
            if settings.cpu_moe {
                ui.horizontal_wrapped(|ui| {
                    ui.label(i18n::t(i18n::Key::LabelNCpuMoe, lang));
                    ui.add(egui::DragValue::new(&mut settings.n_cpu_moe).range(0..=256));
                    ui.small(
                        egui::RichText::new(i18n::t(i18n::Key::HintNCpuMoe, lang))
                            .color(theme::TEXT_MUTED),
                    );
                });
            }
        });

        ui.add_space(12.0);

        theme::section_card(ui, i18n::t(i18n::Key::SectionSpecDecoding, lang), |ui| {
            selectable_values_row(
                ui,
                i18n::t(i18n::Key::SpecTypeLabel, lang),
                &mut settings.spec_type,
                &[
                    "none",
                    "draft-simple",
                    "draft-eagle3",
                    "draft-mtp",
                    "ngram-simple",
                    "ngram-map-k",
                    "ngram-map-k4v",
                    "ngram-mod",
                    "ngram-cache",
                    "dflash",
                ],
                i18n::t(i18n::Key::HelpSpecType, lang),
            );

            ui.horizontal_wrapped(|ui| {
                ui.label(i18n::t(i18n::Key::SpecDraftNMaxLabel, lang));
                ui.add(egui::DragValue::new(&mut settings.spec_draft_n_max).range(0..=64));
                helper::help_button_inline(ui, i18n::t(i18n::Key::HelpSpecDraftNMax, lang));
            });

            ui.horizontal_wrapped(|ui| {
                ui.label(i18n::t(i18n::Key::SpecDraftNMinLabel, lang));
                ui.add(egui::DragValue::new(&mut settings.spec_draft_n_min).range(0..=32));
                helper::help_button_inline(ui, i18n::t(i18n::Key::HelpSpecDraftNMin, lang));
            });

            slider_row_without_ignore(
                ui,
                i18n::t(i18n::Key::SpecDraftPMinLabel, lang),
                &mut settings.spec_draft_p_min,
                0.0..=1.0,
                i18n::t(i18n::Key::HelpSpecDraftPMin, lang),
            );

            slider_row_without_ignore(
                ui,
                i18n::t(i18n::Key::SpecDraftPSplitLabel, lang),
                &mut settings.spec_draft_p_split,
                0.0..=1.0,
                i18n::t(i18n::Key::HelpSpecDraftPSplit, lang),
            );
        });
    });
}

fn slider_row(
    ui: &mut egui::Ui,
    label: &str,
    value: &mut f32,
    range: std::ops::RangeInclusive<f32>,
    help: &str,
    ignore: &mut bool,
    ignore_label: &str,
) {
    ui.horizontal_wrapped(|ui| {
        ui.label(label);
        ui.add(
            egui::Slider::new(value, range)
                .smallest_positive(0.01)
                .custom_formatter(|current, _| format!("{:.2}", current)),
        );
        ui.label(format!("{:.2}", *value));
        helper::help_button_inline(ui, help);
        ui.checkbox(ignore, ignore_label);
    });
}

fn slider_row_without_ignore(
    ui: &mut egui::Ui,
    label: &str,
    value: &mut f32,
    range: std::ops::RangeInclusive<f32>,
    help: &str,
) {
    ui.horizontal_wrapped(|ui| {
        ui.label(label);
        ui.add(
            egui::Slider::new(value, range)
                .smallest_positive(0.01)
                .custom_formatter(|current, _| format!("{:.2}", current)),
        );
        ui.label(format!("{:.2}", *value));
        helper::help_button_inline(ui, help);
    });
}

fn selectable_values_row(
    ui: &mut egui::Ui,
    label: &str,
    current: &mut String,
    values: &[&str],
    help: &str,
) {
    ui.horizontal_wrapped(|ui| {
        ui.label(label);
        for value in values {
            if theme::pill_button(ui, current.as_str() == *value, value).clicked() {
                *current = (*value).to_string();
            }
        }
        helper::help_button_inline(ui, help);
    });
}

fn checkbox_row(ui: &mut egui::Ui, value: &mut bool, label: &str, help: &str) {
    ui.horizontal_wrapped(|ui| {
        ui.checkbox(value, label);
        helper::help_button_inline(ui, help);
    });
}
