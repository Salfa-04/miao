//!
//!  To Download with  OpenOCD:
//! ```
//! $ cargo br && openocd
//! ```
//!

use std::env::var;
use std::fs::write;
use std::path::PathBuf;

fn main() -> std::io::Result<()> {
    cargo_emit::rerun_if_changed!("build.rs");
    cargo_emit::rerun_if_changed!("build.map");

    let package = env!("CARGO_PKG_NAME");

    // Output the build.map file   : This is useful for analysis.
    cargo_emit::rustc_link_arg!(format!("-Map={}/build.map", package));

    write_openocd_config_file(package)?;

    Ok(())
}

///
/// # Writes the OpenOCD flash configuration file.
///
fn write_openocd_config_file(name: &str) -> std::io::Result<()> {
    let workdir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let out_dir = PathBuf::from(var("OUT_DIR").unwrap());

    let executable = out_dir
        .ancestors()
        .find(|p| p.ends_with("build"))
        .map(|p| p.parent())
        .flatten()
        .map(|p| p.join(name))
        .unwrap_or_default();

    write(
        workdir.join("openocd.cfg"),
        format!(
            "source [find ../openocd.cfg]\nprogram {} preverify verify reset exit",
            executable.display()
        ),
    )?;

    Ok(())
}
