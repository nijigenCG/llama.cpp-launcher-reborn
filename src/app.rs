use crate::config::settings::{
    is_rpc_binary_name, is_server_binary_name, AppSettings, SettingsManager,
};
use crate::engine::rpc::{RpcManager, RpcState};
use crate::engine::server::{ServerManager, ServerState};
use crate::i18n::{self, Language};
use crate::spacing_debugger::SpacingDebugger;
use crate::ui::{
    launch_commands_panel, log_panel, model_panel, params_panel, presets_panel, rpc_panel,
    server_panel, theme,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum AppTab {
    Server,
    Rpc,
    Model,
    Params,
    Log,
    Commands,
    Presets,
}

impl AppTab {
    const ALL: [Self; 7] = [
        Self::Server,
        Self::Rpc,
        Self::Model,
        Self::Params,
        Self::Log,
        Self::Commands,
        Self::Presets,
    ];

    fn key(self) -> i18n::Key {
        match self {
            Self::Server => i18n::Key::TabServer,
            Self::Rpc => i18n::Key::TabRpc,
            Self::Model => i18n::Key::TabModel,
            Self::Params => i18n::Key::TabParams,
            Self::Log => i18n::Key::TabLog,
            Self::Commands => i18n::Key::TabCommands,
            Self::Presets => i18n::Key::TabPresets,
        }
    }

    fn label(self, lang: &Language) -> &'static str {
        i18n::t(self.key(), lang)
    }
}

pub struct LlamaLauncherApp {
    settings: AppSettings,
    settings_manager: SettingsManager,
    server_manager: ServerManager,
    rpc_manager: RpcManager,
    model_browser: model_panel::ModelBrowserState,
    tab_selected: AppTab,
    show_about: bool,
    lang: Language,
    auto_start_server_on_first_frame: bool,
    start_minimized: bool,
    debug_mode: bool,
    spacing_debugger: SpacingDebugger,
}

impl LlamaLauncherApp {
    pub fn new(cc: &eframe::CreationContext<'_>, start_minimized: bool) -> Self {
        let settings_manager = SettingsManager::new();
        let mut settings = settings_manager.load().unwrap_or_default();

        if let Some(ref preset_name) = settings.auto_start_preset_name.clone() {
            if let Some(preset) = settings
                .presets
                .iter()
                .find(|preset| preset.name == *preset_name)
            {
                preset.clone().apply_to(&mut settings);
            }
        }

        let auto_start_server_on_first_frame = settings.auto_start_preset_name.is_some();
        let locale = sys_locale::get_locale().unwrap_or_default();
        let lang = if locale.starts_with("zh") {
            Language::Zh
        } else {
            Language::En
        };

        cc.egui_ctx.set_zoom_factor(1.25);
        theme::apply(&cc.egui_ctx);
        crate::set_log_to_file(settings.log_to_file);

        Self {
            settings,
            settings_manager,
            server_manager: ServerManager::new(),
            rpc_manager: RpcManager::new(),
            model_browser: model_panel::ModelBrowserState::default(),
            tab_selected: AppTab::Server,
            show_about: false,
            lang,
            auto_start_server_on_first_frame,
            start_minimized,
            debug_mode: false,
            spacing_debugger: SpacingDebugger::new(),
        }
    }

    fn save(&mut self) {
        if let Err(error) = self.settings_manager.save(&self.settings) {
            log::error!("save settings failed: {}", error);
        }
    }

    fn note_rect(&mut self, response: &egui::Response) {
        if self.debug_mode {
            self.spacing_debugger.rects.push(response.rect);
        }
    }

    fn can_start_server(&self) -> bool {
        let server_path_valid = self
            .settings
            .server_path
            .file_name()
            .and_then(|file| file.to_str())
            .is_some_and(is_server_binary_name);
        server_path_valid && !self.settings.model_path.as_os_str().is_empty()
    }

    fn can_start_rpc(&self) -> bool {
        self.settings
            .rpc_server_path
            .file_name()
            .and_then(|file| file.to_str())
            .is_some_and(is_rpc_binary_name)
    }

    fn current_model_name(&self) -> String {
        self.settings
            .model_path
            .file_name()
            .and_then(|file| file.to_str())
            .unwrap_or("-")
            .to_string()
    }

    fn render_server_controls(&mut self, ui: &mut egui::Ui) {
        match self.server_manager.state() {
            ServerState::Idle | ServerState::Error(_) => {
                let response = ui.add_enabled(
                    self.can_start_server(),
                    theme::accent_button(
                        i18n::t(i18n::Key::BtnStartServer, &self.lang),
                        theme::SUCCESS,
                    ),
                );
                self.note_rect(&response);
                if response.clicked() {
                    self.server_manager.start(&self.settings);
                }
            }
            ServerState::Running => {
                let response = ui.add(theme::accent_button(
                    i18n::t(i18n::Key::BtnStopServer, &self.lang),
                    theme::DANGER,
                ));
                self.note_rect(&response);
                if response.clicked() {
                    self.server_manager.stop();
                }
            }
            ServerState::Starting | ServerState::Stopping => {
                let response = ui.label(i18n::t(i18n::Key::StatusProcessing, &self.lang));
                self.note_rect(&response);
            }
        }
    }

    fn render_rpc_controls(&mut self, ui: &mut egui::Ui) {
        match self.rpc_manager.state() {
            RpcState::Idle | RpcState::Error(_) => {
                let response = ui.add_enabled(
                    self.can_start_rpc(),
                    theme::accent_button(i18n::t(i18n::Key::BtnStartRpc, &self.lang), theme::INFO),
                );
                self.note_rect(&response);
                if response.clicked() {
                    self.rpc_manager.start(&self.settings);
                }
            }
            RpcState::Running => {
                let response = ui.add(theme::accent_button(
                    i18n::t(i18n::Key::BtnStopRpc, &self.lang),
                    theme::DANGER,
                ));
                self.note_rect(&response);
                if response.clicked() {
                    self.rpc_manager.stop();
                }
            }
            RpcState::Starting | RpcState::Stopping => {
                let response = ui.label(i18n::t(i18n::Key::StatusProcessing, &self.lang));
                self.note_rect(&response);
            }
        }
    }

    fn render_web_client_button(&mut self, ui: &mut egui::Ui) {
        let enabled = matches!(self.server_manager.state(), ServerState::Running)
            && self.server_manager.is_listening();
        let response = ui.add_enabled(
            enabled,
            theme::subtle_button(i18n::t(i18n::Key::BtnOpenWebClient, &self.lang)),
        );
        self.note_rect(&response);
        if response.clicked() {
            open_web_client_url(self.settings.port);
        }
    }

    fn render_file_menu(&mut self, ui: &mut egui::Ui) {
        ui.menu_button(i18n::t(i18n::Key::MenuFile, &self.lang), |ui| {
            if ui
                .button(i18n::t(i18n::Key::MenuItemSaveConfig, &self.lang))
                .clicked()
            {
                self.save();
            }

            if ui
                .button(i18n::t(i18n::Key::MenuItemLoadConfig, &self.lang))
                .clicked()
            {
                if let Ok(settings) = self.settings_manager.load() {
                    self.settings = settings;
                    self.model_browser.invalidate();
                    crate::set_log_to_file(self.settings.log_to_file);
                }
            }

            if ui
                .checkbox(
                    &mut self.settings.auto_start,
                    i18n::t(i18n::Key::MenuItemAutoStart, &self.lang),
                )
                .changed()
            {
                if self.settings.auto_start {
                    enable_auto_start();
                } else {
                    disable_auto_start();
                }
                self.save();
            }

            if ui
                .button(i18n::t(i18n::Key::MenuItemCreateShortcut, &self.lang))
                .clicked()
            {
                let _ = crate::shortcut::create_desktop_shortcut();
            }
        });
    }

    fn render_help_menu(&mut self, ui: &mut egui::Ui) {
        ui.menu_button(i18n::t(i18n::Key::MenuHelp, &self.lang), |ui| {
            if ui
                .button(i18n::t(i18n::Key::MenuItemAbout, &self.lang))
                .clicked()
            {
                self.show_about = true;
            }

            if ui
                .button(i18n::t(i18n::Key::MenuItemRepo, &self.lang))
                .clicked()
            {
                open_repo_url();
            }

            let mut log_to_file = self.settings.log_to_file;
            if ui
                .checkbox(
                    &mut log_to_file,
                    i18n::t(i18n::Key::MenuItemLogToFile, &self.lang),
                )
                .changed()
            {
                crate::set_log_to_file(log_to_file);
                self.settings.log_to_file = log_to_file;
                self.save();
            }

            ui.checkbox(
                &mut self.debug_mode,
                i18n::t(i18n::Key::MenuItemDebugMode, &self.lang),
            );
        });
    }

    fn render_status_strip(&self, ui: &mut egui::Ui) {
        let server_status = self.server_manager.status_text(&self.lang);
        let rpc_status = self.rpc_manager.status_text(&self.lang);
        let server_color = server_state_color(self.server_manager.state());
        let rpc_color = rpc_state_color(self.rpc_manager.state());

        theme::status_badge(
            ui,
            i18n::t(i18n::Key::TabServer, &self.lang),
            &server_status,
            server_color,
        );
        theme::status_badge(
            ui,
            i18n::t(i18n::Key::TabRpc, &self.lang),
            &rpc_status,
            rpc_color,
        );
        theme::status_badge(
            ui,
            i18n::t(i18n::Key::TabModel, &self.lang),
            &self.current_model_name(),
            theme::INFO,
        );
    }

    fn show_about_window(&mut self, ui: &mut egui::Ui) {
        if !self.show_about {
            return;
        }

        egui::Window::new(i18n::t(i18n::Key::AboutTitle, &self.lang))
            .collapsible(false)
            .resizable(false)
            .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
            .fixed_size([320.0, 0.0])
            .show(ui, |ui| {
                ui.label(
                    egui::RichText::new(i18n::t(i18n::Key::AboutVersion, &self.lang))
                        .strong()
                        .size(20.0)
                        .color(theme::ACCENT),
                );
                ui.label(i18n::t(i18n::Key::AboutDescription, &self.lang));
                ui.add_space(10.0);
                ui.small(
                    egui::RichText::new(i18n::t(i18n::Key::AboutCopyright, &self.lang))
                        .color(theme::TEXT_MUTED),
                );
                ui.add_space(12.0);
                if ui
                    .add(theme::subtle_button(i18n::t(
                        i18n::Key::BtnClose,
                        &self.lang,
                    )))
                    .clicked()
                {
                    self.show_about = false;
                }
            });
    }
}

impl eframe::App for LlamaLauncherApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        {
            let ctx = ui.ctx();
            ctx.request_repaint_after(std::time::Duration::from_millis(250));

            #[cfg(debug_assertions)]
            {
                ctx.set_debug_on_hover(self.debug_mode);
                ctx.global_style_mut(|style| {
                    style.debug.hover_shows_next = self.debug_mode;
                });
            }
        }

        if self.debug_mode {
            self.spacing_debugger.begin_frame();
        }

        if self.auto_start_server_on_first_frame {
            self.auto_start_server_on_first_frame = false;
            self.server_manager.start(&self.settings);
        }

        if self.start_minimized {
            self.start_minimized = false;
            ui.ctx()
                .send_viewport_cmd(egui::ViewportCommand::Visible(false));
        }

        self.server_manager.poll_logs();
        self.rpc_manager.poll();
        self.show_about_window(ui);

        egui::Panel::top("top_panel")
            .frame(theme::chrome_frame())
            .show_inside(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.vertical(|ui| {
                        ui.label(
                            egui::RichText::new(i18n::t(i18n::Key::AboutVersion, &self.lang))
                                .size(24.0)
                                .strong()
                                .color(theme::ACCENT),
                        );
                        ui.label(
                            egui::RichText::new(i18n::t(i18n::Key::AboutDescription, &self.lang))
                                .color(theme::TEXT_MUTED),
                        );
                    });

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        self.render_help_menu(ui);
                        self.render_file_menu(ui);
                        self.render_web_client_button(ui);
                        self.render_rpc_controls(ui);
                        self.render_server_controls(ui);
                    });
                });

                ui.add_space(14.0);

                ui.horizontal_wrapped(|ui| {
                    for tab in AppTab::ALL {
                        let response =
                            theme::pill_button(ui, self.tab_selected == tab, tab.label(&self.lang));
                        self.note_rect(&response);
                        if response.clicked() {
                            self.tab_selected = tab;
                        }
                    }
                });

                ui.add_space(12.0);

                ui.horizontal_wrapped(|ui| {
                    self.render_status_strip(ui);
                });
            });

        egui::CentralPanel::default()
            .frame(egui::Frame::default().fill(theme::APP_BG))
            .show_inside(ui, |ui| {
                egui::ScrollArea::vertical()
                    .id_salt("main_scroll")
                    .show(ui, |ui| {
                        match self.tab_selected {
                            AppTab::Server => server_panel::ui(
                                ui,
                                &mut self.settings,
                                &self.settings_manager,
                                &self.lang,
                            ),
                            AppTab::Rpc => rpc_panel::ui(
                                ui,
                                &mut self.settings,
                                &self.settings_manager,
                                &self.lang,
                            ),
                            AppTab::Model => model_panel::ui(
                                ui,
                                &mut self.settings,
                                &mut self.model_browser,
                                &self.lang,
                            ),
                            AppTab::Params => params_panel::ui(ui, &mut self.settings, &self.lang),
                            AppTab::Log => log_panel::ui(
                                ui,
                                &mut self.settings,
                                &mut self.server_manager,
                                &self.lang,
                            ),
                            AppTab::Commands => launch_commands_panel::ui(
                                ui,
                                &self.server_manager,
                                &self.rpc_manager,
                                &self.lang,
                            ),
                            AppTab::Presets => {
                                let should_start =
                                    presets_panel::ui(ui, &mut self.settings, &self.lang);
                                if should_start {
                                    self.server_manager.start(&self.settings);
                                }
                            }
                        }

                        if self.debug_mode {
                            self.spacing_debugger.visualize(ui);
                        }
                    });
            });
    }
}

impl Drop for LlamaLauncherApp {
    fn drop(&mut self) {
        self.server_manager.stop();
        self.rpc_manager.stop();
        self.save();
    }
}

fn server_state_color(state: ServerState) -> egui::Color32 {
    match state {
        ServerState::Running => theme::SUCCESS,
        ServerState::Starting => theme::INFO,
        ServerState::Stopping => theme::WARNING,
        ServerState::Error(_) => theme::DANGER,
        ServerState::Idle => theme::TEXT_MUTED,
    }
}

fn rpc_state_color(state: RpcState) -> egui::Color32 {
    match state {
        RpcState::Running => theme::SUCCESS,
        RpcState::Starting => theme::INFO,
        RpcState::Stopping => theme::WARNING,
        RpcState::Error(_) => theme::DANGER,
        RpcState::Idle => theme::TEXT_MUTED,
    }
}

#[cfg(target_os = "windows")]
fn enable_auto_start() {
    let exe_path = match std::env::current_exe() {
        Ok(path) => path,
        Err(error) => {
            log::error!("get current exe path failed: {}", error);
            return;
        }
    };

    let path_str = exe_path.to_string_lossy().to_string();
    match std::process::Command::new("reg")
        .arg("add")
        .arg(r#"HKEY_CURRENT_USER\Software\Microsoft\Windows\CurrentVersion\Run"#)
        .arg("/v")
        .arg("llama.cpp launcher")
        .arg("/d")
        .arg(format!("\"{}\" --minimized", path_str))
        .arg("/f")
        .output()
    {
        Ok(output) => {
            if !output.status.success() {
                let stderr = String::from_utf8_lossy(&output.stderr);
                log::error!("reg add failed: {}", stderr.trim());
            }
        }
        Err(error) => {
            log::error!("run reg command failed: {}", error);
        }
    }
}

#[cfg(target_os = "windows")]
fn disable_auto_start() {
    match std::process::Command::new("reg")
        .arg("delete")
        .arg(r#"HKEY_CURRENT_USER\Software\Microsoft\Windows\CurrentVersion\Run"#)
        .arg("/v")
        .arg("llama.cpp launcher")
        .arg("/f")
        .output()
    {
        Ok(output) => {
            if !output.status.success() {
                let stderr = String::from_utf8_lossy(&output.stderr);
                log::error!("reg delete failed: {}", stderr.trim());
            }
        }
        Err(error) => {
            log::error!("run reg command failed: {}", error);
        }
    }
}

#[cfg(not(target_os = "windows"))]
fn enable_auto_start() {
    use std::fs;

    let autostart_dir = dirs::config_dir()
        .map(|path| path.join("autostart"))
        .expect("unable to get XDG config directory");
    fs::create_dir_all(&autostart_dir).ok();

    let exe_path = match std::env::current_exe() {
        Ok(path) => path,
        Err(error) => {
            log::error!("get current exe path failed: {}", error);
            return;
        }
    };

    let desktop_content = format!(
        r#"[Desktop Entry]
Type=Application
Name=LLama Launcher
Exec={} --minimized
Hidden=false
NoDisplay=false
X-GNOME-Autostart-enabled=true
"#,
        exe_path.display()
    );

    let desktop_path = autostart_dir.join("llama-cpp-launcher.desktop");
    match fs::write(&desktop_path, &desktop_content) {
        Ok(_) => {}
        Err(error) => log::error!("create autostart file failed: {}", error),
    }
}

#[cfg(not(target_os = "windows"))]
fn disable_auto_start() {
    use std::fs;

    if let Some(path) =
        dirs::config_dir().map(|path| path.join("autostart/llama-cpp-launcher.desktop"))
    {
        if let Err(error) = fs::remove_file(&path) {
            log::error!("remove autostart file failed: {}", error);
        }
    }
}

#[cfg(target_os = "windows")]
mod shell_execute {
    use std::ffi::{c_void, OsStr};
    use std::os::windows::ffi::OsStrExt;

    #[link(name = "shell32", kind = "dylib")]
    extern "system" {
        fn ShellExecuteW(
            hind_window: *mut c_void,
            lp_operation: *const u16,
            lp_file: *const u16,
            lp_parameters: *const u16,
            lp_directory: *const u16,
            n_show_cmd: i32,
        ) -> isize;
    }

    const SW_SHOW_NORMAL: i32 = 1;

    pub(crate) fn open_url(url: &str) {
        let operation = OsStr::new("open")
            .encode_wide()
            .chain(Some(0))
            .collect::<Vec<u16>>();
        let file = OsStr::new(url)
            .encode_wide()
            .chain(Some(0))
            .collect::<Vec<u16>>();

        let _ = unsafe {
            ShellExecuteW(
                std::ptr::null_mut(),
                operation.as_ptr(),
                file.as_ptr(),
                std::ptr::null::<u16>(),
                std::ptr::null::<u16>(),
                SW_SHOW_NORMAL,
            )
        };
    }
}

#[cfg(target_os = "windows")]
fn open_web_client_url(port: u16) {
    shell_execute::open_url(&format!("http://127.0.0.1:{}", port));
}

#[cfg(not(target_os = "windows"))]
fn open_web_client_url(port: u16) {
    let _ = std::process::Command::new("xdg-open")
        .arg(format!("http://127.0.0.1:{}", port))
        .spawn();
}

#[cfg(target_os = "windows")]
fn open_repo_url() {
    shell_execute::open_url("https://github.com/yihuishou/llama.cpp-launcher");
}

#[cfg(not(target_os = "windows"))]
fn open_repo_url() {
    let _ = std::process::Command::new("xdg-open")
        .arg("https://github.com/yihuishou/llama.cpp-launcher")
        .spawn();
}
