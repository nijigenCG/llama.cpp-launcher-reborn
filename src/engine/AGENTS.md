# src/engine/ — 引擎目录

## 职责
llama-server 和 rpc-server 的进程管理、状态机、日志聚合。具体参数和 UI 交互见根 AGENTS.md / ui/AGENTS.md。

## 文件清单
- server.rs: ServerManager, ServerState；llama-server 生命周期 + launch_command 捕获
- rpc.rs: RpcManager, RpcState；rpc-server 生命周期
- mod.rs: LogEntry, LogType(Server/Rpc)；日志聚合与枚举定义

## ServerManager 状态机
```
Idle → Starting → Running → Idle          (正常停止)
Idle → Starting → Running → Error(_)      (崩溃/非零退出)
Running → Stopping → Idle                  (停止中)
```
- start(): Command(含所有 --args) → spawn → stdout/stderr BufReader → 捕获 launch_command
- stop(): child.kill() + child.wait().ok() → 清理状态
- poll_logs(): try_wait() 检测退出 → Error(exit_code)
- is_listening(): 检查是否已监听端口（WebClient 按钮启禁用依据）

## RpcManager 状态机
```
Idle → Starting → Running → Idle          (正常停止)
Running → Stopping → Idle                  (停止中)
```
- start(): --host/--port/--threads/--device + rpc_cache?--cache
- poll(): try_wait() 检测退出 → Error(exit_code)

## 进程管理通用模式
- Arc<Mutex<InnerState>> 包裹 std::process::Child
- stdout/stderr: 各一个 thread::spawn, BufReader→lines→push_back
- Windows: CREATE_NO_WINDOW (0x08000000), cfg(windows) 分支
- Drop trait 自动 stop()

## Log 聚合 (mod.rs)
- LogEntry { source: LogType, timestamp, message }
- LogType::Server / LogType::Rpc
- VecDeque<String>, 容量限制 2000 行

## 约束
- start(): path非空 + is_file()；已运行则直接返回（幂等）
- 错误消息走 i18n，禁止硬编码。
- stop() 时通过 child.take() 使日志线程自然退出。
