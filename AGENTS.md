# LLama Launcher — Root

## 项目概述
llama.cpp 桌面启动器。Rust + eframe/egui 0.29 GUI，管理 llama-server/RPC、预设系统、Windows 开机自启。

## 技术栈
- **Rust** 2021 版, egui 0.29, eframe 0.29
- **序列化**: serde + serde_json
- **i18n**: 自研 (zh, en)
- **文件对话框**: rfd
- **日志**: log + env_logger
- **快捷方式**: shortcuts-rs
- **窗口/图标**: winit, winres via build.rs

## 架构
```
main.rs → LlamaLauncherApp (app.rs) → 7面板 (ui/) + SettingsManager (config/) + Server/RpcManager (engine/)
```

## 启动流程
1. `main.rs`: 加载 CJK 字体, set_zoom_factor(1.5), 创建 eframe
2. `LlamaLauncherApp::new()`: sys_locale → zh/en; 读取 llama_cpp_launcher_settings.json
3. auto_start_preset_name 存在: 应用预设 → auto_start_server_on_first_frame=true
4. 每帧 update(): tab_selected (字符串匹配 i18n Tab* key) → 路由到面板
5. 退出: Drop trait → stop server/RPC + save settings

## 核心模块
| 模块 | 路径 | 职责 |
|------|------|------|
| ui | src/ui/ | 7个面板 (egui), 模型标签解析 |
| engine | src/engine/ | Server + RPC 进程管理, 状态机, 日志聚合 |
| config | src/config/settings.rs | AppSettings/Preset/GpuLayersMode + SpeculativeDecoding/PresencePenalty 配置，JSON, exe 自动检测 |
| i18n | src/i18n.rs | Key 枚举 + zh/en 映射 |
| shortcut | src/shortcut.rs | Windows 桌面快捷方式 (ShellLink) |

## 关键数据结构
- `LlamaLauncherApp`: tab_selected(String), show_about, auto_start_server_on_first_frame
- `AppSettings`: server/RPC/模型/推理/GPU + presets(Vec<Preset>) + auto_start_preset_name
- `Preset`: AppSettings 快照, from_settings()/apply_to() 双向转换
- `GpuLayersMode`: Auto / All(99) / Manual(n), 自定义 serde
- `ServerState` / `RpcState`: Idle, Starting, Running, Stopping, Error(_)

## LlamaLauncherApp (app.rs)
- 7个标签: Server, RPC, Model, Params, Log, Commands, Presets
- 顶部栏: File/Help 菜单 + 标签切换 + Server/RPC/WebClient 控制按钮
- server_panel::ui 接收 &SettingsManager (用于自动检测 exe)
- render_server_controls / render_rpc_controls / render_web_client_button — 状态感知启禁用

## Windows 特有功能
- enable/disable_auto_start(): reg add/delete HKEY_CURRENT_USER\...\Run
- create_desktop_shortcut(): shortcuts-rs → ShellLink (.lnk), locale-aware 名称
- --autostart-minimized CLI flag (开机自启时最小化)

## 约束
- **单 binary**: 无 Cargo workspace
- **Windows 优先**: CREATE_NO_WINDOW, .exe 检测, winres 图标
- **进程管理**: std::process::Child + Arc<Mutex<>> + Drop trait
- **日志**: BufReader → VecDeque<String>, 2000 行上限
- **i18n Key**: 所有 UI 文本通过 i18n::t(Key, lang), 禁止硬编码

## 构建
```bash
cargo build --release
# target/release/llama_cpp_launcher.exe
# exe 同级: llama_cpp_launcher_settings.json
```

## 代码风格
- 简体中文注释和 i18n Key 命名
- egui 0.29 API, Result<T, String> + map_err
