# LLama Launcher — Root

## 项目概述
llama.cpp 桌面启动器。Rust + eframe/egui 0.29 GUI，管理 llama-server/RPC、预设系统、Windows 开机自启和快捷方式。单 binary，无 Cargo workspace。

## 技术栈
- Rust 2021, egui 0.29, eframe 0.29
- serde + serde_json (配置/预设)
- rfd (文件对话框), log + env_logger, shortcuts-rs
- Windows优先: winit, winres, CREATE_NO_WINDOW

## 架构总览
- main.rs → LlamaLauncherApp(app.rs) → UI面板(ui/) + 设置(config/) + Server/RPC管理(engine/)
- App启动: 加载字体 → 读配置JSON → 可选自动启动预设 → 每帧按 tab_selected 路由到对应面板。退出时 Drop trait清理子进程并保存配置。

## 核心模块与入口
| 目标 | 查看位置 |
|------|----------|
| UI面板/标签行为 | src/ui/AGENTS.md |
| Server/RPC生命周期、日志 | src/engine/AGENTS.md |
| AppSettings/Preset/GpuLayersMode | src/config/settings.rs |
| i18n键映射 (zh/en) | src/i18n.rs |
| Windows快捷方式 (.lnk) | src/shortcut.rs |
| 顶层App结构、菜单与标签路由 | src/app.rs |

## 启动流程 (高层)
1. main: 加载CJK字体, set_zoom_factor(1.5), 创建 eframe。
2. LlamaLauncherApp::new: 系统语言→zh/en; 读取 llama_cpp_launcher_settings.json。
3. auto_start_preset_name存在 → 应用预设 → auto_start_server_on_first_frame=true。
4. update(): tab_selected(i18n key) → 路由到面板。
5. Drop trait → stop server/RPC + save settings。

## Windows特有功能 (高层)
- 开机自启: HKEY_CURRENT_USER\...\Run (enable/disable_auto_start)。
- 桌面快捷方式: shortcuts-rs, locale-aware名称。
## 全局约束
- 单binary, Windows优先。
- 进程管理: std::process::Child + Arc<Mutex<>> + Drop自动停止。
- 日志: BufReader → VecDeque<String>, 2000行上限。
- i18n: 所有UI文本通过 i18n::t(Key, lang), 禁止硬编码。
- egui 0.29 API; Result<T, String> + map_err。

## 构建命令
- cargo build --release → target/release/llama_cpp_launcher.exe (同级 llama_cpp_launcher_settings.json)。
