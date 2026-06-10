fn main() {
    // build.rs 运行在 host 上，不能用 #[cfg(windows)] 判断目标平台
    // 必须通过 CARGO_CFG_TARGET_OS 环境变量判断
    let target_os = std::env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();

    if target_os == "windows" {
        let mut res = winres::WindowsResource::new();
        res.set_icon("assets/llama.ico");
        res.set("ProductName", "llama.cpp launcher");
        res.set("FileDescription", "llama.cpp launcher - GUI Launcher");
        res.set("LegalCopyright", "Copyright 2025");
        res.set("InternalName", "llama.cpp launcher");
        res.set("OriginalFilename", "llama_cpp_launcher.exe");
        res.set_version_info(winres::VersionInfo::FILEVERSION, 0x0000000100000000u64);
        res.set_version_info(winres::VersionInfo::PRODUCTVERSION, 0x0000000100000000u64);
        res.set_version_info(winres::VersionInfo::FILEOS, 0x40004u64);
        res.set_version_info(winres::VersionInfo::FILETYPE, 0x2u64);
        res.compile().unwrap();
    }

    // Linux: 将图标文件复制到 exe 同级目录
    if target_os == "linux" {
        let out_dir = std::env::var("OUT_DIR").unwrap();
        let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
        let icon_src = std::path::Path::new(&manifest_dir).join("llama-cpp-launcher.png");
        let icon_dst = std::path::Path::new(&out_dir)
            .join("../../..") // 回溯到 target/release 或 target/debug
            .join("llama-cpp-launcher.png");

        if icon_src.exists() {
            let _ = std::fs::copy(&icon_src, &icon_dst);
        }
    }
}
