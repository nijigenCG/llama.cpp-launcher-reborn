use crate::config::settings::AppSettings;
use crate::i18n;
use crate::ui::theme;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

#[derive(Clone)]
struct ModelFileEntry {
    path: PathBuf,
    name: String,
    tags: Vec<(String, egui::Color32)>,
}

#[derive(Clone, Copy, PartialEq)]
enum FileMode {
    Main,
    Mmproj,
    Dflash,
}

#[derive(Default)]
pub struct ModelBrowserState {
    cached_dir: PathBuf,
    cached_modified: Option<SystemTime>,
    main_files: Vec<ModelFileEntry>,
    mmproj_files: Vec<ModelFileEntry>,
    dflash_files: Vec<ModelFileEntry>,
}

impl ModelBrowserState {
    pub fn invalidate(&mut self) {
        self.cached_dir.clear();
        self.cached_modified = None;
        self.main_files.clear();
        self.mmproj_files.clear();
        self.dflash_files.clear();
    }

    pub fn sync(&mut self, dir: &Path) {
        if dir.as_os_str().is_empty() {
            self.invalidate();
            return;
        }

        let modified = directory_modified(dir);
        if self.cached_dir == dir && self.cached_modified == modified {
            return;
        }

        let (main_files, mmproj_files, dflash_files) = scan_directory(dir);
        self.cached_dir = dir.to_path_buf();
        self.cached_modified = modified;
        self.main_files = main_files;
        self.mmproj_files = mmproj_files;
        self.dflash_files = dflash_files;
    }

    fn files(&self, mode: FileMode) -> &[ModelFileEntry] {
        match mode {
            FileMode::Main => &self.main_files,
            FileMode::Mmproj => &self.mmproj_files,
            FileMode::Dflash => &self.dflash_files,
        }
    }
}

fn directory_modified(path: &Path) -> Option<SystemTime> {
    path.metadata().ok()?.modified().ok()
}

fn scan_directory(
    dir: &Path,
) -> (
    Vec<ModelFileEntry>,
    Vec<ModelFileEntry>,
    Vec<ModelFileEntry>,
) {
    let mut main_files = Vec::new();
    let mut mmproj_files = Vec::new();
    let mut dflash_files = Vec::new();

    let entries = match std::fs::read_dir(dir) {
        Ok(entries) => entries,
        Err(_) => return (main_files, mmproj_files, dflash_files),
    };

    for entry in entries.filter_map(Result::ok) {
        let path = entry.path();
        let name = entry.file_name().to_string_lossy().to_string();
        let lower_name = name.to_lowercase();
        if !lower_name.ends_with(".gguf") {
            continue;
        }

        let model = ModelFileEntry {
            path,
            tags: parse_tags(&name),
            name,
        };

        if is_dflash_file(&lower_name) {
            dflash_files.push(model);
        } else if is_mmproj_file(&lower_name) {
            mmproj_files.push(model);
        } else {
            main_files.push(model);
        }
    }

    let sort_by_name = |files: &mut Vec<ModelFileEntry>| {
        files.sort_by_key(|entry| entry.name.to_lowercase());
    };
    sort_by_name(&mut main_files);
    sort_by_name(&mut mmproj_files);
    sort_by_name(&mut dflash_files);

    (main_files, mmproj_files, dflash_files)
}

/// 自动检测模型文件夹
/// 检查应用所在目录中是否有 model 或 models 文件夹（不区分大小写）
fn auto_detect_model_dir() -> Option<PathBuf> {
    let exe_dir = std::env::current_exe().ok()?.parent()?.to_path_buf();

    let dirs: Vec<_> = match std::fs::read_dir(&exe_dir) {
        Ok(entries) => entries
            .filter_map(|e| e.ok())
            .filter(|e| e.path().is_dir())
            .collect(),
        Err(_) => return None,
    };

    let models_dir = dirs.iter().find(|e| {
        e.file_name()
            .to_string_lossy()
            .to_lowercase()
            .eq_ignore_ascii_case("models")
    });
    if let Some(dir) = models_dir {
        return Some(dir.path());
    }

    let model_dir = dirs.iter().find(|e| {
        e.file_name()
            .to_string_lossy()
            .to_lowercase()
            .eq_ignore_ascii_case("model")
    });
    if let Some(dir) = model_dir {
        return Some(dir.path());
    }

    None
}

/// 文件名解析为彩色标签（9 色方案）
fn parse_tags(filename: &str) -> Vec<(String, egui::Color32)> {
    let stem = filename.strip_suffix(".gguf").unwrap_or(filename);

    let purple = egui::Color32::from_rgb(180, 120, 255);
    let orange = egui::Color32::from_rgb(255, 165, 0);
    let gray = egui::Color32::from_rgb(160, 160, 160);
    let green = egui::Color32::from_rgb(100, 200, 100);
    let blue = egui::Color32::from_rgb(100, 150, 255);
    let yellow = egui::Color32::from_rgb(255, 215, 0);
    let pink = egui::Color32::from_rgb(255, 100, 130);
    let brown = egui::Color32::from_rgb(205, 133, 63);
    let cyan = egui::Color32::from_rgb(0, 210, 210);

    let mut tags = Vec::new();
    for part in stem.split('-') {
        let trimmed = part.trim();
        if trimmed.is_empty() {
            continue;
        }

        let lower = trimmed.to_lowercase();
        let color = if is_param_size(&lower) {
            purple
        } else if is_quantization(&lower) {
            orange
        } else if trimmed.chars().all(|c| c.is_ascii_digit() || c == '.') {
            gray
        } else if is_training_method(&lower) {
            green
        } else if lower.contains("fp16")
            || lower.contains("bf16")
            || lower.contains("f32")
            || lower.contains("fp8")
        {
            yellow
        } else if lower.contains("lora") || lower.contains("adapter") || lower.contains("delta") {
            pink
        } else if is_context_length(&lower) {
            brown
        } else if lower.contains("mamba")
            || lower.contains("rwkv")
            || lower.contains("hyena")
            || lower.contains("decoder")
        {
            cyan
        } else {
            blue
        };

        tags.push((trimmed.to_string(), color));
    }

    tags
}

fn is_param_size(s: &str) -> bool {
    let has_digit = s.chars().any(|c| c.is_ascii_digit());
    has_digit && (s.ends_with('b') || s.ends_with('m'))
}

fn is_quantization(s: &str) -> bool {
    if s.starts_with("iq") && s.chars().nth(2).is_some_and(|c| c.is_ascii_digit()) {
        return true;
    }
    s.starts_with('q') && s.chars().nth(1).is_some_and(|c| c.is_ascii_digit())
}

fn is_training_method(s: &str) -> bool {
    s.contains("instruct")
        || s.contains("chat")
        || s.contains("sft")
        || s.contains("rlhf")
        || s.contains("dpo")
        || s.contains("orpo")
        || s.contains("grpo")
}

fn is_context_length(s: &str) -> bool {
    if s.ends_with('k') && s.contains(|c: char| c.is_ascii_digit()) {
        return true;
    }
    s.contains("long") || s == "128" || s == "64" || s == "32"
}

fn is_mmproj_file(filename: &str) -> bool {
    filename.contains("mmproj")
        || filename.contains("clip")
        || (filename.contains("proj") && filename.contains("vision"))
}

fn is_dflash_file(filename: &str) -> bool {
    filename.contains("dflash")
}

fn render_file_list(
    ui: &mut egui::Ui,
    files: &[ModelFileEntry],
    selected_path: &Path,
    empty_key: i18n::Key,
    on_select: &mut impl FnMut(PathBuf),
    lang: &i18n::Language,
) {
    if files.is_empty() {
        ui.colored_label(
            theme::TEXT_MUTED,
            egui::RichText::new(i18n::t(empty_key, lang)).italics(),
        );
        return;
    }

    for file in files {
        let selected = selected_path == file.path.as_path();
        egui::Frame::default()
            .fill(if selected {
                theme::ACCENT_SOFT
            } else {
                theme::SURFACE_BG
            })
            .inner_margin(egui::Margin::same(12))
            .stroke(egui::Stroke::new(
                1.0,
                if selected {
                    theme::ACCENT
                } else {
                    theme::BORDER
                },
            ))
            .corner_radius(16.0)
            .show(ui, |ui| {
                if ui
                    .selectable_label(selected, egui::RichText::new(&file.name).strong())
                    .clicked()
                {
                    on_select(file.path.clone());
                }

                ui.add_space(6.0);
                ui.horizontal_wrapped(|ui| {
                    for (text, color) in &file.tags {
                        theme::tag_chip(ui, text, *color);
                    }
                });
            });
        ui.add_space(8.0);
    }
}

pub fn ui(
    ui: &mut egui::Ui,
    settings: &mut AppSettings,
    browser: &mut ModelBrowserState,
    lang: &i18n::Language,
) {
    browser.sync(&settings.model_dir);

    theme::page_frame().show(ui, |ui| {
        theme::page_title(ui, i18n::t(i18n::Key::PanelModelTitle, lang));

        theme::section_card(ui, i18n::t(i18n::Key::LabelModelDir, lang), |ui| {
            let mut dir_str = settings.model_dir.to_string_lossy().to_string();
            if ui.text_edit_singleline(&mut dir_str).changed() {
                settings.model_dir = PathBuf::from(&dir_str);
            }

            ui.add_space(8.0);
            ui.horizontal_wrapped(|ui| {
                if ui
                    .add(theme::subtle_button(i18n::t(
                        i18n::Key::BtnSelectFolder,
                        lang,
                    )))
                    .clicked()
                {
                    if let Some(path) = rfd::FileDialog::new()
                        .set_title(i18n::t(i18n::Key::DialogSelectFolder, lang))
                        .pick_folder()
                    {
                        settings.model_dir = path;
                    }
                }
                if ui
                    .add(theme::accent_button(
                        i18n::t(i18n::Key::BtnAutoDetect, lang),
                        theme::INFO,
                    ))
                    .clicked()
                {
                    settings.model_dir = auto_detect_model_dir().unwrap_or_default();
                }
            });
        });

        ui.add_space(12.0);

        if settings.model_dir.as_os_str().is_empty() {
            ui.colored_label(theme::TEXT_MUTED, i18n::t(i18n::Key::NoModelDir, lang));
            return;
        }

        browser.sync(&settings.model_dir);

        theme::section_card(
            ui,
            &format!(
                "{} ({})",
                i18n::t(i18n::Key::SectionModels, lang),
                browser.files(FileMode::Main).len()
            ),
            |ui| {
                let selected_model = settings.model_path.clone();
                render_file_list(
                    ui,
                    browser.files(FileMode::Main),
                    &selected_model,
                    i18n::Key::NoGgufFiles,
                    &mut |path| settings.model_path = path,
                    lang,
                );
            },
        );

        ui.add_space(12.0);

        theme::section_card(
            ui,
            &format!(
                "{} ({})",
                i18n::t(i18n::Key::SectionMmproj, lang),
                browser.files(FileMode::Mmproj).len()
            ),
            |ui| {
                let selected_mmproj = settings.mmproj_path.clone();
                render_file_list(
                    ui,
                    browser.files(FileMode::Mmproj),
                    &selected_mmproj,
                    i18n::Key::NoMmprojFiles,
                    &mut |path| {
                        settings.mmproj_path = if selected_mmproj == path {
                            PathBuf::new()
                        } else {
                            path
                        };
                    },
                    lang,
                );
            },
        );

        ui.add_space(12.0);

        theme::section_card(
            ui,
            &format!(
                "{} ({})",
                i18n::t(i18n::Key::SectionDflash, lang),
                browser.files(FileMode::Dflash).len()
            ),
            |ui| {
                let selected_dflash = settings.dflash_path.clone();
                render_file_list(
                    ui,
                    browser.files(FileMode::Dflash),
                    &selected_dflash,
                    i18n::Key::NoDflashFiles,
                    &mut |path| {
                        settings.dflash_path = if selected_dflash == path {
                            PathBuf::new()
                        } else {
                            path
                        };
                    },
                    lang,
                );
            },
        );
    });
}
