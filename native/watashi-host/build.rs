/// Build script — embed Windows manifest + version info into the exe via windres.
fn main() {
    let target_os = std::env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();
    if target_os == "windows" {
        embed_windows_manifest();
    }
}

/// Use windres (mingw) to compile .rc → .res and link into the exe.
fn embed_windows_manifest() {
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let rc_path = format!("{manifest_dir}/watashi.rc");
    let res_path = format!("{out_dir}/watashi.res");

    // Try windres (mingw cross-compile toolchain)
    let windres = std::env::var("WINDRES")
        .unwrap_or_else(|_| "x86_64-w64-mingw32-windres".to_string());

    let status = std::process::Command::new(&windres)
        .args(["--input", &rc_path, "--output", &res_path, "--output-format=coff"])
        .current_dir(&manifest_dir)
        .status();

    match status {
        Ok(s) if s.success() => {
            println!("cargo:rustc-link-arg-bins={res_path}");
            println!("cargo:rerun-if-changed=watashi.rc");
            println!("cargo:rerun-if-changed=watashi.manifest");
        }
        _ => {
            eprintln!("warning: windres not available, skipping manifest embedding");
        }
    }
}
