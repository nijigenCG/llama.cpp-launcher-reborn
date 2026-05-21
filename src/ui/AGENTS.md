# src/ui/ — UI 面板目录

## 职责
7 个 egui 面板 + 模型文件标签解析。纯渲染，业务逻辑委托给 app/config/engine。

## 文件清单
- server_panel: llama-server 路径/端口/槽位、启停/重启、状态、RPC 模式开关
- rpc_panel: rpc-server 路径/端口/threads/device/cache、启停、状态
- model_panel: GGUF 目录浏览、列表、彩色标签解析、mmproj/DFlash 切换
- params_panel: n_ctx/n_predict/temperature/top_p/top_k/repeat_penalty/kv_offload/cache_type/GPU
- log_panel: ServerLog + RpcLog 实时聚合、清空按钮
- launch_commands_panel: server/RPC 最终启动命令只读展示
- presets_panel: 预设保存/应用/删除/自启动，返回 bool(是否应启动 Server)

## 渲染约定
- 路由由 app.rs 按 tab_selected(i18n key) 控制；本目录不直接管理标签切换。
- 面板函数签名统一：fn ui(&mut Ui, settings: &mut AppSettings, lang: &Language)
- 例外:
  - model_panel: +&mut ServerManager, &mut RpcManager；FileMode(Main/Mmproj/DFlash)；auto_detect_model_dir("models"/"model")；is_dflash_file()
  - log_panel: +&mut ServerManager, &mut RpcManager（日志源）
  - presets_panel: bool 返回值用于触发 auto_start_server_on_first_frame

## 标签解析规则 (model_panel::parse_tags)
文件名去 .gguf，按 - 分段：
- 数字+b/m/k → 紫色(参数量)；q* → 橙色(量化)；纯数字/点 → 灰色(版本号)
- instruct/chat/sft/rlhf/dpo/orpo/grpo → 绿色(训练方式)；其余 → 蓝色(模型名)

## 约束
- UI文本仅通过 i18n::t(Key, lang)，禁止硬编码。
- rfd: server/rpc面板 pick_file()，model面板 pick_folder()。
- egui 0.29 API。