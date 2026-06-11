use crate::config::settings::{AppSettings, GpuLayersMode, Preset};
use crate::i18n;
use crate::ui::theme;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct ParamsExport {
    context: usize,
    batch_size: usize,
    ubatch_size: f32,
    temperature: f32,
    top_p: f32,
    top_k: i32,
    repeat_penalty: f32,
    presence_penalty: f32,
    ignore_temperature: bool,
    ignore_top_p: bool,
    ignore_top_k: bool,
    ignore_repeat_penalty: bool,
    ignore_presence_penalty: bool,
    flash_attn: String,
    spec_type: String,
    spec_draft_n_max: usize,
    spec_draft_n_min: usize,
    spec_draft_p_min: f32,
    spec_draft_p_split: f32,
    kv_offload: bool,
    cache_type_k: String,
    cache_type_v: String,
    kv_mlock: bool,
    kv_mmap: bool,
    kv_unified: bool,
    swa_full: bool,
    gpu_layers_mode: GpuLayersMode,
    split_mode: String,
    tensor_split: String,
    cpu_moe: bool,
    n_cpu_moe: usize,
}

impl ParamsExport {
    fn from_settings(settings: &AppSettings) -> Self {
        Self {
            context: settings.context,
            batch_size: settings.batch_size,
            ubatch_size: settings.ubatch_size,
            temperature: settings.temperature,
            top_p: settings.top_p,
            top_k: settings.top_k,
            repeat_penalty: settings.repeat_penalty,
            presence_penalty: settings.presence_penalty,
            ignore_temperature: settings.ignore_temperature,
            ignore_top_p: settings.ignore_top_p,
            ignore_top_k: settings.ignore_top_k,
            ignore_repeat_penalty: settings.ignore_repeat_penalty,
            ignore_presence_penalty: settings.ignore_presence_penalty,
            flash_attn: settings.flash_attn.clone(),
            spec_type: settings.spec_type.clone(),
            spec_draft_n_max: settings.spec_draft_n_max,
            spec_draft_n_min: settings.spec_draft_n_min,
            spec_draft_p_min: settings.spec_draft_p_min,
            spec_draft_p_split: settings.spec_draft_p_split,
            kv_offload: settings.kv_offload,
            cache_type_k: settings.cache_type_k.clone(),
            cache_type_v: settings.cache_type_v.clone(),
            kv_mlock: settings.kv_mlock,
            kv_mmap: settings.kv_mmap,
            kv_unified: settings.kv_unified,
            swa_full: settings.swa_full,
            gpu_layers_mode: settings.gpu_layers_mode,
            split_mode: settings.split_mode.clone(),
            tensor_split: settings.tensor_split.clone(),
            cpu_moe: settings.cpu_moe,
            n_cpu_moe: settings.n_cpu_moe,
        }
    }

    fn apply_to(self, settings: &mut AppSettings) {
        settings.context = self.context;
        settings.batch_size = self.batch_size;
        settings.ubatch_size = self.ubatch_size;
        settings.temperature = self.temperature;
        settings.top_p = self.top_p;
        settings.top_k = self.top_k;
        settings.repeat_penalty = self.repeat_penalty;
        settings.presence_penalty = self.presence_penalty;
        settings.ignore_temperature = self.ignore_temperature;
        settings.ignore_top_p = self.ignore_top_p;
        settings.ignore_top_k = self.ignore_top_k;
        settings.ignore_repeat_penalty = self.ignore_repeat_penalty;
        settings.ignore_presence_penalty = self.ignore_presence_penalty;
        settings.flash_attn = self.flash_attn;
        settings.spec_type = self.spec_type;
        settings.spec_draft_n_max = self.spec_draft_n_max;
        settings.spec_draft_n_min = self.spec_draft_n_min;
        settings.spec_draft_p_min = self.spec_draft_p_min;
        settings.spec_draft_p_split = self.spec_draft_p_split;
        settings.kv_offload = self.kv_offload;
        settings.cache_type_k = self.cache_type_k;
        settings.cache_type_v = self.cache_type_v;
        settings.kv_mlock = self.kv_mlock;
        settings.kv_mmap = self.kv_mmap;
        settings.kv_unified = self.kv_unified;
        settings.swa_full = self.swa_full;
        settings.gpu_layers_mode = self.gpu_layers_mode;
        settings.split_mode = self.split_mode;
        settings.tensor_split = self.tensor_split;
        settings.cpu_moe = self.cpu_moe;
        settings.n_cpu_moe = self.n_cpu_moe;
    }
}

pub fn ui(ui: &mut egui::Ui, settings: &mut AppSettings, lang: &i18n::Language) -> bool {
    let mut should_start_server = false;

    theme::page_frame().show(ui, |ui| {
        theme::page_title(ui, i18n::t(i18n::Key::SectionPresets, lang));

        theme::section_card(ui, i18n::t(i18n::Key::LabelPresetName, lang), |ui| {
            ui.horizontal_wrapped(|ui| {
                ui.text_edit_singleline(&mut settings.new_preset_name);

                if ui
                    .add(theme::accent_button(
                        i18n::t(i18n::Key::BtnSavePreset, lang),
                        theme::SUCCESS,
                    ))
                    .clicked()
                {
                    let trimmed = settings.new_preset_name.trim().to_string();
                    if !trimmed.is_empty() {
                        let preset = Preset::from_settings(settings, trimmed.clone());
                        if let Some(index) = settings.presets.iter().position(|p| p.name == trimmed)
                        {
                            settings.presets[index] = preset;
                        } else {
                            settings.presets.push(preset);
                        }
                        settings.new_preset_name.clear();
                    }
                }

                if ui
                    .add(theme::subtle_button(i18n::t(
                        i18n::Key::BtnExportParams,
                        lang,
                    )))
                    .clicked()
                {
                    export_params(settings);
                }

                if ui
                    .add(theme::subtle_button(i18n::t(
                        i18n::Key::BtnImportParams,
                        lang,
                    )))
                    .clicked()
                {
                    import_params(settings);
                }
            });
        });

        ui.add_space(12.0);

        if settings.presets.is_empty() {
            ui.colored_label(theme::TEXT_MUTED, i18n::t(i18n::Key::HintNoPresets, lang));
            return;
        }

        let mut load_index = None;
        let mut delete_index = None;
        let mut auto_start_name = None;

        theme::section_card(ui, i18n::t(i18n::Key::SectionPresets, lang), |ui| {
            for (index, preset) in settings.presets.iter().enumerate() {
                egui::Frame::default()
                    .fill(theme::SURFACE_BG)
                    .stroke(egui::Stroke::new(1.0, theme::BORDER))
                    .corner_radius(16.0)
                    .inner_margin(egui::Margin::same(12))
                    .show(ui, |ui| {
                        ui.horizontal_wrapped(|ui| {
                            if ui
                                .selectable_label(false, egui::RichText::new(&preset.name).strong())
                                .clicked()
                            {
                                load_index = Some(index);
                            }

                            if ui
                                .add(theme::subtle_button(i18n::t(
                                    i18n::Key::BtnApplyPreset,
                                    lang,
                                )))
                                .clicked()
                            {
                                load_index = Some(index);
                            }

                            let mut is_auto = settings
                                .auto_start_preset_name
                                .as_ref()
                                .is_some_and(|name| *name == preset.name);
                            if ui
                                .checkbox(
                                    &mut is_auto,
                                    i18n::t(i18n::Key::CheckboxAutoStartPreset, lang),
                                )
                                .changed()
                            {
                                if is_auto {
                                    auto_start_name = Some(preset.name.clone());
                                } else if settings.auto_start_preset_name.as_ref()
                                    == Some(&preset.name)
                                {
                                    settings.auto_start_preset_name = None;
                                }
                            }

                            if ui
                                .add(theme::subtle_button(i18n::t(
                                    i18n::Key::BtnRenamePreset,
                                    lang,
                                )))
                                .clicked()
                            {
                                settings.rename_preset_index = Some(index);
                                settings.rename_preset_new_name = preset.name.clone();
                            }

                            if ui
                                .add(theme::accent_button(
                                    i18n::t(i18n::Key::BtnDeletePreset, lang),
                                    theme::DANGER,
                                ))
                                .clicked()
                            {
                                delete_index = Some(index);
                            }
                        });
                    });
                ui.add_space(8.0);
            }
        });

        if let Some(name) = auto_start_name {
            settings.auto_start_preset_name = Some(name);
        }

        if let Some(index) = load_index {
            if let Some(preset) = settings.presets.get(index).cloned() {
                let should_auto_start = settings
                    .auto_start_preset_name
                    .as_ref()
                    .is_some_and(|name| *name == preset.name);
                preset.apply_to(settings);
                should_start_server = should_auto_start;
            }
        }

        if let Some(index) = delete_index {
            if index < settings.presets.len() {
                settings.presets.remove(index);
                if settings.rename_preset_index == Some(index) {
                    settings.rename_preset_index = None;
                    settings.rename_preset_new_name.clear();
                }
            }
        }
    });

    if let Some(index) = settings.rename_preset_index {
        if index < settings.presets.len() {
            egui::Window::new(i18n::t(i18n::Key::BtnRenamePreset, lang))
                .collapsible(false)
                .resizable(false)
                .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
                .fixed_size([320.0, 0.0])
                .show(ui.ctx(), |ui| {
                    ui.label(i18n::t(i18n::Key::LabelPresetName, lang));
                    ui.text_edit_singleline(&mut settings.rename_preset_new_name);
                    ui.add_space(10.0);
                    ui.horizontal_wrapped(|ui| {
                        if ui
                            .add(theme::accent_button(
                                i18n::t(i18n::Key::BtnConfirm, lang),
                                theme::SUCCESS,
                            ))
                            .clicked()
                        {
                            let trimmed = settings.rename_preset_new_name.trim().to_string();
                            if !trimmed.is_empty() {
                                settings.presets[index].name = trimmed;
                            }
                            settings.rename_preset_index = None;
                            settings.rename_preset_new_name.clear();
                        }

                        if ui
                            .add(theme::subtle_button(i18n::t(i18n::Key::BtnCancel, lang)))
                            .clicked()
                        {
                            settings.rename_preset_index = None;
                            settings.rename_preset_new_name.clear();
                        }
                    });
                });
        }
    }

    should_start_server
}

fn export_params(settings: &AppSettings) {
    let params = ParamsExport::from_settings(settings);
    let json = match serde_json::to_string_pretty(&params) {
        Ok(json) => json,
        Err(error) => {
            eprintln!("[export_params] serialize failed: {}", error);
            return;
        }
    };

    if let Some(path) = rfd::FileDialog::new()
        .set_file_name("llama_cpp_launcher_parameter_export.json")
        .add_filter("JSON", &["json"])
        .save_file()
    {
        if let Err(error) = std::fs::write(path, json) {
            eprintln!("[export_params] write failed: {}", error);
        }
    }
}

fn import_params(settings: &mut AppSettings) {
    let Some(path) = rfd::FileDialog::new()
        .add_filter("JSON", &["json"])
        .pick_file()
    else {
        return;
    };

    let content = match std::fs::read_to_string(path) {
        Ok(content) => content,
        Err(error) => {
            eprintln!("[import_params] read failed: {}", error);
            return;
        }
    };

    match serde_json::from_str::<ParamsExport>(&content) {
        Ok(params) => params.apply_to(settings),
        Err(error) => eprintln!("[import_params] parse failed: {}", error),
    }
}
