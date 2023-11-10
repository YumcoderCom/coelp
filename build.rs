// build.rs

// To build and see the result, run: cargo build -vv
// To force a rebuild, remove the "target" folder.

use std::{env, fs, os::unix::prelude::PermissionsExt, path::PathBuf, process::Command};

fn main() {
    // Check the target operating system
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();

    // Define the shell script to run
    let shell_script_file_name = match target_os.as_str() {
        "linux" => "linux.sh",
        "windows" => "windows.bat",
        "macos" => match env::var("CARGO_CFG_TARGET_ARCH").ok().as_deref() {
            Some("x86_64") => "darwin_amd64.sh",
            Some("aarch64") => "darwin_arm64.sh",
            Some(arch) => {
                eprintln!("Building for an unsupported architecture: {}", arch);
                return;
            }
            None => {
                eprintln!("Unable to determine the target architecture.");
                return;
            }
        },
        _ => {
            eprintln!("Unsupported OS: {}", target_os);
            return;
        }
    };

    // Get the directory where the build.rs script is located
    let current_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let working_directory = PathBuf::from(current_dir.clone()).join("src/llm/llama.cpp");

    let shell_script_file_path = working_directory
        .join("build_script")
        .join(shell_script_file_name);

    // All files are executable on Windows, so just set permissions on Unix
    if !has_execute_permission(&shell_script_file_path) {
        if let Err(err) =
            fs::set_permissions(&shell_script_file_path, fs::Permissions::from_mode(0o755))
        {
            eprintln!(
                "Failed to set execute permissions for '{}': {}",
                shell_script_file_path.display(),
                err
            );
            return;
        }
    }

    // Run the build_shell_script file
    Command::new(shell_script_file_path)
        .current_dir(working_directory.clone()) // Set the working directory
        .status()
        .unwrap();

    println!(
        "cargo:rerun-if-changed={}",
        working_directory.to_string_lossy()
    );
}

#[cfg(windows)]
pub fn has_execute_permission(file_path: &PathBuf) -> bool {
    true
}

#[cfg(unix)]
fn has_execute_permission(file_path: &PathBuf) -> bool {
    fs::metadata(file_path)
        .map(|metadata| metadata.permissions().readonly())
        .unwrap_or(false)
}
