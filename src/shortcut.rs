use std::path::PathBuf;

/// 在用户桌面创建指向 llama-lunch.exe 的 .lnk 快捷方式
#[cfg(target_os = "windows")]
pub fn create_desktop_shortcut() -> Result<(), String> {
    // 1. 获取 exe 路径
    let exe_path = std::env::current_exe().map_err(|e| format!("获取 exe 路径失败：{}", e))?;
    
    // 2. 获取桌面路径
    let desktop_dir = dirs::desktop_dir().ok_or_else(|| "无法获取桌面路径".to_string())?;
    
    // 3. 构造快捷方式目标路径
    let shortcut_path: PathBuf = desktop_dir.join("llama.cpp lunch.lnk");
    
    // 4. 创建 ShellLink 快捷方式（自动设置工作目录为 exe 所在目录）
    let link = shortcuts_rs::ShellLink::new(&exe_path, None, None, None)
        .map_err(|e| format!("创建快捷方式对象失败：{}", e))?;
    link.create_lnk(&shortcut_path).map_err(|e| format!("创建快捷方式失败：{}", e))?;
    
    Ok(())
}