fn main() {
    let target_os = std::env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();
    let target_env = std::env::var("CARGO_CFG_TARGET_ENV").unwrap_or_default();

    if target_os != "windows" {
        return;
    }

    if target_env != "msvc" && target_env != "gnu" {
        return;
    }

    if let Err(err) = winres::WindowsResource::new().set_icon("icon.ico").compile() {
        eprintln!("winres skipped: {}", err);
    }
}
