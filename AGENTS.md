# LLama Launcher — Root

## 项目概述
llama.cpp 桌面启动器。Rust + eframe/egui 0.34 GUI，管理 llama-server/RPC、预设系统、Windows 开机自启和快捷方式。单 binary，无 Cargo workspace。

## 技术栈
- Rust 2021, egui 0.34, eframe 0.34
- serde + serde_json (配置/预设)
- rfd (文件对话框), log + env_logger, shortcuts-rs
- Windows优先: winit, winres, CREATE_NO_WINDOW

## STRUCTURE
```
root/
├── Cargo.toml              # 单 binary, llama_cpp_launcher.exe
├── build.rs                # Windows icon / manifest (winres)
├── src/main.rs             # 入口: eframe + CJK字体 + env_logger
├── src/app.rs              # LlamaLauncherApp: UI路由/菜单/状态/开机自启
├── src/config/             # AppSettings/Preset/GpuLayersMode/默认值与读写
├── src/engine/             # llama-server / rpc-server 进程管理与日志聚合
├── src/ui/                 # 7 个 egui 面板 (server/rpc/model/params/log/presets/cmds)
└── src/i18n.rs             # i18n::t(Key, lang)，zh/en key→文案映射
```

## WHERE TO LOOK
| Task | Location | Notes |
|------|----------|-------|
| UI面板/标签行为 | src/ui/AGENTS.md | 路由由 app.rs 控制，面板仅渲染 |
| Server/RPC生命周期、日志 | src/engine/AGENTS.md | 状态机 + 进程管理 + log聚合 |
| AppSettings/Preset/GpuLayersMode | src/config/settings.rs | 配置结构体 + Defaults + 读写 |
| i18n键映射 (zh/en) | src/i18n.rs | Key enum + t() 函数，所有 UI 文本入口 |
| Windows快捷方式 (.lnk) | src/shortcut.rs | shortcuts-rs 封装 |
| App结构、菜单与标签路由 | src/app.rs | LlamaLauncherApp::ui, tab_selected |

## CODE MAP (核心符号)
| Symbol | Type | Location | Role |
|--------|------|----------|------|
| main | fn | main.rs | eframe 入口，加载字体 & env_logger |
| load_cjk_fonts | fn | main.rs | CJK字体加载（fallback） |
| LlamaLauncherApp | struct+impl | app.rs | App根：settings/server/rpc/tab/lang/auto_start |
| impl eframe::App for LlamaLauncherApp::ui | method | app.rs | 每帧 UI 路由 + 菜单栏 |
| enable_auto_start / disable_auto_start | fn | app.rs | Windows 注册表开机自启 |
| open_web_client_url, open_repo_url | fn | app.rs | ShellExecuteW 打开浏览器 |
| AppSettings | struct | config/settings.rs | 全部配置字段（server/RPC/预设/GPU） |
| Preset | struct | config/settings.rs | 可保存/应用/删除的预设；含 apply_to() |
| GpuLayersMode | enum | config/settings.rs | Auto/All/Manual，序列化策略 |
| SettingsManager | impl | config/settings.rs | load/save + auto_detect_server_path/rpc |
| ServerManager | struct+impl | engine/server.rs | llama-server 生命周期、日志、launch_command |
| RpcManager | struct+impl | engine/rpc.rs | rpc-server 生命周期、连接状态 |

## 启动流程 (高层)
1. main: 加载CJK字体, set_zoom_factor(1.5), 创建 eframe。
2. LlamaLauncherApp::new: 系统语言→zh/en; 读取 llama_cpp_launcher_settings.json。
3. auto_start_preset_name存在 → 应用预设 → auto_start_server_on_first_frame=true。
4. update(): tab_selected(i18n key) → 路由到对应面板。
5. Drop trait → stop server/RPC + save settings。

## Windows特有功能 (高层)
- 开机自启: HKEY_CURRENT_USER\...\Run (enable/disable_auto_start)。
- 桌面快捷方式: shortcuts-rs, locale-aware名称。

## 全局约束
- 单binary, Windows优先。
- 进程管理: std::process::Child + Arc<Mutex<>> + Drop自动停止。
- 日志: BufReader → VecDeque<String>, 2000行上限。
- i18n: 所有UI文本通过 i18n::t(Key, lang), 禁止硬编码中文/英文到 UI 代码中。
- egui 0.34 API; Result<T, String> + map_err。

## ANTI-PATTERNS (THIS PROJECT)
- 在 UI 面板中直接启动/停止进程（必须走 engine）。
- 绕过 i18n::t() 硬编码中文/英文到 UI 代码。
- main.rs / app.rs 中写死启动参数而不经过 AppSettings/Preset。

## COMMANDS
- cargo build --release → target/release/llama_cpp_launcher.exe（同级 llama_cpp_launcher_settings.json）。
