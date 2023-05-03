use std::process::Command;

fn main() {
    let output = Command::new("mpv").arg("--version").output();
    if let Ok(output) = output {
        let version_string = String::from_utf8_lossy(&output.stdout);
        let version = version_string
            .lines()
            .next()
            .unwrap_or("")
            .split_whitespace()
            .nth(1)
            .unwrap_or("");

        if version.starts_with("0.34") {
            println!("cargo:rustc-cfg=feature=\"mpv_0_34\"");
        } else if version.starts_with("0.35") {
            println!("cargo:rustc-cfg=feature=\"mpv_0_35\"");
        } else {
            // fallback to rodio
            println!("cargo:rustc-cfg=feature=\"rodio_audio\"");
        }
    } else {
        // fallback to rodio
        println!("cargo:rustc-cfg=feature=\"rodio_audio\"");
    }
}
